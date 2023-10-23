use rand_xorshift::XorShiftRng;

use crate::{hittable::base::HitRecord, ray::Ray, vector::Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool;

    fn clone(&self) -> Box<dyn Material>;
}
