@group(0) @binding(0) var output_image: texture_storage_2d<rgba8unorm, write>;

struct Data {
    width: f32,
    height: f32,
    scale_factor: f32,
    time_elapsed: f32,
};

@group(0) @binding(1) var<uniform> data: Data;

@group(0) @binding(2) var<uniform> objects_len: u32;
@group(0) @binding(3) var<storage, read> objects: array<Object>;

struct Object {
    object_id: u32,
    _1: f32,
    _2: f32,
    _3: f32,
    _4: f32,
    _5: f32,
    _6: f32,
    _7: f32,
}

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

fn hash(seed: u32) -> u32 {
    var result = seed;
    result = (result ^ 61u) ^ (result >> 16u);
    result = result + (result << 3u);
    result = result ^ (result >> 4u);
    result = result * 0x27d4eb2du;
    result = result ^ (result >> 15u);
    return result;
}

fn random(seed: u32) -> f32 {
    let hashed = hash(seed);
    return f32(hashed & 0x00FFFFFFu) / f32(0x01000000u);
}

fn writePixel(color: vec4<f32>, global_id: vec3<u32>) {
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

    // inside the loop...
    let pixel_screen_pos = vec2<f32>(global_id.xy) * data.scale_factor;
    let pixel_center = pixel_00_loc + (pixel_screen_pos.x * pixel_delta_u) + (pixel_screen_pos.y * pixel_delta_v);
    let ray_direction = pixel_center - camera_center;

    let ray = Ray(camera_center, ray_direction);
    let color = ray_color(ray);

    writePixel(color, global_id);
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

fn hit_sphere(center: vec3<f32>, radius: f32, ray: Ray) -> f32 {
    let oc = center - ray.origin;
    let a = vec_length_squared(ray.direction);
    let h = dot(ray.direction, oc);
    let c = vec_length_squared(oc) - radius * radius;
    let discriminant = h * h - a * c;

    var ret = 0.0;
    if discriminant < 0.0 {
        ret = -1.0;
    } else {
        ret = (h - sqrt(discriminant)) / a;
    }
    return ret;
}

fn ray_color(ray: Ray) -> vec4<f32> {
    var color = vec4<f32>(0.0);

    let t = hit_sphere(vec3<f32>(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = unit_vector(ray_at(ray, t) - vec3<f32>(0.0, 0.0, -1.0));
        color = vec4<f32>(0.5 * vec3<f32>(n.x + 1, n.y + 1, n.z + 1), 1.0);
    } else {
        let unit_direction = unit_vector(ray.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        color = (1.0 - a) * vec4<f32>(1.0, 1.0, 1.0, 1.0) + a * vec4<f32>(0.5, 0.7, 1.0, 1.0);
    }

    return color;
}
