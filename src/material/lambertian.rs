use rand_xorshift::XorShiftRng;

use crate::{
    hittable::base::HitRecord,
    ray::Ray,
    vector::{random_unit_vector, Vec3},
};

use super::base::Material;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(color: &Vec3) -> Lambertian {
        Lambertian {
            albedo: color.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.point, &scatter_direction);
        *attenuation = self.albedo;
        return true;
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: self.albedo,
        })
    }
}
