use std::{rc::Rc, sync::Arc, cell::RefCell};

use crate::{
    ray::Ray,
    vector::{dot, Vec3}, interval::Interval, material::base::Material,
};

use super::base::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, mat: Box<dyn Material>) -> Sphere {
        Sphere {
            center: *center,
            radius,
            mat
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal: Vec3 = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}
