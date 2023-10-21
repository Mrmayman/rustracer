use std::rc::Rc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::{Lambertian, Material},
    ray::{Ray, degrees_to_radians},
    vector::Vec3,
    vector::{cross, dot},
};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
            material: Rc::new(Lambertian::new_color(&Vec3::new(0.0, 0.0, 0.0))),
            u: 0.0,
            v: 0.0,
        }
    }

    fn set_face_normal(&mut self, _ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait Hittable {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere {
            center: center,
            radius: radius,
            material: material,
            bbox: AABB::new_point(&(center + rvec), &(center - rvec)),
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
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let current_ray_direction = hit_ray.direction();

        let oc: Vec3 = hit_ray.origin() - self.center;
        let a: f64 = current_ray_direction.length_squared();
        let half_b: f64 = dot(&oc, &current_ray_direction);
        let c: f64 = oc.length_squared() - self.radius * self.radius;
        let discriminant: f64 = (half_b * half_b) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant: f64 = discriminant.sqrt();
        let mut nearest_root = (-half_b - sqrt_discriminant) / a;
        if nearest_root <= ray_t.min || ray_t.max <= nearest_root {
            nearest_root = (-half_b + sqrt_discriminant) / a;
            if nearest_root <= ray_t.min || ray_t.max <= nearest_root {
                return false;
            }
        }

        hit_record.t = nearest_root;
        hit_record.point = hit_ray.at(hit_record.t);
        let outward_normal: Vec3 = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(hit_ray, &outward_normal);
        // hit_record.u = 0.0;
        // hit_record.v = 0.0;
        Sphere::get_sphere_uv(&outward_normal, &mut hit_record.u, &mut hit_record.v);
        // println!("{}", hit_record.u);
        hit_record.material = self.material.clone();

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new(),
        }
    }

    pub fn new_add(object: Rc<dyn Hittable>) -> HittableList {
        let mut temp_list = HittableList {
            objects: Vec::new(),
            bbox: AABB::new(),
        };
        temp_list.add(object);
        temp_list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = AABB::new_aabb(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = ray_t.max;

        for object in &self.objects {
            if (*object).hit(
                hit_ray,
                Interval {
                    min: ray_t.min,
                    max: closest_so_far,
                },
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Hittable for Quad {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let denom = dot(&self.normal, &hit_ray.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return false;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.d - dot(&self.normal, &hit_ray.origin())) / denom;
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
        hit_record.material = self.mat.clone();
        hit_record.set_face_normal(hit_ray, &self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

impl Quad {
    pub fn set_bounding_box(&mut self) {
        self.bbox = AABB::new_point(&self.q, &(self.q + self.u + self.v)).pad();
    }

    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Quad {
        let n = cross(u, v);
        let normal = n.unit_vector();
        let mut temp_quad = Quad {
            q: q,
            u: u,
            v: v,
            mat: mat,
            bbox: AABB::new(),
            normal: normal,
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

pub struct Translate {
    pub object: Rc<dyn Hittable>,
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
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            offset,
            bbox,
        }
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}

impl Hittable for RotateY {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = hit_ray.origin();
        let mut direction = hit_ray.direction();

        origin[0] = self.cos_theta*hit_ray.origin()[0] - self.sin_theta*hit_ray.origin()[2];
        origin[2] = self.sin_theta*hit_ray.origin()[0] + self.cos_theta*hit_ray.origin()[2];

        direction[0] = self.cos_theta*hit_ray.direction()[0] - self.sin_theta*hit_ray.direction()[2];
        direction[2] = self.sin_theta*hit_ray.direction()[0] + self.cos_theta*hit_ray.direction()[2];

        let rotated_r: Ray = Ray::new(origin, direction, hit_ray.time());

        // Determine where (if any) an intersection occurs in object space
        if !self.object.hit(&rotated_r, ray_t, hit_record) {
            return false;
        }

        // Change the intersection point from object space to world space
        let mut p = hit_record.point;
        p[0] =  self.cos_theta*hit_record.point[0] + self.sin_theta*hit_record.point[2];
        p[2] = -self.sin_theta*hit_record.point[0] + self.cos_theta*hit_record.point[2];

        // Change the normal from object space to world space
        let mut normal = hit_record.normal;
        normal[0] =  self.cos_theta*hit_record.normal[0] + self.sin_theta*hit_record.normal[2];
        normal[2] = -self.sin_theta*hit_record.normal[0] + self.cos_theta*hit_record.normal[2];

        hit_record.point = p;
        hit_record.normal = normal;

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

impl RotateY {
    pub fn new(p: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let mut temp_rot_y = RotateY {
            object: p,
            sin_theta: 0.0,
            cos_theta: 0.0,
            bbox: AABB::new()
        };

        let radians = degrees_to_radians(angle);
        temp_rot_y.sin_theta = radians.sin();
        temp_rot_y.cos_theta = radians.cos();
        temp_rot_y.bbox = temp_rot_y.object.bounding_box();

        let mut min: Vec3 = Vec3::new( f64::INFINITY,  f64::INFINITY,  f64::INFINITY);
        let mut max: Vec3 = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64 * temp_rot_y.bbox.x.max) + ((1-i) as f64 * temp_rot_y.bbox.x.min);
                    let y = (j as f64 * temp_rot_y.bbox.y.max) + ((1-j) as f64 * temp_rot_y.bbox.y.min);
                    let z = (k as f64 * temp_rot_y.bbox.z.max) + ((1-k) as f64 * temp_rot_y.bbox.z.min);

                    let newx =  temp_rot_y.cos_theta*x + temp_rot_y.sin_theta*z;
                    let newz = -temp_rot_y.sin_theta*x + temp_rot_y.cos_theta*z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

                    for c in 0..3{
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