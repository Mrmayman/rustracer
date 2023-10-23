use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    aabb::AABB,
    interval::Interval,
    material::base::Material,
    ray::Ray,
    vector::{dot, Vec3},
};

use super::base::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Box<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f64, mat: Box<dyn Material>) -> Sphere {
        let radius_vector = Vec3::new(radius, radius, radius);
        Sphere {
            center: *center,
            radius,
            mat,
            bbox: AABB::new_point(&(*center + radius_vector), &(*center - radius_vector)),
        }
    }

    pub fn get_sphere_uv(p: &Vec3, u: &mut f64, v: &mut f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        // let theta: f64 = f64::acos(-p.y());
        // let phi: f64 = f64::atan2(-p.z(), p.x()) + std::f64::consts::PI;

        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        *u = 1.0 - (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
        *v = (theta + std::f64::consts::FRAC_PI_2) / std::f64::consts::PI;

        // *u = phi / (2.0*std::f64::consts::PI);
        // *v = theta / std::f64::consts::PI;
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
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn clone(&self) -> Box<dyn Hittable> {
        Box::new(Sphere {
            center: self.center,
            radius: self.radius,
            mat: self.mat.clone(),
            bbox: self.bbox.clone(),
        })
    }
}
