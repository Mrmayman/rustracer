use rand_xorshift::XorShiftRng;

use crate::{
    hittable::base::HitRecord,
    ray::Ray,
    vector::{reflect, Vec3, random_unit_vector},
};

use super::base::Material;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: color,
            fuzz
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        let reflected: Vec3 = reflect(r_in.direction.unit_vector(), rec.normal);
        *scattered = Ray::new(rec.point, reflected + (self.fuzz * random_unit_vector(rng)));
        *attenuation = self.albedo;
        true
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Metal {
            albedo: self.albedo,
            fuzz: self.fuzz
        })
    }
}
