use std::rc::Rc;

use rand_xorshift::XorShiftRng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    random::random_int_range,
    ray::Ray,
};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BvhNode {
    fn hit(&self, hit_ray: &Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(hit_ray, ray_t) {
            return false;
        }

        let hit_left: bool = self.left.hit(hit_ray, ray_t, hit_record);
        let hit_right: bool;
        if hit_left {
            hit_right = self
                .right
                .hit(hit_ray, Interval::new(ray_t.min, hit_record.t), hit_record);
        } else {
            hit_right = self
                .right
                .hit(hit_ray, Interval::new(ray_t.min, ray_t.max), hit_record);
        }

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

impl BvhNode {
    pub fn new(
        src_objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        rng: &mut XorShiftRng,
    ) -> BvhNode {
        let objects = src_objects; // Create a modifiable array of the source scene objects

        let axis: i32 = random_int_range(0, 2, rng);
        type CompareFunction = fn(Rc<dyn Hittable>, Rc<dyn Hittable>) -> bool;
        let comparator: CompareFunction;
        if axis == 0 {
            comparator = BvhNode::box_x_compare;
        } else if axis == 1 {
            comparator = BvhNode::box_y_compare;
        } else {
            comparator = BvhNode::box_z_compare;
        }

        let object_span: usize = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if object_span == 1 {
            right = objects[start].clone();
            left = objects[start].clone();
        } else if (object_span == 2) {
            if comparator(objects[start].clone(), objects[start + 1].clone()) {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            } else {
                left = objects[start + 1].clone();
                right = objects[start].clone();
            }
        } else {
            // Port this line
            // std::sort(objects.begin() + start, objects.begin() + end, comparator);
            let mut objects_slice: Vec<Rc<dyn Hittable>> = objects[start..end]
                .iter()
                .map(|item| Rc::clone(item))
                .collect();
            objects_slice.sort_by(|a, b| {
                if comparator(a.clone(), b.clone()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid: usize = start + object_span / 2;
            left = Rc::new(BvhNode::new(objects, start, mid, rng));
            right = Rc::new(BvhNode::new(objects, mid, end, rng));
        }

        let bbox = AABB::new_aabb(&left.bounding_box(), &right.bounding_box());

        BvhNode {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    pub fn new_list(list: &HittableList, rng: &mut XorShiftRng) -> BvhNode {
        BvhNode::new(&list.objects, 0, list.objects.len(), rng)
    }

    fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis_index: i32) -> bool {
        return a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min;
    }

    fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 0);
    }

    fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 1);
    }

    fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 2);
    }
}
