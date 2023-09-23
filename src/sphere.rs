use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::AnyMaterial,
    ray::Ray,
    rtweekend::PI,
    vec3::{Point3, Vec3},
};

#[derive(Clone)]
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: AnyMaterial,
    center_vec: Vec3,
    bbox: Aabb,
    is_moving: bool,
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
            is_moving: false,
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
            is_moving: true,
        }
    }

    pub fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + time * self.center_vec
    }

    fn get_sphere_uv(&self, p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2. * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center1
        };
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
        (rec.u, rec.v) = self.get_sphere_uv(outward_normal);
        rec.mat = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
