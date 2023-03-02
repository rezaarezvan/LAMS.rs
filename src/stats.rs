// src/stats.rs

pub mod stats {

    pub struct Bernoulli {
        pub p: f64,
    }

    pub struct Binomial {
        pub n: u32,
        pub p: f64,
    }

    pub struct DiscreteUniform {
        pub a: u32,
        pub b: u32,
    }

    pub struct Geometric {
        pub p: f64,
    }

    pub struct HyperGeometric {
        pub n: u32,
        pub m: u32,
        pub k: u32,
    }

    pub struct NegativeBinomial {
        pub r: u32,
        pub p: f64,
    }

    pub struct NegativeHyperGeometric {
        pub n: u32,
        pub k: u32,
        pub r: u32,
    }

    pub struct Poisson {
        pub lambda: f64,
    }

    pub struct ContinuousUniform {
        pub a: f64,
        pub b: f64,
    }

    pub struct Normal {
        pub mu: f64,
        pub sigma: f64,
    }

    pub struct Exponential {
        pub lambda: f64,
    }

    pub struct Beta {
        pub alpha: f64,
        pub beta: f64,
    }

    pub struct ChiSquared {
        pub k: f64,
    }

    pub struct F {
        pub d1: f64,
        pub d2: f64,
    }

    pub struct Gamma {
        // Shape - alpha = k
        pub alpha: f64,
        // Scale - beta = 1 / theta
        pub beta: f64,
    }

    pub struct Laplace {
        pub mu: f64,
        pub b: f64,
    }

    pub struct T {
        pub nu: f64,
    }

    // Auxiliary function to calculate the binomial coefficient

    pub fn binomial_coefficient(n: u32, k: u32) -> u32 {
        let mut result = 1;

        for i in 1..=k {
            result *= n - k + i;
            result /= i;
        }

        return result;
    }

    impl Bernoulli {
        pub fn new(p: f64) -> Bernoulli {
            assert!(p >= 0.0 && p <= 1.0);

            Bernoulli { p }
        }

        pub fn pmf(&self, k: u32) -> f64 {
            match k {
                0 => 1.0 - self.p,
                1 => self.p,
                _ => 0.0,
            }
        }

        pub fn cdf(&self, k: u32) -> f64 {
            if k < 0 {
                0.0
            } else if k < 1 {
                1.0 - self.p
            } else {
                1.0
            }
        }

        pub fn mean(&self) -> f64 {
            self.p
        }

        pub fn variance(&self) -> f64 {
            self.p * (1.0 - self.p)
        }

        pub fn skewness(&self) -> f64 {
            (1.0 - self.p - self.p) / ((self.p * (1.0 - self.p)).sqrt())
        }
    }
}
