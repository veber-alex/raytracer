use crate::rtweekend::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        Self::new(a.min.min(b.min), a.max.max(b.max))
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    // pub fn contains(self, x: f64) -> bool {
    //     self.min <= x && x <= self.max
    // }

    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    // pub fn expand(self, delta: f64) -> Self {
    //     let padding = delta / 2.;
    //     Self::new(self.min - padding, self.max + padding)
    // }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}
