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
fn test_set_vector() {
    let mut v = linear_algebra::Vector::new(3);
    v.set(vec![1.0, 2.0, 3.0]);
    assert_eq!(v.size, 3);
    assert_eq!(v.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(v.data.len(), 3);
}

#[test]
#[should_panic]
fn test_set_vector_panic() {
    let mut v = linear_algebra::Vector::new(3);
    v.set(vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_add_vectors() {
    let mut v1 = linear_algebra::Vector::new(3);
    let mut v2 = linear_algebra::Vector::new(3);

    v1.set(vec![1.0, 2.0, 3.0]);
    v2.set(vec![3.0, 4.0, 5.0]);

    let v3 = v1.add(&v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![4.0, 6.0, 8.0]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_add_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = v1.add(&v2);
}

#[test]
fn test_sub_vectors() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(3);

    let v3 = v1.sub(&v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![0.0; 3]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_sub_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = v1.sub(&v2);
}

#[test]
fn test_scale_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.set(vec![1.0, 2.0, 3.0]);

    let v2 = v1.scale(2.0);
    assert_eq!(v2.size, 3);
    assert_eq!(v2.data, vec![2.0, 4.0, 6.0]);
    assert_eq!(v2.data.len(), 3);
}

#[test]
fn test_dot_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    let mut v2 = linear_algebra::Vector::new(3);

    v1.set(vec![1.0, 2.0, 3.0]);
    v2.set(vec![3.0, 4.0, 5.0]);

    let v3 = linear_algebra::Vector::dot(&v1, &v2);
    assert_eq!(v3, 26.0);
}

#[test]
#[should_panic]
fn test_dot_vector_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::dot(&v1, &v2);
}

#[test]
fn test_norm_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.set(vec![1.0, 2.0, 2.0]);

    let v2 = linear_algebra::Vector::norm(&v1);
    assert_eq!(v2, 3.0);
}

#[test]
fn test_normalize_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.set(vec![1.0, 2.0, 3.0]);

    let v2 = linear_algebra::Vector::normalize(&v1);
    assert_eq!(v2.size, 3);
    assert_eq!(
        v2.data,
        vec![0.2672612419124244, 0.5345224838248488, 0.8017837257372732]
    );
    assert_eq!(v2.data.len(), 3);
}
