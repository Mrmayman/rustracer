#[repr(C)]
#[derive(Clone)]
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

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: [1.0, 0.0, 1.0, 1.0],
            _padding: Default::default(),
        }
    }
}
