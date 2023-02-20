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

        pub fn set(&mut self, list: Vec<f64>) {
            assert_eq!(
                self.size,
                list.len(),
                "Vector size must be equal to list size!"
            );
            self.data = list;
        }

        pub fn add(&self, other: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(self.size, other.size, "Vectors must be of the same size!");

            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] + other.data[i];
            }

            return result;
        }

        pub fn sub(&self, other: &Vector) -> Vector {
            // Check so that the vectors are of the same size
            // If not, return an error
            assert_eq!(self.size, other.size, "Vectors must be of the same size!");

            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] - other.data[i];
            }

            return result;
        }

        pub fn scale(&self, scalar: f64) -> Vector {
            let mut result = Vector::new(self.size);

            for i in 0..self.size {
                result.data[i] = self.data[i] * scalar;
            }

            return result;
        }

        pub fn dot(v1: &Vector, v2: &Vector) -> f64 {
            // Check for same size
            // If not, return an error
            assert_eq!(v1.size, v2.size, "Vectors must be of the same size!");

            let mut result: f64 = 0.0;

            for i in 0..v1.size {
                result += v1.data[i] * v2.data[i];
            }

            return result;
        }

        pub fn norm(v: &Vector) -> f64 {
            let mut result: f64 = 0.0;

            for i in 0..v.size {
                result += v.data[i] * v.data[i];
            }

            return result.sqrt();
        }

        pub fn normalize(v: &Vector) -> Vector {
            let mut result = Vector::new(v.size);

            let norm = Vector::norm(v);

            for i in 0..v.size {
                result.data[i] = v.data[i] / norm;
            }

            return result;
        }

        pub fn cross(v1: &Vector, v2: &Vector) -> Vector {
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
    }
}
