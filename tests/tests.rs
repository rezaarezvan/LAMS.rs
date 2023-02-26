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
    v.vector_set(vec![1.0, 2.0, 3.0]);
    assert_eq!(v.size, 3);
    assert_eq!(v.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(v.data.len(), 3);
}

#[test]
#[should_panic]
fn test_set_vector_panic() {
    let mut v = linear_algebra::Vector::new(3);
    v.vector_set(vec![1.0, 2.0, 3.0, 4.0]);
}

#[test]
fn test_add_vectors() {
    let mut v1 = linear_algebra::Vector::new(3);
    let mut v2 = linear_algebra::Vector::new(3);

    v1.vector_set(vec![1.0, 2.0, 3.0]);
    v2.vector_set(vec![3.0, 4.0, 5.0]);

    let v3 = linear_algebra::Vector::vector_add(&v1, &v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![4.0, 6.0, 8.0]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_add_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::vector_add(&v1, &v2);
}

#[test]
fn test_sub_vectors() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(3);

    let v3 = linear_algebra::Vector::vector_sub(&v1, &v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![0.0; 3]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_sub_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::vector_sub(&v1, &v2);
}

#[test]
fn test_scale_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.vector_set(vec![1.0, 2.0, 3.0]);

    let v2 = linear_algebra::Vector::vector_scale(&v1, 2.0);
    assert_eq!(v2.size, 3);
    assert_eq!(v2.data, vec![2.0, 4.0, 6.0]);
    assert_eq!(v2.data.len(), 3);
}

#[test]
fn test_dot_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    let mut v2 = linear_algebra::Vector::new(3);

    v1.vector_set(vec![1.0, 2.0, 3.0]);
    v2.vector_set(vec![3.0, 4.0, 5.0]);

    let v3 = linear_algebra::Vector::vector_dot(&v1, &v2);
    assert_eq!(v3, 26.0);
}

#[test]
#[should_panic]
fn test_dot_vector_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::vector_dot(&v1, &v2);
}

#[test]
fn test_norm_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.vector_set(vec![1.0, 2.0, 2.0]);

    let v2 = linear_algebra::Vector::vector_norm(&v1);
    assert_eq!(v2, 3.0);
}

#[test]
fn test_normalize_vector() {
    let mut v1 = linear_algebra::Vector::new(3);
    v1.vector_set(vec![1.0, 2.0, 3.0]);

    let v2 = linear_algebra::Vector::vector_normalize(&v1);
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

    v1.vector_set(vec![1.0, 2.0, 3.0]);
    v2.vector_set(vec![3.0, 4.0, 5.0]);

    let v3 = linear_algebra::Vector::vector_cross(&v1, &v2);
    assert_eq!(v3.size, 3);
    assert_eq!(v3.data, vec![-2.0, 4.0, -2.0]);
    assert_eq!(v3.data.len(), 3);
}

#[test]
#[should_panic]
fn test_cross_vectors_panic() {
    let v1 = linear_algebra::Vector::new(3);
    let v2 = linear_algebra::Vector::new(4);

    let _v3 = linear_algebra::Vector::vector_cross(&v1, &v2);
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

    m.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

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
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m3 = linear_algebra::Matrix::matrix_add(&m1, &m2);
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

    let _m3 = linear_algebra::Matrix::matrix_add(&m1, &m2);
}

#[test]
fn test_sub_matrices() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m3 = linear_algebra::Matrix::matrix_sub(&m1, &m2);
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

    let _m3 = linear_algebra::Matrix::matrix_sub(&m1, &m2);
}

#[test]
fn test_scale_matrix() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m2 = linear_algebra::Matrix::matrix_scale(&m1, 2.0);

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
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![-1.0, 2.0, -3.0, 4.0, -5.0, 6.0, -7.0, 8.0, -9.0]);

    let m3 = linear_algebra::Matrix::matrix_mul(&m1, &m2);

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

    let _m3 = linear_algebra::Matrix::matrix_mul(&m1, &m2);
}

#[test]
fn test_matrix_vector_mul() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut v1 = linear_algebra::Vector::new(3);
    v1.vector_set(vec![1.0, 2.0, 3.0]);

    let m2 = linear_algebra::Matrix::matrix_vector_mul(&m1, &v1);

    assert_eq!(m2.rows, 3);
    assert_eq!(m2.cols, 1);
    assert_eq!(m2.data, vec![vec![14.0], vec![32.0], vec![50.0]]);
}

#[test]
#[should_panic]
fn test_matrix_vector_mul_panic() {
    let m1 = linear_algebra::Matrix::new(3, 3);
    let v1 = linear_algebra::Vector::new(4);

    let _m2 = linear_algebra::Matrix::matrix_vector_mul(&m1, &v1);
}

#[test]
fn test_matrix_transpose() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let m2 = linear_algebra::Matrix::matrix_transpose(&m1);

    assert_eq!(m2.rows, 3);
    assert_eq!(m2.cols, 3);
    assert_eq!(
        m2.data,
        vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 9.0]
        ]
    );

    // Test a non-square matrix
    let mut m3 = linear_algebra::Matrix::new(3, 2);
    m3.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let m4 = linear_algebra::Matrix::matrix_transpose(&m3);

    assert_eq!(m4.rows, 2);
    assert_eq!(m4.cols, 3);
    assert_eq!(m4.data, vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 6.0]]);
}

#[test]
pub fn test_matrix_fill() {
    let mut m1 = linear_algebra::Matrix::new(3, 3);

    m1.matrix_fill(1.0);

    assert_eq!(m1.rows, 3);
    assert_eq!(m1.cols, 3);
    assert_eq!(
        m1.data,
        vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0]
        ]
    );
}

#[test]
pub fn test_tensor_new() {
    let t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    assert_eq!(t1.rank, 3);
    assert_eq!(t1.rows, 3);
    assert_eq!(t1.cols, 3);

    // Check that each 3 x 3 matrix is filled with 0.0
    for i in 0..t1.rank {
        assert_eq!(
            t1.data[i].data,
            vec![
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0]
            ]
        );
    }
}

#[test]
pub fn test_tensor_set() {
    let mut t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

    let mut m3 = linear_algebra::Matrix::new(3, 3);
    m3.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mv = vec![m1, m2, m3];

    let clone = mv.clone();

    t1.tensor_set(clone);

    assert_eq!(t1.rank, 3);
    assert_eq!(t1.rows, 3);
    assert_eq!(t1.cols, 3);

    // Check that each 3 x 3 corresponds to the correct matrix

    for i in 0..t1.rank {
        assert_eq!(t1.data[i].data, mv[i].data);
    }
}

#[test]
#[should_panic]
pub fn test_tensor_set_panic() {
    let mut t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let m1 = linear_algebra::Matrix::new(3, 3);
    let m2 = linear_algebra::Matrix::new(3, 3);
    let m3 = linear_algebra::Matrix::new(3, 3);
    let m4 = linear_algebra::Matrix::new(3, 3);

    let mv = vec![m1, m2, m3, m4];

    t1.tensor_set(mv);
}

#[test]
pub fn test_tensor_insert() {
    let mut t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

    let mut m3 = linear_algebra::Matrix::new(3, 3);
    m3.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mv = vec![m1, m2, m3];

    let clone = mv.clone();

    for i in 0..t1.rank {
        t1.tensor_insert(clone[i].clone(), i);
    }

    assert_eq!(t1.rank, 3);
    assert_eq!(t1.rows, 3);
    assert_eq!(t1.cols, 3);

    // Check that each 3 x 3 corresponds to the correct matrix

    for i in 0..t1.rank {
        assert_eq!(t1.data[i].data, mv[i].data);
    }
}

#[test]
#[should_panic]
pub fn test_tensor_insert_panic() {
    let mut t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let m1 = linear_algebra::Matrix::new(3, 3);

    t1.tensor_insert(m1, 4);
}

#[test]
pub fn test_tensor_add() {
    let mut t1 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let mut m1 = linear_algebra::Matrix::new(3, 3);
    m1.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m2 = linear_algebra::Matrix::new(3, 3);
    m2.matrix_set(vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

    let mut m3 = linear_algebra::Matrix::new(3, 3);
    m3.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mv1 = vec![m1, m2, m3];

    let clone1 = mv1.clone();

    t1.tensor_set(clone1);

    let mut t2 = linear_algebra::Tensor::tensor_new(3, 3, 3);

    let mut m4 = linear_algebra::Matrix::new(3, 3);
    m4.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mut m5 = linear_algebra::Matrix::new(3, 3);
    m5.matrix_set(vec![9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);

    let mut m6 = linear_algebra::Matrix::new(3, 3);
    m6.matrix_set(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let mv2 = vec![m4, m5, m6];

    let clone2 = mv2.clone();

    t2.tensor_set(clone2);

    let expected = mv1
        .iter()
        .zip(mv2.iter())
        .map(|(a, b)| linear_algebra::Matrix::matrix_add(a, b))
        .collect::<Vec<_>>();

    let t3 = linear_algebra::Tensor::tensor_add(&t1, &t2);

    assert_eq!(t3.rank, 3);
    assert_eq!(t3.rows, 3);
    assert_eq!(t3.cols, 3);

    // Check that each 3 x 3 corresponds to the correct matrix
    for i in 0..t3.rank {
        assert_eq!(t3.data[i].data, expected[i].data);
    }
}
