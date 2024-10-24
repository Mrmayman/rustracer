// The following imports are automatically added during shader loading stage

// import(uniforms.wgsl)
// import(rng.wgsl)
// import(interval.wgsl)
// import(vec.wgsl)

// Tweak these settings as per your needs:
// =======================================
// Samples: Less is faster. Higher samples reduce noise.
// Each pixel is averaged across multiple rays to smooth out lighting.
// Recommended: 32 for faster rendering, 64+ for quality.
const samples = 16;
// Bounces: Controls the number of light bounces (depth) for reflection and refraction.
// More bounces improve accuracy but slow down performance.
// Recommended: 2-4 for basic reflections, higher for complex scenes.
const bounces = 6;
// Antialiasing: Improves the quality of object edges by smoothing jagged pixels.
// 1 for On, and 0 for Off.
// Note: Motion blur On looks better with antialiasing Off.
const antialiasing = 0;
// Motion Blur: Adds motion blur based on camera or object movement, reducing noise.
// Cuts noise in half when set to 1.0, so:
//   samples=64, motion_blur=0.0 == samples=32, motion_blur=1.0
// This improves performance, but motion blur can cause eye strain.
// 1 for On, and 0 for Off.
const motion_blur = 1.0;
// =======================================

const infinity: f32 = pow(2.0, 127.0);
const bounce_cutoff = 0.1;

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * pi / 180.0;
}

struct Object {
    material: u32,
    object_id: u32,
    _1: f32, // Sphere.x
    _2: f32, // Sphere.y
    _3: f32, // Sphere.z
    _4: f32, // Sphere.radius
    _5: f32,
    _6: f32,
    _7: f32,
    _8: f32,
    _9: f32,
    _10: f32,
    _11: f32,
    _12: f32,
    _13: f32,
    _14: f32,
}

struct Material {
    material_id: u32,
    _1: f32, // Lambertian.albedo.r
    _2: f32, // Lambertian.albedo.g
    _3: f32, // Lambertian.albedo.b
    _4: f32, // Lambertian.albedo.a
    _5: f32,
    _6: f32,
    _7: f32,
    // albedo: [f32; 4],
    // _padding: [f32; 8 - 5],
}

const mat_id_lambertian: u32 = 0;
const mat_id_metal: u32 = 1;
const mat_id_dielectric: u32 = 2;

fn material_scatter(material: Material, r_in: Ray, rec: ptr<function, HitRecord>, attenuation: ptr<function, vec3<f32>>, scattered: ptr<function, Ray>, rng_state: ptr<function, u32>) -> bool {
    let hrec = (*rec);
    let normal = hrec.normal;
    let point = hrec.point;
    if material.material_id == mat_id_lambertian {
        var scatter_direction = normal + rng_vec_unit_vector(rng_state);
        if vec_near_zero(scatter_direction) {
            scatter_direction = normal;
        }

        (*scattered) = Ray(point, scatter_direction);
        (*attenuation) *= vec3<f32>(material._1, material._2, material._3);
        return true;
    } else if material.material_id == mat_id_metal {
        var reflected = vec_reflected(r_in.direction, normal);
        reflected = vec_unit_vector(reflected) + (material._5 * rng_vec_unit_vector(rng_state));
        (*scattered) = Ray(point, reflected);
        (*attenuation) *= vec3<f32>(material._1, material._2, material._3);
        return (dot(reflected, normal) > 0);
    } else if material.material_id == mat_id_dielectric {
        var ri = 0.0;
        if hrec.front_face {
            ri = 1.0 / material._1;
        } else {
            ri = material._1;
        }

        let unit_direction = vec_unit_vector(r_in.direction);

        let cos_theta = min(dot(-unit_direction, normal), 1.0);
        let sin_theta = sqrt(1.0 - (cos_theta * cos_theta));

        let cannot_refract = ri * sin_theta > 1.0;
        var direction = vec3<f32>(0.0);

        if cannot_refract || dielectric_reflectance(cos_theta, ri) > rng_float(rng_state) {
            direction = vec_reflected(unit_direction, normal);
        } else {
            direction = vec_refract(unit_direction, normal, ri);
        }

        (*scattered) = Ray(point, direction);
        return true;
    } else {
        return false;
    }
}

fn dielectric_reflectance(cosine: f32, refraction_index: f32) -> f32 {
    var r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}

const obj_id_sphere: u32 = 0;
const obj_id_triangle: u32 = 1;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct HitRecord {
    point: vec3<f32>,
    t: f32,
    normal: vec3<f32>,
    front_face: bool,
    material: Material,
}

fn hit_record_set_face_normal(rec: HitRecord, ray: Ray, outward_normal: vec3<f32>) -> HitRecord {
    let dot_normal = dot(ray.direction, outward_normal);
    var hrec = rec;
    hrec.front_face = dot_normal < 0;
    hrec.normal = (abs(dot_normal) / -dot_normal) * outward_normal;
    return hrec;
}

fn hit_record_new() -> HitRecord {
    return HitRecord(vec3<f32>(0.0), 0.0, vec3<f32>(0.0), false, Material(0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
}

fn write_pixel(color: vec4<f32>, global_id: vec3<u32>) {
    let scale = u32(data.scale_factor);
    let old_color = textureLoad(texture, vec2<i32>(global_id.xy), 0);
    textureStore(output_image, vec2<i32>(global_id.xy), (color + (old_color * motion_blur)) / (1.0 + motion_blur));
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // write_pixel(vec4<f32>(0.0, 0.0, f32(data.frame_number) / 50.0, 1.0), global_id);
    let aspect_ratio = data.width / data.height;

    let camera_center = vec3<f32>(data.camx, data.camy, data.camz);
    let lookat = vec3<f32>(data.lookx, data.looky, data.lookz);
    let focal_length = vec_length(camera_center - lookat);
    let theta = degrees_to_radians(90.0);
    let h = tan(theta / 2.0);
    let viewport_height = 2.0 * h * focal_length;

    let vup = vec3<f32>(0.0, 1.0, 0.0);

    let w = vec_unit_vector(camera_center - lookat);
    let u = vec_unit_vector(cross(vup, w));
    let v = cross(w, u);

    let viewport_u = viewport_height * aspect_ratio * u;
    let viewport_v = viewport_height * -v;

    let pixel_delta_u = viewport_u / data.width;
    let pixel_delta_v = viewport_v / data.height;

    // let viewport_upper_left = camera_center - vec3<f32>(0.0, 0.0, focal_length) - 0.5 * (viewport_u + viewport_v);
    // let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // May god forgive me for this abomination...
    let pixel_00_loc = camera_center - (focal_length * w) - 0.5 * ((viewport_u + viewport_v) - (pixel_delta_u + pixel_delta_v));

    var rng: u32 = rng_init(global_id.xy, vec2<u32>(u32(data.width), u32(data.height)), data.frame_number);

    var color = vec3<f32>(0.0);
    // inside the loop...
    for (var sample_i = 0; sample_i < samples; sample_i += 1) {
        let ray = get_ray(global_id, &rng, pixel_00_loc, pixel_delta_u, pixel_delta_v, camera_center);
        color += ray_color(ray, &rng);
    }

    write_pixel(vec4<f32>(color / f32(samples), 1.0), global_id);
}

fn get_ray(
    global_id: vec3<u32>,
    rng: ptr<function, u32>,
    pixel_00_loc: vec3<f32>,
    pixel_delta_u: vec3<f32>,
    pixel_delta_v: vec3<f32>,
    camera_center: vec3<f32>,
) -> Ray {
    let pixel_screen_pos = vec2<f32>(global_id.xy) * data.scale_factor;

    var pixel_center = vec3<f32>(0.0);
    if antialiasing == 1 {
        let offset = vec3<f32>(rng_float(rng) - 0.5, rng_float(rng) - 0.5, 0);
        pixel_center = pixel_00_loc + ((pixel_screen_pos.x + offset.x) * pixel_delta_u) + ((pixel_screen_pos.y + offset.y) * pixel_delta_v);
    } else {
        pixel_center = pixel_00_loc + (pixel_screen_pos.x * pixel_delta_u) + (pixel_screen_pos.y * pixel_delta_v);
    }

    let ray_direction = pixel_center - camera_center;

    return Ray(camera_center, ray_direction);
}

fn ray_at(ray: Ray, val: f32) -> vec3<f32> {
    return ray.origin + (val * ray.direction);
}

fn sphere_hit(material: u32, center: vec3<f32>, radius: f32, ray: Ray, ray_t: Interval, hit_record: ptr<function, HitRecord>) -> bool {
    let oc = center - ray.origin;
    let a = vec_length_squared(ray.direction);
    let h = dot(ray.direction, oc);
    let c = vec_length_squared(oc) - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return false;
    } else {
        let sqrtd = sqrt(discriminant);
        var root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                return false;
            }
        }
        var hrec = (*hit_record);
        hrec.t = root;
        hrec.point = ray_at(ray, root);
        hrec.material = materials[material];

        let outward_normal = (hrec.point - center) / radius;
        (*hit_record) = hit_record_set_face_normal(hrec, ray, outward_normal);
    }
    return true;
}

fn triangle_hit(material: u32, vert0: vec3<f32>, vert1: vec3<f32>, vert2: vec3<f32>, ray: Ray, ray_t: Interval, hit_record: ptr<function, HitRecord>) -> bool {
    let tu_vec = vert1 - vert0;  // Vector from vertex 0 to vertex 1 (triangle edge)
    let tv_vec = vert2 - vert0;  // Vector from vertex 0 to vertex 2 (triangle edge)
    let p_vec = cross(ray.direction, tv_vec); // Cross product of ray direction and tv_vec
    let det = dot(tu_vec, p_vec); // Determinant to check if ray is parallel to triangle

    if abs(det) < 0.0001 { // If determinant is too small, ray is parallel to triangle plane
        return false;
    }

    let inv_det = 1.0 / det;
    let t_vec = ray.origin - vert0;
    let u = dot(t_vec, p_vec) * inv_det;

    if u < 0.0 || u > 1.0 { // If u is outside of valid range, no hit
        return false;
    }

    let q_vec = cross(t_vec, tu_vec); // Cross product of t_vec and tu_vec
    let v = dot(ray.direction, q_vec) * inv_det;

    if v < 0.0 || u + v > 1.0 { // If v is outside of valid range, or u+v > 1 (outside triangle)
        return false;
    }

    let t = dot(tv_vec, q_vec) * inv_det;

    if t < ray_t.min || t > ray_t.max { // Check if t is within the valid range
        return false;
    }

    // Update hit record
    let hrec = (*hit_record);
    hrec.t = t;
    hrec.point = ray_at(ray, t);
    hrec.material = materials[material];
    let outward_normal = normalize(cross(tu_vec, tv_vec)); // Normal of the triangle plane

    (*hit_record) = hit_record_set_face_normal(hrec, ray, outward_normal);

    return true; // Intersection found
}

fn world_hit(ray: Ray, ray_t: Interval, hit_record: ptr<function, HitRecord>) -> bool {
    var hit_anything = false;
    var temp_rec = hit_record_new();
    var closest_so_far = ray_t.max;

    for (var i = 0u; i < objects_len; i += 1u) {
        let object = objects[i];
        if object.object_id == obj_id_sphere {
            let circle_pos = vec3<f32>(object._1, object._2, object._3);
            let circle_radius = object._4;
            let material = object.material;
            if sphere_hit(material, circle_pos, circle_radius, ray, Interval(ray_t.min, closest_so_far), &temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                (*hit_record) = temp_rec;
            }
        }
    }

    return hit_anything;
}

fn ray_color(primary_ray: Ray, state: ptr<function, u32>) -> vec3<f32> {
    var color = vec3<f32>(0.0, 0.0, 0.0);
    var ray = primary_ray;
    var attenuation = vec3<f32>(1.0);

    for (var bounce = 0; bounce < bounces; bounce += 1) {
        var hit_record = hit_record_new();
        if world_hit(ray, Interval(0.001, infinity), &hit_record) {
            var scattered = Ray(vec3<f32>(0.0), vec3<f32>(0.0));
            material_scatter(hit_record.material, ray, &hit_record, &attenuation, &scattered, state);
            ray = scattered;
            if attenuation.x < bounce_cutoff && attenuation.y < bounce_cutoff && attenuation.z < bounce_cutoff {
                break;
            }
        } else {
            let unit_direction = vec_unit_vector(ray.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            let sky_color = mix(vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(0.5, 0.7, 1.0), t);
            color = (sky_color * attenuation);
            break;
        }
    }

    return color;
}

