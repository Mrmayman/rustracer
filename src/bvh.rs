use rand_xorshift::XorShiftRng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    random::random_int_range,
    ray::Ray,
};

pub struct BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
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

    fn clone_custom(&self) -> Box<dyn Hittable> {
        Box::new(BvhNode {
            left: self.left.clone_custom(),
            right: self.right.clone_custom(),
            bbox: self.bbox.clone(),
        })
    }
}

impl BvhNode {
    pub fn new(
        src_objects: &Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        rng: &mut XorShiftRng,
    ) -> BvhNode {
        let objects = src_objects; // Create a modifiable array of the source scene objects

        let axis: i32 = random_int_range(0, 2, rng);
        type CompareFunction = fn(Box<dyn Hittable>, Box<dyn Hittable>) -> bool;
        let comparator: CompareFunction;
        if axis == 0 {
            comparator = BvhNode::box_x_compare;
        } else if axis == 1 {
            comparator = BvhNode::box_y_compare;
        } else {
            comparator = BvhNode::box_z_compare;
        }

        let object_span: usize = end - start;

        let left: Box<dyn Hittable>;
        let right: Box<dyn Hittable>;

        if object_span == 1 {
            right = objects[start].clone_custom();
            left = objects[start].clone_custom();
        } else if (object_span == 2) {
            if comparator(
                objects[start].clone_custom(),
                objects[start + 1].clone_custom(),
            ) {
                left = objects[start].clone_custom();
                right = objects[start + 1].clone_custom();
            } else {
                left = objects[start + 1].clone_custom();
                right = objects[start].clone_custom();
            }
        } else {
            // Port this line
            // std::sort(objects.begin() + start, objects.begin() + end, comparator);
            let mut objects_slice: Vec<&Box<dyn Hittable>> = objects[start..end]
                .iter()
                .collect::<Vec<&Box<dyn Hittable>>>();
            objects_slice.sort_by(|a, b| {
                if comparator(a.clone_custom(), b.clone_custom()) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });

            let mid: usize = start + object_span / 2;
            left = Box::new(BvhNode::new(objects, start, mid, rng));
            right = Box::new(BvhNode::new(objects, mid, end, rng));
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

    fn box_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>, axis_index: i32) -> bool {
        return a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min;
    }

    fn box_x_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 0);
    }

    fn box_y_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 1);
    }

    fn box_z_compare(a: Box<dyn Hittable>, b: Box<dyn Hittable>) -> bool {
        return BvhNode::box_compare(a, b, 2);
    }
}
