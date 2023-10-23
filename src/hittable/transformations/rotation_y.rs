use crate::{hittable::base::{Hittable, HitRecord}, aabb::Aabb, ray::Ray, interval::Interval, utils::degrees_to_radians, vector::Vec3};

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl Hittable for RotateY {
    fn hit(&self, hit_ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        // Change the ray from world space to object space
        let mut origin = hit_ray.origin;
        let mut direction = hit_ray.direction;

        origin[0] = self.cos_theta * hit_ray.origin[0] - self.sin_theta * hit_ray.origin[2];
        origin[2] = self.sin_theta * hit_ray.origin[0] + self.cos_theta * hit_ray.origin[2];

        direction[0] =
            self.cos_theta * hit_ray.direction[0] - self.sin_theta * hit_ray.direction[2];
        direction[2] =
            self.sin_theta * hit_ray.direction[0] + self.cos_theta * hit_ray.direction[2];

        let rotated_r: Ray = Ray::new(origin, direction);

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

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone(&self) -> Box<dyn Hittable> {
        Box::new(RotateY {
            object: self.object.clone(),
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
            bbox: Aabb::new(),
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

        temp_rot_y.bbox = Aabb::new_point(&min, &max);

        temp_rot_y
    }
}