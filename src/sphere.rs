use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::AnyMaterial,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: AnyMaterial,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: impl Into<AnyMaterial>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_points(center - rvec, center + rvec);

        Self {
            center1: center,
            radius,
            mat: mat.into(),
            center_vec: Vec3::default(),
            bbox,
        }
    }

    pub fn moving(
        center1: Point3,
        center2: Point3,
        radius: f64,
        mat: impl Into<AnyMaterial>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        let bbox = Aabb::from_aabs(box1, box2);

        Self {
            center1,
            radius,
            mat: mat.into(),
            center_vec: center2 - center1,
            bbox,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = self.center(r.time());
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
