@group(0) @binding(0) var output_image: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> data: Data;

@group(0) @binding(2) var texture: texture_2d<f32>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let sigma = 1.5;
    let ksigma = sigma * 2.0;

    let uv = (vec2<f32>(global_id.xy) * data.scale_factor) / vec2<f32>(data.width, data.height);
    let real_uv = vec2<u32>(uv * vec2<f32>(data.width, data.height) / data.scale_factor);
    let color = smartDeNoise(vec2<i32>(global_id.xy), sigma, ksigma, 0.195);
    // let color = textureLoad(texture, vec2<i32>(global_id.xy), 0);

    textureStore(output_image, vec2<i32>(global_id.xy), color);
}