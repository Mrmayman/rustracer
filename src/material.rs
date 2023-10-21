use rand_xorshift::XorShiftRng;

use crate::{
    hittable::HitRecord,
    random::random_double,
    ray::Ray,
    texture::{SolidColor, Texture},
    vector::{dot, random_unit_vector, reflect, refract, Vec3},
};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool;
    fn clone_custom(&self) -> Box<dyn Material>;
    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct Lambertian {
    albedo: Box<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector(rng);

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.point, scatter_direction, ray.time());
        *attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        return true;
    }

    fn clone_custom(&self) -> Box<dyn Material> {
        let temp: Box<dyn Material> = Box::new(Lambertian {
            albedo: self.albedo.clone_custom(),
        });
        temp
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::new_default()
    }
}

impl Lambertian {
    pub fn new_color(color: &Vec3) -> Lambertian {
        Lambertian {
            albedo: Box::new(SolidColor::new(color)),
        }
    }

    pub fn new_texture(texture: Box<dyn Texture>) -> Lambertian {
        Lambertian { albedo: texture }
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzziness: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        let reflected = reflect(&ray.direction().unit_vector(), &hit_record.normal);
        *scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzziness * random_unit_vector(rng),
            ray.time(),
        );
        *attenuation = self.albedo;
        return dot(&scattered.direction(), &hit_record.normal) > 0.0;
    }

    fn clone_custom(&self) -> Box<dyn Material> {
        let temp: Box<dyn Material> = Box::new(Metal::new(&self.albedo, self.fuzziness));
        temp
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::new_default()
    }
}

impl Metal {
    pub fn new(color: &Vec3, fuzziness: f64) -> Metal {
        Metal {
            albedo: color.clone(),
            fuzziness: fuzziness.clone(),
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio: f64;
        if hit_record.front_face {
            refraction_ratio = 1.0 / self.index_of_refraction;
        } else {
            refraction_ratio = self.index_of_refraction;
        }
        let unit_direction: Vec3 = ray.direction().unit_vector();
        let cos_theta: f64 = f64::min(dot(&(-unit_direction), &hit_record.normal), 1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double(rng)
        {
            direction = reflect(&unit_direction, &hit_record.normal);
        } else {
            direction = refract(&unit_direction, &hit_record.normal, refraction_ratio);
        }

        *scattered = Ray::new(hit_record.point, direction, ray.time());
        return true;
    }

    fn clone_custom(&self) -> Box<dyn Material> {
        Box::new(Dielectric {
            index_of_refraction: self.index_of_refraction,
        })
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        Vec3::new_default()
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0: f64 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0);
    }
}

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
        rng: &mut XorShiftRng,
    ) -> bool {
        false
    }

    fn clone_custom(&self) -> Box<dyn Material> {
        Box::new(DiffuseLight {
            emit: self.emit.clone_custom(),
        })
    }

    fn emitted(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.emit.value(u, v, p)
    }
}

impl DiffuseLight {
    pub fn new_texture(texture: Box<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit: texture }
    }

    pub fn new_color(color: Vec3) -> DiffuseLight {
        DiffuseLight { emit: Box::new(SolidColor::new(&color)) }
    }
}
