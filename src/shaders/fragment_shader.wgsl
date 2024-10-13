@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct Data {
    width: f32,
    height: f32,
    scale_factor: f32,
    time_elapsed: f32,
    frame_number: u32,
    camx: f32,
    camy: f32,
    camz: f32,
    lookx: f32,
    looky: f32,
    lookz: f32,
};

@group(0) @binding(2) var<uniform> data: Data;
@group(0) @binding(3) var previous_texture: texture_storage_2d<rgba8unorm, write>;

@fragment
fn main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(data.width, data.height);
    let color = textureSample(texture, texture_sampler, uv);
    textureStore(previous_texture, vec2<i32>(frag_coord.xy / data.scale_factor), color);
    let gamma = 1.0;
    return vec4<f32>(
        pow(color.x, gamma),
        pow(color.y, gamma),
        pow(color.z, gamma),
        1.0
    );
    // return vec4<f32>(uv, 0.0, 1.0);
}

