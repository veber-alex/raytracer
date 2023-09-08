use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        let x = Interval::new(a[0].min(b[0]), a[0].max(b[0]));
        let y = Interval::new(a[1].min(b[1]), a[1].max(b[1]));
        let z = Interval::new(a[2].min(b[2]), a[2].max(b[2]));

        Self::from_intervals(x, y, z)
    }

    pub fn from_aabs(box0: Self, box1: Self) -> Self {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Self::from_intervals(x, y, z)
    }

    pub fn axis(self, n: i32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn hit(self, r: Ray, mut ray_t: Interval) -> bool {
        for a in 0..3 {
            let invd = 1. / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * invd;
            let mut t1 = (self.axis(a).max - orig) * invd;

            if invd < 0. {
                (t0, t1) = (t1, t0)
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
