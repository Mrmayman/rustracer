use crate::{
    aabb::Aabb,
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
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Hittable for Quad {
    fn hit(&self, hit_ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let denom = dot(self.normal, hit_ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(self.normal, hit_ray.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = hit_ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(self.w, cross(planar_hitpt_vector, self.v));
        let beta = dot(self.w, cross(self.u, planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, hit_record) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        hit_record.t = t;
        hit_record.point = intersection;
        hit_record.mat = self.mat.clone();
        hit_record.set_face_normal(hit_ray, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
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
            w: self.w,
        })
    }
}

impl Quad {
    pub fn set_bounding_box(&mut self) {
        self.bbox = Aabb::new_point(&self.q, &(self.q + self.u + self.v)).pad();
    }

    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Quad {
        let n = cross(u, v);
        let normal = n.unit_vector();
        let mut temp_quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: Aabb::new(),
            normal,
            d: dot(normal, q),
            w: n / dot(n, n),
        };

        temp_quad.set_bounding_box();

        temp_quad
    }

    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}