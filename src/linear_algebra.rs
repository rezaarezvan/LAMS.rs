// src/linear_algebra.rs

pub mod linear_algebra {

    pub struct Vector {
        pub size: usize,
        pub data: Vec<f64>,
    }

    pub struct Matrix {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Vec<f64>>,
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

        pub fn vector_add(&self, other: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(self.size, other.size, "Vectors must be of the same size!");

            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] + other.data[i];
            }

            return result;
        }

        pub fn vector_sub(&self, other: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(self.size, other.size, "Vectors must be of the same size!");

            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] - other.data[i];
            }

            return result;
        }

        pub fn vector_scale(&self, scalar: f64) -> Vector {
            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] * scalar;
            }

            return result;
        }

        pub fn vector_dot(v1: &Vector, v2: &Vector) -> f64 {
            // Check for same size
            // If not, return an error
            assert_eq!(v1.size, v2.size, "Vectors must be of the same size!");

            let mut result: f64 = 0.0;

            for i in 0..v1.size {
                result += v1.data[i] * v2.data[i];
            }

            return result;
        }

        pub fn vector_norm(v: &Vector) -> f64 {
            let mut result: f64 = 0.0;

            for i in 0..v.size {
                result += v.data[i] * v.data[i];
            }

            return result.sqrt();
        }

        pub fn vector_normalize(v: &Vector) -> Vector {
            let mut result = Vector::new(v.size);

            let norm = Vector::vector_norm(v);

            for i in 0..v.size {
                result.data[i] = v.data[i] / norm;
            }

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

        pub fn set(&mut self, data: Vec<f64>) {
            assert_eq!(
                self.rows * self.cols,
                data.len(),
                "Matrix size must be equal to list size!"
            );

            for i in 0..data.len() {
                let row = i / self.cols;
                let col = i % self.cols;

                self.data[row][col] = data[i];
            }
        }

        pub fn add(a: &Matrix, b: &Matrix) -> Matrix {
            // Check so that `a` and `b` are of the same size

            assert_eq!(
                (a.rows, a.cols),
                (b.rows, b.cols),
                "Matrices must be of the same size!"
            );

            let mut result = Matrix::new(a.rows, a.cols);

            for i in 0..a.rows {
                for j in 0..a.cols {
                    result.data[i][j] = a.data[i][j] + b.data[i][j];
                }
            }

            return result;
        }

        pub fn sub(a: &Matrix, b: &Matrix) -> Matrix {
            // Check so that `a` and `b` are of the same size

            assert_eq!(
                (a.rows, a.cols),
                (b.rows, b.cols),
                "Matrices must be of the same size!"
            );

            let mut result = Matrix::new(a.rows, a.cols);

            for i in 0..a.rows {
                for j in 0..a.cols {
                    result.data[i][j] = a.data[i][j] - b.data[i][j];
                }
            }

            return result;
        }

        pub fn scale(m: &Matrix, s: f64) -> Matrix {
            let mut result = Matrix::new(m.rows, m.cols);

            for i in 0..m.rows {
                for j in 0..m.cols {
                    result.data[i][j] = m.data[i][j] * s;
                }
            }

            return result;
        }

        pub fn mul(a: &Matrix, b: &Matrix) -> Matrix {
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

        pub fn matrix_fill(&mut self, value: f64) -> () {
            for i in 0..self.rows {
                for j in 0..self.cols {
                    self.data[i][j] = value;
                }
            }
        }
    }
}
