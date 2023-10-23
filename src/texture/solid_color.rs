use crate::vector::Vec3;

use super::base::Texture;

pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new_vector(color: Vec3) -> SolidColor {
        SolidColor {
            color,
        }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color: Vec3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.color
    }
}
