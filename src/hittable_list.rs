use crate::{
    aabb::Aabb,
    hittable::{AnyHittable, HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<AnyHittable>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }

    pub fn from_hittable(hittable: impl Into<AnyHittable>) -> Self {
        let mut this = Self::new();
        this.add(hittable.into());

        this
    }

    pub fn add(&mut self, object: impl Into<AnyHittable>) {
        let object = object.into();
        self.bbox = Aabb::from_aabs(self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &*self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
