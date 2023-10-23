use std::sync::Arc;

use rand_xorshift::XorShiftRng;

use crate::{hittable::base::HitRecord, ray::Ray, texture::base::Texture, vector::Vec3};

use super::base::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        false
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(DiffuseLight {
            emit: self.emit.clone(),
        })
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}
