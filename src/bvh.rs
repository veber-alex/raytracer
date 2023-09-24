use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{AnyHittable, HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    rtweekend::random_int_min_max,
};

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<AnyHittable>,
    right: Arc<AnyHittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut objects: Vec<AnyHittable>, start: usize, end: usize) -> Self {
        let axis = random_int_min_max(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                if comparator(&objects[start], &objects[start + 1]) {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects[start..end].sort_unstable_by(|a, b| {
                    if comparator(a, b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                let mid = start + object_span / 2;
                let left = Self::new(objects.clone(), start, mid);
                let right = Self::new(objects, mid, end);

                (left.into(), right.into())
            }
        };

        Self {
            bbox: Aabb::from_aabs(left.bounding_box(), right.bounding_box()),
            left: Arc::new(left),
            right: Arc::new(right),
        }
    }

    pub fn from_list(list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(list.objects, 0, len)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, hit_left.as_ref().map_or(ray_t.max, |rec| rec.t)),
        );

        match hit_right {
            Some(_) => hit_right,
            None => hit_left,
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

fn box_compare(a: &AnyHittable, b: &AnyHittable, axis_index: i32) -> bool {
    a.bounding_box().axis(axis_index).min < b.bounding_box().axis(axis_index).min
}

fn box_x_compare(a: &AnyHittable, b: &AnyHittable) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &AnyHittable, b: &AnyHittable) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &AnyHittable, b: &AnyHittable) -> bool {
    box_compare(a, b, 2)
}
