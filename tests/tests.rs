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

#[test]
fn test_cross_vectors() {
    let mut v1 = linear_algebra::Vector::new(3);
    let mut v2 = linear_algebra::Vector::new(3);

    v1.set(vec![1.0, 2.0, 3.0]);
    v2.set(vec![3.0, 4.0, 5.0]);

    let v3 = linear_algebra::Vector::cross(&v1, &v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![-2.0, 4.0, -2.0]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_cross_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::cross(&v1, &v2);
}

#[test]
fn test_matrix_new() {
    let m = linear_algebra::Matrix::new(3, 3);
    assert_eq!(m.rows, 3);
    assert_eq!(m.cols, 3);
    assert_eq!(m.data, vec![vec![0.0; 3]; 3]);
    assert_eq!(m.data.len(), 3);
    assert_eq!(m.data[0].len(), 3);
}

#[test]
fn test_set_matrix() {
    let mut m = linear_algebra::Matrix::new(3, 3);

    m.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    assert_eq!(m.rows, 3);
    assert_eq!(m.cols, 3);
    assert_eq!(
        m.data,
        vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0]
        ]
    );
    assert_eq!(m.data.len(), 3);
    assert_eq!(m.data[0].len(), 3);
}

#[test]
fn test_add_matrices() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m3 = linear_algebra::Matrix::add(&m1, &m2);
    assert_eq!(m3.rows, 3);
    assert_eq!(m3.cols, 3);

    assert_eq!(
        m3.data,
        vec![
            vec![2.0, 4.0, 6.0],
            vec![8.0, 10.0, 12.0],
            vec![14.0, 16.0, 18.0]
        ]
    );

    assert_eq!(m3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_add_matrices_panic() {
    let m1 = linear_algebra::Matrix::new(3, 3);
    let m2 = linear_algebra::Matrix::new(4, 4);

    let _m3 = linear_algebra::Matrix::add(&m1, &m2);
}

#[test]
fn test_sub_matrices() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m3 = linear_algebra::Matrix::sub(&m1, &m2);
    assert_eq!(m3.rows, 3);
    assert_eq!(m3.cols, 3);

    assert_eq!(
        m3.data,
        vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0]
        ]
    );

    assert_eq!(m3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_sub_matrices_panic() {
    let m1 = linear_algebra::Matrix::new(3, 3);
    let m2 = linear_algebra::Matrix::new(4, 4);

    let _m3 = linear_algebra::Matrix::sub(&m1, &m2);
}

#[test]
fn test_scale_matrix() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m2 = linear_algebra::Matrix::scale(&m1, 2.0);

    assert_eq!(m2.rows, 3);
    assert_eq!(m2.cols, 3);
    assert_eq!(
        m2.data,
        vec![
            vec![2.0, 4.0, 6.0],
            vec![8.0, 10.0, 12.0],
            vec![14.0, 16.0, 18.0]
        ]
    );
    assert_eq!(m2.data.len(), 3);
}

#[test]
fn test_mul_matrices() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.set(vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0]);

    let m3 = linear_algebra::Matrix::mul(&m1, &m2);

    assert_eq!(m3.rows, 3);
    assert_eq!(m3.cols, 3);
    assert_eq!(
        m3.data,
        vec![
            vec![-14.0, 16.0, -18.0],
            vec![-26.0, 31.0, -36.0],
            vec![-38.0, 46.0, -54.0]
        ]
    );
}

#[test]
#[should_panic]
fn test_mul_matrices_panic() {
    let m1 = linear_algebra::Matrix::new(3, 3);
    let m2 = linear_algebra::Matrix::new(4, 4);

    let _m3 = linear_algebra::Matrix::mul(&m1, &m2);
}
