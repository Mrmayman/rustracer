@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct WindowSize {
    width: f32,
    height: f32,
    scale_factor: f32,
};

@group(0) @binding(2) var<uniform> window_size: WindowSize;

@fragment
fn main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(window_size.width, window_size.height);
    let color = textureSample(texture, texture_sampler, uv);
    let gamma = 2.2;
    return vec4<f32>(
        pow(color.x, gamma),
        pow(color.y, gamma),
        pow(color.z, gamma),
        1.0
    );
}

