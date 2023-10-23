use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    interval::Interval,
    material::{base::Material, lambertian::Lambertian},
    ray::Ray,
    vector::{dot, Vec3},
};

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Box<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new_default(),
            normal: Vec3::new_default(),
            t: 0.0,
            mat: Box::new(Lambertian::new(&Vec3::new(1.0, 0.0, 1.0))),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.direction, &outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }

    pub fn clone(&self) -> HitRecord {
        HitRecord {
            point: self.point,
            normal: self.normal,
            t: self.t,
            mat: self.mat.clone(),
            front_face: self.front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}
