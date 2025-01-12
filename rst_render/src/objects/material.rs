use std::fmt::Debug;

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
    Emissive {
        color: [f32; 4],
        _padding: [f32; 8 - 5],
    },
}

impl Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Material::Lambertian { albedo, .. } => f
                .debug_struct("Lambertian")
                .field("albedo", albedo)
                .finish(),
            Material::Metal {
                albedo, fuzziness, ..
            } => f
                .debug_struct("Metal")
                .field("albedo", albedo)
                .field("fuzziness", fuzziness)
                .finish(),
            Material::Dielectric {
                refraction_index, ..
            } => f
                .debug_struct("Dielectric")
                .field("refraction_index", refraction_index)
                .finish(),
            Material::Emissive { color, _padding } => {
                f.debug_struct("Emissive").field("color", color).finish()
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: [1.0, 0.0, 1.0, 1.0],
            _padding: Default::default(),
        }
    }
}
