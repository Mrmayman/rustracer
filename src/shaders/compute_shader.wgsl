// import(uniforms.wgsl)
// import(rng.wgsl)
// import(interval.wgsl)

const infinity: f32 = pow(2.0, 127.0);

const samples = 32;
const bounces = 4;

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
    if material.material_id == mat_id_lambertian {
        var scatter_direction = (*rec).normal + rng_vec_unit_vector(rng_state);

        if vec_near_zero(scatter_direction) {
            scatter_direction = (*rec).normal;
        }

        (*scattered) = Ray((*rec).point, scatter_direction);
        (*attenuation) *= vec3<f32>(material._1, material._2, material._3);
        return true;
    } else if material.material_id == mat_id_metal {
        var reflected = vec_reflected(r_in.direction, (*rec).normal);
        reflected = vec_unit_vector(reflected) + (material._5 * rng_vec_unit_vector(rng_state));
        (*scattered) = Ray((*rec).point, reflected);
        (*attenuation) *= vec3<f32>(material._1, material._2, material._3);
        return (dot((*scattered).direction, (*rec).normal) > 0);
    } else if material.material_id == mat_id_dielectric {
        var ri = 0.0;
        if (*rec).front_face {
            ri = 1.0 / material._1;
        } else {
            ri = material._1;
        }

        let unit_direction = vec_unit_vector(r_in.direction);

        let cos_theta = min(dot(-unit_direction, (*rec).normal), 1.0);
        let sin_theta = sqrt(1.0 - (cos_theta * cos_theta));

        let cannot_refract = ri * sin_theta > 1.0;
        var direction = vec3<f32>(0.0);

        if cannot_refract || dielectric_reflectance(cos_theta, ri) > rng_float(rng_state) {
            direction = vec_reflected(unit_direction, (*rec).normal);
        } else {
            direction = vec_refract(unit_direction, (*rec).normal, ri);
        }

        (*scattered) = Ray((*rec).point, direction);
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

fn hit_record_set_face_normal(rec: ptr<function, HitRecord>, ray: Ray, outward_normal: vec3<f32>) {
    let dot_normal = dot(ray.direction, outward_normal);
    (*rec).front_face = dot_normal < 0;
    (*rec).normal = (abs(dot_normal) / -dot_normal) * outward_normal;
}

fn hit_record_new() -> HitRecord {
    return HitRecord(vec3<f32>(0.0), 0.0, vec3<f32>(0.0), false, Material(0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0));
}

fn write_pixel(color: vec4<f32>, global_id: vec3<u32>) {
    let uv = vec2<f32>(global_id.xy) / (vec2<f32>(data.width, data.height) / data.scale_factor);
    textureStore(output_image, vec2<i32>(global_id.xy), color);
}

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // write_pixel(vec4<f32>(0.0, 0.0, f32(data.frame_number) / 50.0, 1.0), global_id);
    let aspect_ratio = data.width / data.height;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;

    let focal_length = 1.0;
    let camera_center = vec3<f32>(0.0, 0.0, 0.0);

    let viewport_u = vec3<f32>(viewport_width, 0.0, 0.0);
    let viewport_v = vec3<f32>(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / data.width;
    let pixel_delta_v = viewport_v / data.height;

    let viewport_upper_left = camera_center - vec3<f32>(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
    let offset = vec3<f32>(rng_float(rng) - 0.5, rng_float(rng) - 0.5, 0);

    let pixel_screen_pos = vec2<f32>(global_id.xy) * data.scale_factor;
    let pixel_center = pixel_00_loc + ((pixel_screen_pos.x + offset.x) * pixel_delta_u) + ((pixel_screen_pos.y + offset.y) * pixel_delta_v);
    // let pixel_center = pixel_00_loc + (pixel_screen_pos.x * pixel_delta_u) + (pixel_screen_pos.y * pixel_delta_v);
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
        (*hit_record).t = root;
        (*hit_record).point = ray_at(ray, root);
        (*hit_record).material = materials[material];

        let outward_normal = ((*hit_record).point - center) / radius;
        hit_record_set_face_normal(hit_record, ray, outward_normal);
    }
    return true;
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

