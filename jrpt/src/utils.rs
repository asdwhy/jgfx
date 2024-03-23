use std::cmp::Ordering;
use crate::constants::EPSILON;

/// Returns true if a is roughly 0
pub fn is_zero(a: f64) -> bool {
    f64::abs(a) <  EPSILON
}

/// Returns true if a is roughly equal to b
pub fn equal(a: f64, b: f64) -> bool {
    f64::abs(a - b) <  EPSILON
}

/// Returns true if a is in range [min,max] (inclusive)
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

/// Returns the greater of the values a,b
pub fn fmax(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// Clamps values between min and max
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min }
    else if x > max { max }
    else { x }
}

/// Sorts vector v, but only from index i (inclusive) to j (exclusive) using given comparator function
pub fn sort_from<T, F>(v: &mut Vec<T>, i: usize, j: usize, compare: F)
    where F: FnMut(&T, &T) -> Ordering
{
    let t_in_range = v.drain(i..j);

    let mut t_in_range: Vec<T> = t_in_range.collect();

    t_in_range.sort_by(compare);

    let mut c = 0;
    for t in t_in_range {
        v.insert(c+i, t);
        c += 1;
    }
}