fn rng_int(state: ptr<function, u32>) {
    // PCG random number generator
    // Based on https://www.shadertoy.com/view/XlGcRh

    let oldState = *state + 747796405u + 2891336453u;
    let word = ((oldState >> ((oldState >> 28u) + 4u)) ^ oldState) * 277803737u;
    *state = (word >> 22u) ^ word;
}

fn rng_float(state: ptr<function, u32>) -> f32 {
    rng_int(state);
    return f32(*state) / f32(0xffffffffu);
}

fn rng_float_range(state: ptr<function, u32>, min: f32, max: f32) -> f32 {
    let raw = (rng_float(state) + 1.0) / 2.0;
    return min + ((max - min) * raw);
}

fn rng_vec(state: ptr<function, u32>) -> vec3<f32> {
    return vec3<f32>(rng_float(state), rng_float(state), rng_float(state));
}

fn rng_vec_range(state: ptr<function, u32>, min: f32, max: f32) -> vec3<f32> {
    return vec3<f32>(rng_float_range(state, min, max), rng_float_range(state, min, max), rng_float_range(state, min, max));
}

fn rng_vec_unit_sphere(state: ptr<function, u32>) -> vec3<f32> {
    let r = pow(rng_float(state), 0.33333f);
    let theta = pi * rng_float(state);
    let phi = 2f * pi * rng_float(state);

    let x = r * sin(theta) * cos(phi);
    let y = r * sin(theta) * sin(phi);
    let z = r * cos(theta);

    return vec3(x, y, z);
}

fn rng_vec_unit_disk(state: ptr<function, u32>) -> vec3<f32> {
    let r = pow(rng_float(state), 0.33333f);
    let theta = pi * rng_float(state);
    let phi = 2f * pi * rng_float(state);

    let x = r * sin(theta) * cos(phi);
    let y = r * sin(theta) * sin(phi);

    return vec3(x, y, 0.0);
}

fn rng_vec_unit_vector(state: ptr<function, u32>) -> vec3<f32> {
    return vec_unit_vector(rng_vec_unit_sphere(state));
}

fn rng_vec_unit_hemisphere(state: ptr<function, u32>, normal: vec3<f32>) -> vec3<f32> {
    // let r1 = rng_float(state);
    // let r2 = rng_float(state);

    // let phi = 2f * pi * r1;
    // let sinTheta = sqrt(1f - r2 * r2);

    // let x = cos(phi) * sinTheta;
    // let y = sin(phi) * sinTheta;
    // let z = r2;

    // return vec3(x, y, z);

    let sphere = rng_vec_unit_vector(state);
    if dot(sphere, normal) > 0.0 {
        return sphere;
    } else {
        return -sphere;
    }
}

fn rng_init(pixel: vec2<u32>, resolution: vec2<u32>, frame: u32) -> u32 {
    // Adapted from https://github.com/boksajak/referencePT
    let seed = dot(pixel, vec2<u32>(1u, resolution.x)) ^ jenkinsHash(frame);
    return jenkinsHash(seed);
}

fn jenkinsHash(input: u32) -> u32 {
    var x = input;
    x += x << 10u;
    x ^= x >> 6u;
    x += x << 3u;
    x ^= x >> 11u;
    x += x << 15u;
    return x;
}