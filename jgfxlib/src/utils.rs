use crate::constants::EPSILON;

/// Returns true if a is roughly 0
pub fn is_zero(a: f64) -> bool {
    f64::abs(a) <  EPSILON
}

/// Returns true if a is roughly equal to b
pub fn equal(a: f64, b: f64) -> bool {
    f64::abs(a - b) <  EPSILON
}

/// Returns true if a is positive
pub fn is_positive(a: f64) -> bool {
    a > EPSILON
}

/// Returns true if a is negative
pub fn is_negative(a: f64) -> bool {
    a < -EPSILON
}

/// Returns true if a is in range [min,max]
pub fn in_range(a: f64, min: f64, max: f64) -> bool {
    a >= min && a <= max
}

/// Returns the greater of the values a,b
pub fn max(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

/// Returns the lesser of the values a,b
pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// Clamps values between min and max
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min }
    else if x > max { max }
    else { x }
}