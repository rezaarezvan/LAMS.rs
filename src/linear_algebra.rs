// src/linear_algebra.rs

pub mod linear_algebra {

    pub struct Vector {
        pub size: usize,
        pub data: Vec<f64>,
    }

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
    }
}
