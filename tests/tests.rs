// tests/tests.rs

use LAMS_rs::linear_algebra::*;

#[test]
fn test_new_vector() {
    let v = linear_algebra::Vector::new(3);
    assert_eq!(v.size, 3);
    assert_eq!(v.data, vec![0.0; 3]);
    assert_eq!(v.data.len(), 3);
}

#[test]
fn test_add_vectors() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(3);

    let v3 = v1.add(&v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![0.0; 3]);
    assert_eq!(v3.data.len(), 3);
}
