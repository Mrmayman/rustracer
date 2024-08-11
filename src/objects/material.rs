#[repr(C)]
pub enum Material {
    Lambertian {
        albedo: [f32; 4],
        _padding: [f32; 8 - 5],
    },
    Metal {
        albedo: [f32; 4],
        fuzziness: f32,
        _padding: [f32; 8 - 6],
    },
    Dielectric {
        refraction_index: f32,
        _padding: [f32; 8 - 2],
    },
}
