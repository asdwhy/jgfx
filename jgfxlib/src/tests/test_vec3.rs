use crate::utils::equal;
use crate::vec3::Vec3;

use std::f64::consts::PI;

static TEST_VEC: (f64, f64, f64) = (1f64, 2f64, 3f64);
static TEST_VEC2: (f64, f64, f64) = (5f64, 4f64, 1f64);
static TEST_VEC3: (f64, f64, f64) = (10f64, -1f64, 13f64);
static TEST_VEC4: (f64, f64, f64) = (1f64, 2f64, -3f64);
static TEST_VEC5: (f64, f64, f64) = (PI, PI, -PI * 2.0);

static SCALAR: f64 = 5f64;

#[test]
fn test_vec3_xyz() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    assert_eq!(v.x, TEST_VEC.0);
    assert_eq!(v.y, TEST_VEC.1);
    assert_eq!(v.z, TEST_VEC.2);
}

#[test]
fn test_vec3_length() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let lensq = TEST_VEC.0 * TEST_VEC.0 + TEST_VEC.1 * TEST_VEC.1 + TEST_VEC.2 * TEST_VEC.2;

    assert_eq!(v.length_squared(), lensq);
    assert_eq!(v.length(), f64::sqrt(lensq));
}


#[test]
fn test_vec3_dot() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let dot = v.x * u.x + v.y * u.y + v.z * u.z;

    assert_eq!(v.dot(&u), dot);
    assert_eq!(u.dot(&v), dot);
}

#[test]
fn test_vec3_cross() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let cross_x = -10f64;
    let cross_y = 14f64;
    let cross_z = -6f64;

    let cross = v.cross(&u);
    assert_eq!(cross_x, cross.x);
    assert_eq!(cross_y, cross.y);
    assert_eq!(cross_z, cross.z);

    let cross2 = u.cross(&v);
    assert_eq!(-cross_x, cross2.x);
    assert_eq!(-cross_y, cross2.y);
    assert_eq!(-cross_z, cross2.z);
}

#[test]
fn test_vec3_normalized() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let ref vn = v.normalized();
    let ref un = u.normalized();

    assert_eq!(equal(vn.length(), 1f64), true);
    assert_eq!(equal(un.length(), 1f64), true);
}

#[test]
fn test_vec3_add_owned() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 + TEST_VEC2.0;
    let expected_y = TEST_VEC.1 + TEST_VEC2.1;
    let expected_z = TEST_VEC.2 + TEST_VEC2.2;


    let actual = u + v;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let actual = v + u;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_add_ref() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 + TEST_VEC2.0;
    let expected_y = TEST_VEC.1 + TEST_VEC2.1;
    let expected_z = TEST_VEC.2 + TEST_VEC2.2;

    let ref actual = u + v;
    let ref actual2 = v + u;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    assert_eq!(actual2.x, expected_x);
    assert_eq!(actual2.y, expected_y);
    assert_eq!(actual2.z, expected_z);
}

#[test]
fn test_vec3_add_multiple() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let ref v4 = Vec3::new(TEST_VEC4.0, TEST_VEC4.1, TEST_VEC4.2);

    let expected_x = TEST_VEC.0 + TEST_VEC2.0 + TEST_VEC3.0 + TEST_VEC4.0;
    let expected_y = TEST_VEC.1 + TEST_VEC2.1 + TEST_VEC3.1 + TEST_VEC4.1;
    let expected_z = TEST_VEC.2 + TEST_VEC2.2 + TEST_VEC3.2 + TEST_VEC4.2;

    let ref actual = v1 + v2 + v3 + v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = (v1 + v2) + v3 + v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = (v1 + (v2 + v3)) + v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = v1 + (v2 + (v3 + v4));
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_sub_owned() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 - TEST_VEC2.0;
    let expected_y = TEST_VEC.1 - TEST_VEC2.1;
    let expected_z = TEST_VEC.2 - TEST_VEC2.2;

    let actual = v - u;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    
    
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let actual = u - v;

    assert_eq!(actual.x, -expected_x);
    assert_eq!(actual.y, -expected_y);
    assert_eq!(actual.z, -expected_z);
}

#[test]
fn test_vec3_sub_ref() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 - TEST_VEC2.0;
    let expected_y = TEST_VEC.1 - TEST_VEC2.1;
    let expected_z = TEST_VEC.2 - TEST_VEC2.2;

    let ref actual = v - u;
    let ref actual2 = u - v;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    assert_eq!(actual2.x, -expected_x);
    assert_eq!(actual2.y, -expected_y);
    assert_eq!(actual2.z, -expected_z);
}

#[test]
fn test_vec3_sub_multiple() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let ref v4 = Vec3::new(TEST_VEC4.0, TEST_VEC4.1, TEST_VEC4.2);

    let expected_x = TEST_VEC.0 - TEST_VEC2.0 - TEST_VEC3.0 - TEST_VEC4.0;
    let expected_y = TEST_VEC.1 - TEST_VEC2.1 - TEST_VEC3.1 - TEST_VEC4.1;
    let expected_z = TEST_VEC.2 - TEST_VEC2.2 - TEST_VEC3.2 - TEST_VEC4.2;

    let ref actual = v1 - v2 - v3 - v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = (v1 - v2) - v3 - v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = ((v1 - v2) - v3) - v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let expected2_x = TEST_VEC.0 - (TEST_VEC2.0 - (TEST_VEC3.0 - TEST_VEC4.0));
    let expected2_y = TEST_VEC.1 - (TEST_VEC2.1 - (TEST_VEC3.1 - TEST_VEC4.1));
    let expected2_z = TEST_VEC.2 - (TEST_VEC2.2 - (TEST_VEC3.2 - TEST_VEC4.2));

    let ref actual = v1 - (v2 - (v3 - v4));
    assert_eq!(actual.x, expected2_x);
    assert_eq!(actual.y, expected2_y);
    assert_eq!(actual.z, expected2_z);
}

#[test]
fn test_vec3_mul_owned() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 * TEST_VEC2.0;
    let expected_y = TEST_VEC.1 * TEST_VEC2.1;
    let expected_z = TEST_VEC.2 * TEST_VEC2.2;

    let actual = v * u;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    
    
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let actual = u * v;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_mul_ref() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref u = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);

    let expected_x = TEST_VEC.0 * TEST_VEC2.0;
    let expected_y = TEST_VEC.1 * TEST_VEC2.1;
    let expected_z = TEST_VEC.2 * TEST_VEC2.2;

    let ref actual = v * u;
    let ref actual2 = u * v;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    assert_eq!(actual2.x, expected_x);
    assert_eq!(actual2.y, expected_y);
    assert_eq!(actual2.z, expected_z);
}

#[test]
fn test_vec3_mul_multiple() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let ref v4 = Vec3::new(TEST_VEC4.0, TEST_VEC4.1, TEST_VEC4.2);

    let expected_x = TEST_VEC.0 * TEST_VEC2.0 * TEST_VEC3.0 * TEST_VEC4.0;
    let expected_y = TEST_VEC.1 * TEST_VEC2.1 * TEST_VEC3.1 * TEST_VEC4.1;
    let expected_z = TEST_VEC.2 * TEST_VEC2.2 * TEST_VEC3.2 * TEST_VEC4.2;

    let ref actual = v1 * v2 * v3 * v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = (v1 * v2) * v3 * v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = v1 * ((v2 * v3) * v4);
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_mul_scalar_owned() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let expected_x = TEST_VEC.0 * SCALAR;
    let expected_y = TEST_VEC.1 * SCALAR;
    let expected_z = TEST_VEC.2 * SCALAR;

    let actual = v * SCALAR;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    
    
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let actual = SCALAR * v;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_mul_scalar_ref() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let expected_x = TEST_VEC.0 * SCALAR;
    let expected_y = TEST_VEC.1 * SCALAR;
    let expected_z = TEST_VEC.2 * SCALAR;

    let ref actual = v * SCALAR;
    let ref actual2 = SCALAR * v;
    
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
    assert_eq!(actual2.x, expected_x);
    assert_eq!(actual2.y, expected_y);
    assert_eq!(actual2.z, expected_z);
}

#[test]
fn test_vec3_mul_scalar_multiple() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let ref v4 = Vec3::new(TEST_VEC4.0, TEST_VEC4.1, TEST_VEC4.2);

    let expected_x = TEST_VEC.0 * TEST_VEC2.0 * TEST_VEC3.0 * TEST_VEC4.0 * SCALAR * SCALAR;
    let expected_y = TEST_VEC.1 * TEST_VEC2.1 * TEST_VEC3.1 * TEST_VEC4.1 * SCALAR * SCALAR;
    let expected_z = TEST_VEC.2 * TEST_VEC2.2 * TEST_VEC3.2 * TEST_VEC4.2 * SCALAR * SCALAR;

    let ref actual = v1 * v2 * SCALAR * v3 * v4 * SCALAR;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = (SCALAR * v1 * v2) * (v3 * SCALAR) * v4;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

    let ref actual = SCALAR * v1 * ((v2 * v3) * SCALAR * v4);
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);

}

#[test]
fn test_vec3_div_scalar_owned() {
    let v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let expected_x = TEST_VEC.0 / SCALAR;
    let expected_y = TEST_VEC.1 / SCALAR;
    let expected_z = TEST_VEC.2 / SCALAR;

    let actual = v / SCALAR;

    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_div_scalar_ref() {
    let ref v = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    let expected_x = TEST_VEC.0 / SCALAR;
    let expected_y = TEST_VEC.1 / SCALAR;
    let expected_z = TEST_VEC.2 / SCALAR;

    let ref actual = v / SCALAR;
    
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_div_scalar_multiple() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);

    let expected_x = (((TEST_VEC.0 / SCALAR) * TEST_VEC2.0) / SCALAR) * TEST_VEC3.0;
    let expected_y = (((TEST_VEC.1 / SCALAR) * TEST_VEC2.1) / SCALAR) * TEST_VEC3.1;
    let expected_z = (((TEST_VEC.2 / SCALAR) * TEST_VEC2.2) / SCALAR) * TEST_VEC3.2;

    let ref actual = (((v1 / SCALAR) * v2) / SCALAR) * v3;
    assert_eq!(actual.x, expected_x);
    assert_eq!(actual.y, expected_y);
    assert_eq!(actual.z, expected_z);
}

#[test]
fn test_vec3_add_assign() {
    let v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let mut v4 = Vec3::new(0f64,0f64,0f64);
    
    v4 += v1;
    v4 += &v2;
    v4 += v3;

    let expected_x = TEST_VEC.0 + TEST_VEC2.0 + TEST_VEC3.0;
    let expected_y = TEST_VEC.1 + TEST_VEC2.1 + TEST_VEC3.1;
    let expected_z = TEST_VEC.2 + TEST_VEC2.2 + TEST_VEC3.2;

    assert_eq!(expected_x, v4.x);
    assert_eq!(expected_y, v4.y);
    assert_eq!(expected_z, v4.z);
}

#[test]
fn test_vec3_sub_assign() {
    let v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);
    let mut v4 = Vec3::new(0f64,0f64,0f64);
    
    v4 -= v1;
    v4 -= &v2;
    v4 -= v3;

    let expected_x = -TEST_VEC.0 - TEST_VEC2.0 - TEST_VEC3.0;
    let expected_y = -TEST_VEC.1 - TEST_VEC2.1 - TEST_VEC3.1;
    let expected_z = -TEST_VEC.2 - TEST_VEC2.2 - TEST_VEC3.2;

    assert_eq!(expected_x, v4.x);
    assert_eq!(expected_y, v4.y);
    assert_eq!(expected_z, v4.z);
}

#[test]
fn test_vec3_div_assign() {
    let mut v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    
    v1 /= SCALAR;

    let expected_x = TEST_VEC.0 / SCALAR;
    let expected_y = TEST_VEC.1 / SCALAR;
    let expected_z = TEST_VEC.2 / SCALAR;

    assert_eq!(expected_x, v1.x);
    assert_eq!(expected_y, v1.y);
    assert_eq!(expected_z, v1.z);
}

#[test]
fn test_vec3_mul_assign() {
    let mut v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref mut v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    
    v1 *= SCALAR;
    
    let expected_x = TEST_VEC.0 * SCALAR;
    let expected_y = TEST_VEC.1 * SCALAR;
    let expected_z = TEST_VEC.2 * SCALAR;

    assert_eq!(expected_x, v1.x);
    assert_eq!(expected_y, v1.y);
    assert_eq!(expected_z, v1.z);

    *v2 *= SCALAR;
    
    let expected_x = TEST_VEC2.0 * SCALAR;
    let expected_y = TEST_VEC2.1 * SCALAR;
    let expected_z = TEST_VEC2.2 * SCALAR;

    assert_eq!(expected_x, v2.x);
    assert_eq!(expected_y, v2.y);
    assert_eq!(expected_z, v2.z);
}

#[test]
fn test_vec3_equality() {
    let coef = 0.33555;

    let ref v1 = Vec3::new(TEST_VEC5.0 * coef, TEST_VEC5.1 * coef, TEST_VEC5.2 * coef);
    let ref mut v2 = Vec3::new(TEST_VEC5.0, TEST_VEC5.1, TEST_VEC5.2);

    *v2 = &(*v2) * std::f64::consts::PI * coef;
    *v2 /= std::f64::consts::PI;

    assert_eq!(v1, v2);
}

#[test]
fn test_vec3_unary_neg() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);
    let ref v2 = Vec3::new(TEST_VEC2.0, TEST_VEC2.1, TEST_VEC2.2);
    let ref v3 = Vec3::new(TEST_VEC3.0, TEST_VEC3.1, TEST_VEC3.2);

    let ref v1 = -v1;
    let ref v2 = -v2;
    let ref v3 = -v3;

    assert_eq!(-TEST_VEC.0, v1.x);
    assert_eq!(-TEST_VEC.1, v1.y);
    assert_eq!(-TEST_VEC.2, v1.z);

    assert_eq!(-TEST_VEC2.0, v2.x);
    assert_eq!(-TEST_VEC2.1, v2.y);
    assert_eq!(-TEST_VEC2.2, v2.z);

    assert_eq!(-TEST_VEC3.0, v3.x);
    assert_eq!(-TEST_VEC3.1, v3.y);
    assert_eq!(-TEST_VEC3.2, v3.z);
}

#[test]
fn test_vec3_index() {
    let ref v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    assert_eq!(TEST_VEC.0, v1[0]);
    assert_eq!(TEST_VEC.1, v1[1]);
    assert_eq!(TEST_VEC.2, v1[2]);
}

#[test]
fn test_vec3_index_mut() {
    let mut v1 = Vec3::new(TEST_VEC.0, TEST_VEC.1, TEST_VEC.2);

    assert_eq!(TEST_VEC.0, v1[0]);
    assert_eq!(TEST_VEC.1, v1[1]);
    assert_eq!(TEST_VEC.2, v1[2]);

    v1[0] = TEST_VEC2.0;
    v1[1] = TEST_VEC2.1;
    v1[2] = TEST_VEC2.2;

    assert_eq!(TEST_VEC2.0, v1[0]);
    assert_eq!(TEST_VEC2.1, v1[1]);
    assert_eq!(TEST_VEC2.2, v1[2]);
}