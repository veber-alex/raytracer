use enum_dispatch::enum_dispatch;

use crate::{
    aabb::Aabb,
    bvh::BvhNode,
    hittable_list::HittableList,
    interval::Interval,
    material::{AnyMaterial, Lambertian},
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;
}

#[enum_dispatch(Hittable)]
#[derive(Clone)]
pub enum AnyHittable {
    Sphere,
    HittableList,
    BvhNode,
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
            mat: Lambertian::default().into(),
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
