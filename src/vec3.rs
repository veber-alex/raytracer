use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::rtweekend::{random_double, random_double_min_max};

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    pub fn dot(self, other: Self) -> f64 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(self, other: Self) -> Self {
        Vec3::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            self.e[2] * other.e[0] - self.e[0] * other.e[2],
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
        )
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn random() -> Self {
        Self::new(random_double(), random_double(), random_double())
    }

    pub fn random_min_max(min: f64, max: f64) -> Self {
        Self::new(
            random_double_min_max(min, max),
            random_double_min_max(min, max),
            random_double_min_max(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_min_max(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        // In the same hemisphere as the normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Self::new(
                random_double_min_max(-1., 1.),
                random_double_min_max(-1., 1.),
                0.,
            );
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn reflect(self, other: Self) -> Self {
        self - 2. * self.dot(other) * other
    }

    pub fn refract(self, other: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(other).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * other);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * other;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        &self.e[index as usize]
    }
}

impl IndexMut<i32> for Vec3 {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        &mut self.e[index as usize]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1. / rhs;
    }
}

pub type Point3 = Vec3;

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}
