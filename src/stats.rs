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
        pub N: u32,
        pub K: u32,
        pub n: u32,
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

    impl Binomial {
        pub fn new(n: u32, p: f64) -> Binomial {
            assert!(p >= 0.0 && p <= 1.0);

            Binomial { n, p }
        }

        pub fn pmf(&self, k: u32) -> f64 {
            if k < 0 || k > self.n {
                0.0
            } else {
                binomial_coefficient(self.n, k) as f64
                    * self.p.powi(k as i32)
                    * (1.0 - self.p).powi((self.n - k) as i32)
            }
        }

        pub fn cdf(&self, k: u32) -> f64 {
            if k < 0 {
                0.0
            } else if k < self.n {
                let mut result = 0.0;

                for i in 0..=k {
                    result += self.pmf(i);
                }

                result
            } else {
                1.0
            }
        }

        pub fn mean(&self) -> f64 {
            self.n as f64 * self.p
        }

        pub fn variance(&self) -> f64 {
            self.n as f64 * self.p * (1.0 - self.p)
        }

        pub fn skewness(&self) -> f64 {
            ((1.0 - self.p) - self.p) / ((self.n as f64 * self.p * (1.0 - self.p)).sqrt())
        }
    }

    impl DiscreteUniform {
        pub fn new(a: u32, b: u32) -> DiscreteUniform {
            assert!(a <= b);

            DiscreteUniform { a, b }
        }

        pub fn pmf(&self, k: u32) -> f64 {
            if k < self.a || k > self.b {
                0.0
            } else {
                1.0 / (self.b - self.a + 1) as f64
            }
        }

        pub fn cdf(&self, k: u32) -> f64 {
            if k < self.a {
                0.0
            } else if k <= self.b {
                (k - self.a) as f64 / (self.b - self.a) as f64
            } else {
                1.0
            }
        }

        pub fn mean(&self) -> f64 {
            (self.a + self.b) as f64 / 2.0
        }

        pub fn variance(&self) -> f64 {
            let n = self.b - self.a + 1;
            (n * n - 1) as f64 / 12.0
        }

        pub fn skewness(&self) -> f64 {
            0.0
        }
    }

    impl Geometric {
        pub fn new(p: f64) -> Geometric {
            assert!(p >= 0.0 && p <= 1.0);

            Geometric { p }
        }

        pub fn pmf(&self, k: u32) -> f64 {
            self.p * (1.0 - self.p).powi(k as i32)
        }

        pub fn cdf(&self, k: u32) -> f64 {
            1.0 - (1.0 - self.p).powi((k + 1) as i32)
        }

        pub fn mean(self) -> f64 {
            1.0 / self.p
        }

        pub fn variance(self) -> f64 {
            (1.0 - self.p) / (self.p * self.p)
        }

        pub fn skewness(self) -> f64 {
            (2.0 - self.p) / ((1.0 - self.p).sqrt())
        }
    }

    impl HyperGeometric {
        pub fn new(N: u32, K: u32, n: u32) -> HyperGeometric {
            assert!(K <= N && n <= N);

            HyperGeometric { N, K, n }
        }
    }
}
