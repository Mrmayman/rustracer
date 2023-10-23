use crate::{ray::Ray, interval::Interval, aabb::AABB};

use super::base::{HitRecord, Hittable};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![], bbox: AABB::new() }
    }

    pub fn new_add(object: Box<dyn Hittable>) -> HittableList {
        let mut temp_hittable_list = HittableList::new();
        temp_hittable_list.add(object);
        temp_hittable_list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::new_aabb(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    fn manual_clone_vec_custom(original_vec: &Vec<Box<dyn Hittable>>) -> Vec<Box<dyn Hittable>> {
        let mut cloned_vec: Vec<Box<dyn Hittable>> = Vec::new();
    
        for item in original_vec {
            // Clone each boxed trait object inside the original vector and push it into the new vector.
            let cloned_item: Box<dyn Hittable> = (*item).clone();
            cloned_vec.push(cloned_item);
        }
    
        cloned_vec
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn clone(&self) -> Box<dyn Hittable> {
        Box::new(HittableList {
            objects: HittableList::manual_clone_vec_custom(&self.objects),
            bbox: self.bbox.clone(),
        })
    }
}
