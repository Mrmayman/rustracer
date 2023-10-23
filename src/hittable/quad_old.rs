use std::rc::Rc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::base::Material,
    ray::Ray,
    vector::{cross, dot, Vec3},
};

use super::base::{HitRecord, Hittable};

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Box<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Quad {
        let n = cross(u, v);
        let normal = n.unit_vector();
        Quad {
            q,
            u,
            v,
            mat,
            bbox: AABB::new(),
            normal,
            d: dot(&normal, &q),
            w: n / dot(&n, &n),
        }
    }

    pub fn set_bounding_box(&mut self) {
        self.bbox = AABB::new_point(&self.q, &(self.q + self.u + self.v)).pad()
    }

    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        return true;
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, &ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, &ray.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(planar_hitpt_vector, self.v));
        let beta = dot(&self.w, &cross(self.u, planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.point = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(ray, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn clone(&self) -> Box<dyn Hittable> {
        Box::new(Quad {
            q: self.q,
            u: self.u,
            v: self.v,
            mat: self.mat.clone(),
            bbox: self.bbox.clone(),
            normal: self.normal,
            d: self.d,
            w: self.w
        })
    }
}
