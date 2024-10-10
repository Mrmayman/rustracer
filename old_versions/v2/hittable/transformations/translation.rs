use crate::{aabb::Aabb, hittable::base::{Hittable, HitRecord}, ray::Ray, vector::Vec3, interval::Interval};

pub struct Translate {
    pub object: Box<dyn Hittable>,
    pub offset: Vec3,
    bbox: Aabb,
}

impl Hittable for Translate {
    fn hit(&self, hit_ray: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r: Ray = Ray::new(hit_ray.origin - self.offset, hit_ray.direction);

        // Determine where (if any) an intersection occurs along the offset ray
        if !self.object.hit(&offset_r, ray_t, hit_record) {
            return false;
        }

        // Move the intersection point forwards by the offset
        hit_record.point = hit_record.point + self.offset;

        return true;
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }

    fn clone(&self) -> Box<dyn Hittable> {
        Box::new(Translate {
            object: self.object.clone(),
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
