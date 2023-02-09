use crate::{
    utils::{is_zero, equal, in_range, max, fmin, fmax, clamp, sort_from},
    constants::EPSILON
};

#[test]
fn test_is_zero() {
    let small_number = 0.01 + EPSILON;
    let pretty_much_zero = EPSILON - EPSILON/2.0;
    
    assert_eq!(false, is_zero(small_number));
    assert_eq!(true, is_zero(pretty_much_zero));

}

#[test]
fn test_is_equal() {
    let a = 5.41232;
    let b = 5.41232 + EPSILON * 0.9999;
    let c = 5.41232 + EPSILON;
    
    assert_eq!(true, equal(a, b));
    assert_eq!(false, equal(a, c));
}

#[test]
fn test_in_range() {
    let a = 5.0;
    let b = 333.0;
    
    assert_eq!(false, in_range(4.9, a, b));
    assert_eq!(true, in_range(5.9, a, b));
    assert_eq!(true, in_range(333.0, a, b));
    assert_eq!(false, in_range(334.9, a, b));
}

#[test]
fn test_max() {
    let a = 5;
    let b = 333;
    
    assert_eq!(b, max(a, b));
}

#[test]
fn test_fmin() {
    let a = 5.0;
    let b = 333.0;
    
    assert_eq!(a, fmin(a, b));
}

#[test]
fn test_fmax() {
    let a = 5.0;
    let b = 333.0;
    
    assert_eq!(b, fmax(a, b));
}

#[test]
fn test_clamp() {
    let a = 5.0;
    let b = 333.0;
    
    assert_eq!(b, clamp(10000.0, a, b));
    assert_eq!(54.0, clamp(54.0, a, b));
    assert_eq!(a, clamp(-10000.0, a, b));
}

#[test]
fn test_sort_from() {
    let v = vec![4, 2, 6, 3, 4, 3, 8, 6];
    let mut v2 = v;

    sort_from(&mut v2, 0, 4, |a, b| {
       a.cmp(b)
    });

    assert_eq!(vec![2,3,4,6,4,3,8,6], v2);
}