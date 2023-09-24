use enum_dispatch::enum_dispatch;

use crate::{
    aabb::Aabb,
    bvh::BvhNode,
    hittable_list::HittableList,
    interval::Interval,
    material::AnyMaterial,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
}

#[enum_dispatch(Hittable)]
#[derive(Clone)]
pub enum AnyHittable {
    Sphere,
    HittableList,
    BvhNode,
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: AnyMaterial,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        r: Ray,
        p: Point3,
        outward_normal: Vec3,
        mat: AnyMaterial,
        t: f64,
        u: f64,
        v: f64,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
}
