// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    fastrand::f64()
}

pub fn random_double_min_max(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_double()
}

pub fn random_int_min_max(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    fastrand::i32(min..=max)
}
