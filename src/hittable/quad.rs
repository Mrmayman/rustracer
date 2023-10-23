use std::sync::Arc;

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

impl Hittable for Quad {
    fn hit(&self, hit_ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, &hit_ray.direction);

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, &hit_ray.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        // Determine the hit point lies within the planar shape using its plane coordinates.
        let intersection = hit_ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(planar_hitpt_vector, self.v));
        let beta = dot(&self.w, &cross(self.u, planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, hit_record) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        hit_record.t = t;
        hit_record.point = intersection;
        hit_record.mat = self.mat.clone();
        hit_record.set_face_normal(hit_ray, &self.normal);

        true
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
            w: self.w,
        })
    }
}

impl Quad {
    pub fn set_bounding_box(&mut self) {
        self.bbox = AABB::new_point(&self.q, &(self.q + self.u + self.v)).pad();
    }

    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Quad {
        let n = cross(u, v);
        let normal = n.unit_vector();
        let mut temp_quad = Quad {
            q,
            u,
            v,
            mat,
            bbox: AABB::new(),
            normal,
            d: dot(&normal, &q),
            w: n / dot(&n, &n),
        };

        temp_quad.set_bounding_box();

        temp_quad
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

/*pub struct Translate {
    pub object: Box<dyn Hittable>,
    pub offset: Vec3,
    bbox: AABB,
}

impl Hittable for Translate {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r: Ray = Ray::new(
            hit_ray.origin() - self.offset,
            hit_ray.direction(),
            hit_ray.time(),
        );

        // Determine where (if any) an intersection occurs along the offset ray
        if !self.object.hit(&offset_r, ray_t, hit_record) {
            return false;
        }

        // Move the intersection point forwards by the offset
        hit_record.point = hit_record.point + self.offset;

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn get_translate(&self) -> (f64, f64, f64) {
        (self.offset.x(), self.offset.y(), self.offset.z())
    }

    fn set_translate(&mut self, pos: (f64, f64, f64)) {
        self.offset = Vec3::new(pos.0, pos.1, pos.2);
    }

    fn clone_custom(&self) -> Box<dyn Hittable> {
        Box::new(Translate {
            object: self.object.clone_custom(),
            offset: self.offset,
            bbox: self.bbox.clone(),
        })
    }
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl Hittable for RotateY {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = hit_ray.origin();
        let mut direction = hit_ray.direction();

        origin[0] = self.cos_theta * hit_ray.origin()[0] - self.sin_theta * hit_ray.origin()[2];
        origin[2] = self.sin_theta * hit_ray.origin()[0] + self.cos_theta * hit_ray.origin()[2];

        direction[0] =
            self.cos_theta * hit_ray.direction()[0] - self.sin_theta * hit_ray.direction()[2];
        direction[2] =
            self.sin_theta * hit_ray.direction()[0] + self.cos_theta * hit_ray.direction()[2];

        let rotated_r: Ray = Ray::new(origin, direction, hit_ray.time());

        // Determine where (if any) an intersection occurs in object space
        if !self.object.hit(&rotated_r, ray_t, hit_record) {
            return false;
        }

        // Change the intersection point from object space to world space
        let mut p = hit_record.point;
        p[0] = self.cos_theta * hit_record.point[0] + self.sin_theta * hit_record.point[2];
        p[2] = -self.sin_theta * hit_record.point[0] + self.cos_theta * hit_record.point[2];

        // Change the normal from object space to world space
        let mut normal = hit_record.normal;
        normal[0] = self.cos_theta * hit_record.normal[0] + self.sin_theta * hit_record.normal[2];
        normal[2] = -self.sin_theta * hit_record.normal[0] + self.cos_theta * hit_record.normal[2];

        hit_record.point = p;
        hit_record.normal = normal;

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn get_translate(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 0.0)
    }

    fn set_translate(&mut self, pos: (f64, f64, f64)) {}

    fn clone_custom(&self) -> Box<dyn Hittable> {
        Box::new(RotateY {
            object: self.object.clone_custom(),
            sin_theta: self.sin_theta,
            cos_theta: self.cos_theta,
            bbox: self.bbox.clone(),
        })
    }
}

impl RotateY {
    pub fn new(p: Box<dyn Hittable>, angle: f64) -> RotateY {
        let mut temp_rot_y = RotateY {
            object: p,
            sin_theta: 0.0,
            cos_theta: 0.0,
            bbox: AABB::new(),
        };

        let radians = degrees_to_radians(angle);
        temp_rot_y.sin_theta = radians.sin();
        temp_rot_y.cos_theta = radians.cos();
        temp_rot_y.bbox = temp_rot_y.object.bounding_box();

        let mut min: Vec3 = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max: Vec3 = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64 * temp_rot_y.bbox.x.max)
                        + ((1 - i) as f64 * temp_rot_y.bbox.x.min);
                    let y = (j as f64 * temp_rot_y.bbox.y.max)
                        + ((1 - j) as f64 * temp_rot_y.bbox.y.min);
                    let z = (k as f64 * temp_rot_y.bbox.z.max)
                        + ((1 - k) as f64 * temp_rot_y.bbox.z.min);

                    let newx = temp_rot_y.cos_theta * x + temp_rot_y.sin_theta * z;
                    let newz = -temp_rot_y.sin_theta * x + temp_rot_y.cos_theta * z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }

        temp_rot_y.bbox = AABB::new_point(&min, &max);

        temp_rot_y
    }
}
*/