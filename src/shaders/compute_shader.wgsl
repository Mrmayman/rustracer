// import(uniforms.wgsl)
// import(rng.wgsl)
// import(interval.wgsl)

const pi: f32 = 3.1415926535897932385;
const infinity: f32 = pow(2.0, 127.0);

const samples = 20;

fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * pi / 180.0;
}

struct Object {
    object_id: u32,
    _1: f32, // Sphere.x
    _2: f32, // Sphere.y
    _3: f32, // Sphere.z
    _4: f32, // Sphere.radius
    _5: f32,
    _6: f32,
    _7: f32,
}

const id_sphere: u32 = 0;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct HitRecord {
    point: vec3<f32>,
    t: f32,
    normal: vec3<f32>,
    front_face: bool,
}

fn hit_record_set_face_normal(rec: ptr<function, HitRecord>, ray: Ray, outward_normal: vec3<f32>) {
    (*rec).front_face = dot(ray.direction, outward_normal) < 0;
    if (*rec).front_face {
        (*rec).normal = outward_normal;
    } else {
        (*rec).normal = -outward_normal;
    }
}

fn write_pixel(color: vec4<f32>, global_id: vec3<u32>) {
    let uv = vec2<f32>(global_id.xy) / (vec2<f32>(data.width, data.height) / data.scale_factor);
    textureStore(output_image, vec2<i32>(global_id.xy), color);
}

@compute @workgroup_size(1, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
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

    var rng: u32 = rng_init(global_id.xy, vec2<u32>(u32(data.width), u32(data.height)), u32(data.time_elapsed * 100));

    var color = vec3<f32>(0.0);
    // inside the loop...
    for (var sample_i = 0; sample_i < samples; sample_i += 1) {
        let ray = get_ray(global_id, &rng, pixel_00_loc, pixel_delta_u, pixel_delta_v, camera_center);
        color += ray_color(ray);
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
    // let offset = vec3<f32>(rng_float(rng) - 0.5, rng_float(rng) - 0.5, 0);

    let pixel_screen_pos = vec2<f32>(global_id.xy) * data.scale_factor;
    // let pixel_center = pixel_00_loc + ((pixel_screen_pos.x + offset.x) * pixel_delta_u) + ((pixel_screen_pos.y + offset.y) * pixel_delta_v);
    let pixel_center = pixel_00_loc + (pixel_screen_pos.x * pixel_delta_u) + (pixel_screen_pos.y * pixel_delta_v);
    let ray_direction = pixel_center - camera_center;

    return Ray(camera_center, ray_direction);
}

fn vec_length_squared(vector: vec3<f32>) -> f32 {
    return (vector.x * vector.x) + (vector.y * vector.y) + (vector.z * vector.z);
}

fn vec_length(vector: vec3<f32>) -> f32 {
    return sqrt(vec_length_squared(vector));
}

fn unit_vector(vector: vec3<f32>) -> vec3<f32> {
    return vector * inverseSqrt(vec_length_squared(vector));
}

fn ray_at(ray: Ray, val: f32) -> vec3<f32> {
    return ray.origin + (val * ray.direction);
}

fn sphere_hit(center: vec3<f32>, radius: f32, ray: Ray, ray_t: Interval, hit_record: ptr<function, HitRecord>) -> bool {
    let oc = center - ray.origin;
    let a = vec_length_squared(ray.direction);
    let h = dot(ray.direction, oc);
    let c = vec_length_squared(oc) - radius * radius;
    let discriminant = h * h - a * c;

    var ret = true;
    if discriminant < 0.0 {
        ret = false;
    } else {
        let sqrtd = sqrt(discriminant);
        var root = (h - sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t.min || ray_t.max <= root {
                ret = false;
            }
        }
        if ret {
            (*hit_record).t = root;
            (*hit_record).point = ray_at(ray, root);

            let outward_normal = ((*hit_record).point - center) / radius;
            hit_record_set_face_normal(hit_record, ray, outward_normal);
        }
    }
    return ret;
}

fn world_hit(ray: Ray, ray_t: Interval, hit_record: ptr<function, HitRecord>) -> bool {
    var hit_anything = false;
    var temp_rec = HitRecord(vec3<f32>(0.0), 0.0, vec3<f32>(0.0), false);
    var closest_so_far = ray_t.max;

    for (var i = 0u; i < objects_len; i += 1u) {
        let object = objects[i];
        if object.object_id == id_sphere {
            let circle_pos = vec3<f32>(object._1, object._2, object._3);
            let circle_radius = object._4;
            if sphere_hit(circle_pos, circle_radius, ray, Interval(ray_t.min, closest_so_far), &temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                (*hit_record) = temp_rec;
            }
        }
    }

    return hit_anything;
}

fn ray_color(ray: Ray) -> vec3<f32> {
    var hit_record = HitRecord(vec3<f32>(0.0), 0.0, vec3<f32>(0.0), false);
    if world_hit(ray, Interval(0.0, infinity), &hit_record) {
        return 0.5 * (hit_record.normal + vec3<f32>(1.0));
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        return mix(vec3<f32>(1.0, 1.0, 1.0), vec3<f32>(0.5, 0.7, 1.0), t);
    }
}

