use crate::{
    hittable::{AnyHittable, HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<AnyHittable>,
}

impl HittableList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, object: impl Into<AnyHittable>) {
        self.objects.push(object.into());
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
