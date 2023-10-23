use std::{rc::Rc, sync::Arc};

use rand_xorshift::XorShiftRng;

use crate::{
    hittable::base::HitRecord,
    ray::Ray,
    texture::{base::Texture, solid_color::SolidColor},
    vector::{random_unit_vector, Vec3},
};

use super::base::Material;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_color(color: &Vec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new_vector(color)),
        }
    }

    pub fn new_texture(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
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
        *attenuation = self.albedo.value(rec.u, rec.v, rec.point);
        return true;
    }

    fn clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian {
            albedo: self.albedo.clone(),
        })
    }
}
