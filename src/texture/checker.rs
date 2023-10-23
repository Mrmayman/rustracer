use std::rc::Rc;

use crate::vector::Vec3;

use super::{base::Texture, solid_color::SolidColor};

pub struct Checker {
    scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl Checker {
    pub fn new_texture(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Checker {
        Checker { scale, even, odd }
    }

    pub fn new_color(scale: f64, even: &Vec3, odd: &Vec3) -> Checker {
        Checker {
            scale,
            even: Rc::new(SolidColor::new_vector(even)),
            odd: Rc::new(SolidColor::new_vector(odd)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        let x_integer = (self.scale * p.x()) as i64;
        let y_integer = (self.scale * p.y()) as i64;
        let z_integer = (self.scale * p.z()) as i64;

        let is_even: bool = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }
    }
}
