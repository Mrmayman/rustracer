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

fn FsrEasuRF(uv: vec2<f32>) -> vec4<f32> {
    let color = textureSample(texture, texture_sampler, uv);
    return vec4<f32>(color.x);
}

fn FsrEasuGF(uv: vec2<f32>) -> vec4<f32> {
    let color = textureSample(texture, texture_sampler, uv);
    return vec4<f32>(color.y);
}

fn FsrEasuBF(uv: vec2<f32>) -> vec4<f32> {
    let color = textureSample(texture, texture_sampler, uv);
    return vec4<f32>(color.z);
}

@fragment
fn main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = frag_coord.xy / vec2<f32>(data.width, data.height);
    var color: vec4<f32>;

    if false {
        // AMD FSR Upscaling
        var con0: vec4<u32>;
        var con1: vec4<u32>;
        var con2: vec4<u32>;
        var con3: vec4<u32>;

        let ix = data.width / data.scale_factor;
        let iy = data.height / data.scale_factor;

        FsrEasuCon(&con0, &con1, &con2, &con3, ix, iy, ix, iy, data.width, data.height);
        var fsr_color = vec3(0.0);
        FsrEasuF(&fsr_color, vec2<u32>(frag_coord.xy), con0, con1, con2, con3);
        color = vec4(fsr_color.xyz, 0.0);
    } else if true {
        // Denoiser
        let sigma = 1.0;
        let ksigma = sigma * 2.0;
        color = smartDeNoise(uv, sigma, ksigma, 0.195);
    } else {
        // Nothing
        color = textureSample(texture, texture_sampler, uv);
    }

    textureStore(previous_texture, vec2<i32>(frag_coord.xy / data.scale_factor), color);
    let gamma = 1.0;
    return vec4<f32>(
        pow(color.x, gamma),
        pow(color.y, gamma),
        pow(color.z, gamma),
        1.0
    );
}

