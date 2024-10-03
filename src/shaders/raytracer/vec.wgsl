fn vec_near_zero(vector: vec3<f32>) -> bool {
    let s = pow(10.0, -8.0);
    return (abs(vector.x) < s) && (abs(vector.y) < s) && (abs(vector.z) < s);
}

fn vec_reflected(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - (2 * dot(v, n) * n);
}

fn vec_refract(uv: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    let cos_theta = min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -sqrt(abs(1.0 - vec_length_squared(r_out_perp))) * n;
    return r_out_perp + r_out_parallel;
}

fn vec_length_squared(vector: vec3<f32>) -> f32 {
    return (vector.x * vector.x) + (vector.y * vector.y) + (vector.z * vector.z);
}

fn vec_length(vector: vec3<f32>) -> f32 {
    return sqrt(vec_length_squared(vector));
}

fn vec_unit_vector(vector: vec3<f32>) -> vec3<f32> {
    return vector * inverseSqrt(vec_length_squared(vector));
}