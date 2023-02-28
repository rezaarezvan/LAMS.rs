// src/linear_algebra.rs

pub mod linear_algebra {

    #[derive(Debug, Clone)]
    pub struct Vector {
        pub size: usize,
        pub data: Vec<f64>,
    }

    #[derive(Debug, Clone)]
    pub struct Matrix {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Vec<f64>>,
    }

    #[derive(Debug, Clone)]
    pub struct Tensor {
        pub rank: usize,
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Matrix>,
    }

    /* Vector functions */
    impl Vector {
        pub fn new(size: usize) -> Vector {
            Vector {
                size,
                data: vec![0.0; size],
            }
        }

        pub fn vector_set(&mut self, list: Vec<f64>) {
            assert_eq!(
                self.size,
                list.len(),
                "Vector size must be equal to list size!"
            );
            self.data = list;
        }

        pub fn vector_add(v: &Vector, u: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(v.size, u.size, "Vectors must be of the same size!");

            let mut result = Vector::new(v.size);

            result.data = v
                .data
                .iter()
                .zip(u.data.iter())
                .map(|(&x, &y)| x + y)
                .collect();

            return result;
        }

        pub fn vector_sub(v: &Vector, u: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(v.size, u.size, "Vectors must be of the same size!");

            let mut result = Vector::new(v.size);

            result.data = v
                .data
                .iter()
                .zip(u.data.iter())
                .map(|(&x, &y)| x - y)
                .collect();

            return result;
        }

        pub fn vector_scale(v: &Vector, scalar: f64) -> Vector {
            let mut result = Vector::new(v.size);

            result.data = v.data.iter().map(|&x| x * scalar).collect();

            return result;
        }

        pub fn vector_dot(v1: &Vector, v2: &Vector) -> f64 {
            // Check for same size
            // If not, return an error
            assert_eq!(v1.size, v2.size, "Vectors must be of the same size!");

            let result: f64;

            result = v1
                .data
                .iter()
                .zip(v2.data.iter())
                .map(|(&x, &y)| x * y)
                .sum();

            return result;
        }

        pub fn vector_norm(v: &Vector) -> f64 {
            let result: f64;

            result = v.data.iter().map(|&x| x * x).sum();

            return result.sqrt();
        }

        pub fn vector_normalize(v: &Vector) -> Vector {
            let mut result = Vector::new(v.size);

            let norm = Vector::vector_norm(v);

            result.data = v.data.iter().map(|&x| x / norm).collect();

            return result;
        }

        pub fn vector_cross(v1: &Vector, v2: &Vector) -> Vector {
            // Check for same size
            // If not, return an error
            assert_eq!(v1.size, v2.size, "Vectors must be of the same size!");

            let mut result = Vector::new(v1.size);

            for i in 0..v1.size {
                result.data[i] = v1.data[(i + 1) % v1.size] * v2.data[(i + 2) % v1.size]
                    - v1.data[(i + 2) % v1.size] * v2.data[(i + 1) % v1.size];
            }

            return result;
        }
    }

    /* Matrix functions */
    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix {
                rows,
                cols,
                data: vec![vec![0.0; cols]; rows],
            }
        }

        pub fn matrix_set(&mut self, data: Vec<f64>) {
            assert_eq!(
                self.rows * self.cols,
                data.len(),
                "Matrix size must be equal to list size!"
            );

            data.iter()
                .enumerate()
                .for_each(|(i, &x)| self.data[i / self.cols][i % self.cols] = x);
        }

        pub fn matrix_add(a: &Matrix, b: &Matrix) -> Matrix {
            // Check so that `a` and `b` are of the same size

            assert_eq!(
                (a.rows, a.cols),
                (b.rows, b.cols),
                "Matrices must be of the same size!"
            );

            let mut result = Matrix::new(a.rows, a.cols);

            result.data = a
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(x, y)| x.iter().zip(y.iter()).map(|(&x, &y)| x + y).collect())
                .collect();

            return result;
        }

        pub fn matrix_sub(a: &Matrix, b: &Matrix) -> Matrix {
            // Check so that `a` and `b` are of the same size

            assert_eq!(
                (a.rows, a.cols),
                (b.rows, b.cols),
                "Matrices must be of the same size!"
            );

            let mut result = Matrix::new(a.rows, a.cols);

            result.data = a
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(x, y)| x.iter().zip(y.iter()).map(|(&x, &y)| x - y).collect())
                .collect();

            return result;
        }

        pub fn matrix_scale(m: &Matrix, s: f64) -> Matrix {
            let mut result = Matrix::new(m.rows, m.cols);

            result.data = m
                .data
                .iter()
                .map(|x| x.iter().map(|&x| x * s).collect())
                .collect();

            return result;
        }

        pub fn matrix_mul(a: &Matrix, b: &Matrix) -> Matrix {
            // Check so that `a` and `b` are of the same size

            assert_eq!(
                a.cols, b.rows,
                "Number of columns in `a` must be equal to number of rows in `b`!"
            );

            let mut result = Matrix::new(a.rows, b.cols);

            for i in 0..a.rows {
                for j in 0..b.cols {
                    for k in 0..a.cols {
                        result.data[i][j] += a.data[i][k] * b.data[k][j];
                    }
                }
            }

            return result;
        }

        pub fn matrix_vector_mul(m: &Matrix, v: &Vector) -> Matrix {
            assert_eq!(
                m.cols, v.size,
                "Number of columns in `m` must be equal to number of rows in `v`!"
            );

            let mut result = Matrix::new(m.rows, 1);

            for i in 0..m.rows {
                for j in 0..m.cols {
                    result.data[i][0] += m.data[i][j] * v.data[j];
                }
            }

            return result;
        }

        pub fn matrix_transpose(m: &Matrix) -> Matrix {
            let mut result = Matrix::new(m.cols, m.rows);

            for i in 0..m.rows {
                for j in 0..m.cols {
                    result.data[j][i] = m.data[i][j];
                }
            }

            return result;
        }

        pub fn matrix_fill(&mut self, value: f64) {
            self.data = self
                .data
                .iter()
                .map(|x| x.iter().map(|_| value).collect())
                .collect();
        }
    }

    impl Tensor {
        pub fn tensor_new(rank: usize, rows: usize, cols: usize) -> Tensor {
            Tensor {
                rank,
                rows,
                cols,
                data: vec![Matrix::new(rows, cols); rank],
            }
        }

        pub fn tensor_set(&mut self, data: Vec<Matrix>) {
            assert_eq!(
                self.rank,
                data.len(),
                "Tensor rank must be equal to list size!"
            );

            self.data = data;
        }

        pub fn tensor_insert(&mut self, data: Matrix, index: usize) {
            assert_eq!(
                (self.rows, self.cols),
                (data.rows, data.cols),
                "Matrix size must be equal to tensor size!"
            );

            self.data[index] = data;
        }

        pub fn tensor_add(a: &Tensor, b: &Tensor) -> Tensor {
            assert_eq!(
                (a.rank, a.rows, a.cols),
                (b.rank, b.rows, b.cols),
                "Tensors must be of the same size!"
            );

            let mut result = Tensor::tensor_new(a.rank, a.rows, a.cols);

            result.data = a
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(x, y)| Matrix::matrix_add(x, y))
                .collect();

            return result;
        }

        pub fn tensor_sub(a: &Tensor, b: &Tensor) -> Tensor {
            assert_eq!(
                (a.rank, a.rows, a.cols),
                (b.rank, b.rows, b.cols),
                "Tensors must be of the same size!"
            );

            let mut result = Tensor::tensor_new(a.rank, a.rows, a.cols);

            result.data = a
                .data
                .iter()
                .zip(b.data.iter())
                .map(|(x, y)| Matrix::matrix_sub(x, y))
                .collect();

            return result;
        }
    }
}
