use crate::{
    hittable_list::HittableList,
    interval::Interval,
    material::AnyMaterial,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn into_any_hittable(self) -> AnyHittable;
}

pub enum AnyHittable {
    Sphere(Sphere),
    List(HittableList),
}

impl Hittable for AnyHittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            AnyHittable::Sphere(h) => h.hit(r, ray_t, rec),
            AnyHittable::List(h) => h.hit(r, ray_t, rec),
        }
    }

    fn into_any_hittable(self) -> AnyHittable {
        self
    }
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: AnyMaterial,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: AnyMaterial::None,
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.
        self.front_face = r.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
