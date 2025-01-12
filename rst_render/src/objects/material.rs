use std::fmt::Debug;

/// # Materials
///
/// Materials are used to define how light interacts with objects.
///
/// # Variants
/// - `Lambertian`: Rough matte finish, has no reflections.
/// - `Metal`: Reflective material.
/// - `Dielectric`: A glass-like material that refracts light.
/// - `Emissive`: A material that emits light.
///
/// Go over the documentation of each variant to see how to use them.
#[repr(C)]
#[derive(Clone)]
pub enum Material {
    /// # Lambertian material
    ///
    /// Rough matte finish, has no reflections.
    ///
    /// ## Fields
    /// - `albedo`: The color of the material
    /// - `_padding`: Internal data, just do `Default::default()`
    Lambertian {
        albedo: [f32; 3],
        _padding: [f32; 8 - 4],
    },

    /// # Metal material
    ///
    /// Reflective material with a certain amount of fuzziness.
    ///
    /// You can control how reflective/fuzzy it is by setting
    /// the `fuzziness` parameter.
    ///
    /// ## Fields
    /// - `albedo`: The color of the material
    /// - `fuzziness`: The amount of fuzziness
    /// - `_padding`: Internal data, just do `Default::default()`
    Metal {
        albedo: [f32; 3],
        fuzziness: f32,
        _padding: [f32; 8 - (1 + 1 + 3)],
    },

    /// # Dielectric material
    ///
    /// A glass-like material that refracts light.
    ///
    /// ## Fields
    /// - `refraction_index`: The index of refraction of the material
    ///   (usually around 1.05 for glass)
    /// - `tint`: The color of the material
    /// - `tint_alpha`: How much the material is tinted
    ///   (0.0 is no tint, 1.0 is fully tinted)
    /// - `_padding`: Internal data, just do `Default::default()`
    Dielectric {
        refraction_index: f32,
        tint: [f32; 3],
        tint_alpha: f32,
        _padding: [f32; 8 - (2 + 3 + 1)],
    },

    /// # Emissive material
    ///
    /// A material that emits light.
    ///
    /// ## Fields
    /// - `color`: The color of the light
    /// - `_padding`: Internal data, just do `Default::default()`
    Emissive {
        color: [f32; 3],
        _padding: [f32; 8 - 4],
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
            albedo: [1.0, 0.0, 1.0],
            _padding: Default::default(),
        }
    }
}
