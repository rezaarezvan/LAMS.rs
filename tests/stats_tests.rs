// test/stats_tests.rs

use LAMS_rs::stats::*;

#[test]
pub fn test_bernoulli_new() {
    let b = stats::Bernoulli::new(0.5);
    assert_eq!(b.p, 0.5);
}

#[test]
#[should_panic]
pub fn test_bernoulli_new_panic() {
    let b = stats::Bernoulli::new(1.5);
}

#[test]
pub fn test_bernoulli_pmf() {
    let b = stats::Bernoulli::new(0.3);

    assert_eq!(b.pmf(0), 0.7);

    assert_eq!(b.pmf(1), 0.3);

    assert_eq!(b.pmf(2), 0.0);
}

#[test]
pub fn test_bernoulli_cdf() {
    let b = stats::Bernoulli::new(0.3);

    assert_eq!(b.cdf(0), 0.7);

    assert_eq!(b.cdf(1), 1.0);

    assert_eq!(b.cdf(2), 1.0);
}

#[test]
pub fn test_bernoulli_mean() {
    let b = stats::Bernoulli::new(0.3);

    assert_eq!(b.mean(), 0.3);
}

#[test]
pub fn test_bernoulli_variance() {
    let b = stats::Bernoulli::new(0.3);

    let expected_variance = 0.3 * 0.7;

    assert_eq!(b.variance(), expected_variance);
}

#[test]
pub fn test_bernoulli_skewness() {
    let b = stats::Bernoulli::new(0.3);

    let expected_skewness = ((1.0 - b.p) - b.p) / ((b.p * (1.0 - b.p)).sqrt());

    assert_eq!(b.skewness(), expected_skewness);
}

#[test]
pub fn test_binomial_new() {
    let b = stats::Binomial::new(10, 0.5);
    assert_eq!(b.n, 10);
    assert_eq!(b.p, 0.5);
}

#[test]
#[should_panic]
pub fn test_binomial_new_panic() {
    let b = stats::Binomial::new(10, 1.5);
}

#[test]
pub fn test_binomial_coefficient() {
    assert_eq!(stats::binomial_coefficient(5, 2), 10);
    assert_eq!(stats::binomial_coefficient(5, 3), 10);
    assert_eq!(stats::binomial_coefficient(5, 4), 5);
    assert_eq!(stats::binomial_coefficient(5, 5), 1);
}

#[test]
pub fn test_binomial_pmf() {
    let b = stats::Binomial::new(10, 0.3);

    let mut expected: Vec<f64> = Vec::new();

    let mut sum = 0.0;

    for k in 0..=b.n {
        expected.push(
            stats::binomial_coefficient(b.n, k) as f64
                * b.p.powi(k as i32)
                * (1.0 - b.p).powi((b.n - k) as i32),
        );
    }

    for k in 0..=b.n {
        println!("sum: {}", sum);
        sum += b.pmf(k);
        assert_eq!(b.pmf(k), expected[k as usize]);
    }
    assert_eq!(sum.round(), 1.0);
}

#[test]
pub fn test_binomial_cdf() {
    let b = stats::Binomial::new(10, 0.3);

    let mut expected: Vec<f64> = Vec::new();

    for k in 0..=b.n {
        expected.push(b.pmf(k));
        for i in 0..k {
            expected[k as usize] += b.pmf(i);
        }
    }

    let epsilon = 0.00000000001;

    for k in 0..=b.n {
        let error = (b.cdf(k) - expected[k as usize]).abs();
        assert!(error < epsilon);
    }
}

#[test]
pub fn test_binomial_mean() {
    let b = stats::Binomial::new(10, 0.3);

    let expected_mean = b.n as f64 * b.p;

    assert_eq!(b.mean(), expected_mean);
}

#[test]
pub fn test_binomial_variance() {
    let b = stats::Binomial::new(10, 0.3);

    let expected_variance = b.n as f64 * b.p * (1.0 - b.p);

    assert_eq!(b.variance(), expected_variance);
}

#[test]
pub fn test_binomial_skewness() {
    let b = stats::Binomial::new(10, 0.3);

    let expected_skewness = (1.0 - 2.0 * b.p) / (b.n as f64 * b.p * (1.0 - b.p)).sqrt();

    let epsilon = 0.00000000001;
    let error = (b.skewness() - expected_skewness).abs();
    assert!(error < epsilon);
}

#[test]
pub fn test_discreteuniform_new() {
    let d = stats::DiscreteUniform::new(0, 10);
    assert_eq!(d.a, 0);
    assert_eq!(d.b, 10);
}

#[test]
#[should_panic]
pub fn test_discreteuniform_new_panic() {
    let d = stats::DiscreteUniform::new(10, 0);
}

#[test]
pub fn test_discreteuniform_pmf() {
    let d = stats::DiscreteUniform::new(0, 10);

    let expected_pmf = 1.0 / (d.b - d.a + 1) as f64;

    for k in d.a..=d.b {
        assert_eq!(d.pmf(k), expected_pmf);
    }
}

#[test]
pub fn test_discreteuniform_pmf_panic() {
    let d = stats::DiscreteUniform::new(0, 10);
    let expected_pmf = 0.0;

    assert_eq!(d.pmf(11), expected_pmf);
}

#[test]
pub fn test_discreteuniform_cdf() {
    let d = stats::DiscreteUniform::new(0, 10);

    for k in d.a..=d.b {
        let expected_cdf = (k - d.a) as f64 / (d.b - d.a) as f64;
        assert_eq!(d.cdf(k), expected_cdf);
    }
}

#[test]
pub fn test_discreteuniform_mean() {
    let d = stats::DiscreteUniform::new(0, 10);

    let expected_mean = (d.a + d.b) as f64 / 2.0;

    assert_eq!(d.mean(), expected_mean);
}

#[test]
pub fn test_discreteuniform_variance() {
    let d = stats::DiscreteUniform::new(0, 10);
    let n = d.b - d.a + 1;
    let expected_variance = (n * n - 1) as f64 / 12.0;

    assert_eq!(d.variance(), expected_variance);
}

#[test]
pub fn test_discreteuniform_skewness() {
    let d = stats::DiscreteUniform::new(0, 10);

    assert_eq!(d.skewness(), 0.0);
}

#[test]
pub fn test_geometric_new() {
    let g = stats::Geometric::new(0.5);
    assert_eq!(g.p, 0.5);
}

#[test]
#[should_panic]
pub fn test_geometric_new_panic() {
    let g = stats::Geometric::new(1.5);
}

#[test]
pub fn test_geometric_pmf() {
    let g = stats::Geometric::new(0.5);
    let n = 10;

    for k in 0..=n {
        let expected_pmf = g.p * (1.0 - g.p).powi(k as i32);
        assert_eq!(g.pmf(k), expected_pmf);
    }
}

#[test]
pub fn test_geometric_cdf() {
    let g = stats::Geometric::new(0.5);
    let n = 10;

    let expected_cdf = 1.0 - (1.0 - g.p).powi(n as i32 + 1);

    assert_eq!(g.cdf(n), expected_cdf);

    assert_eq!(g.cdf(0), 0.5);
}

#[test]
pub fn test_geometric_mean() {
    let g = stats::Geometric::new(0.5);

    let expected_mean = 1.0 / g.p;

    assert_eq!(g.mean(), expected_mean);
}

#[test]
pub fn test_geometric_variance() {
    let g = stats::Geometric::new(0.5);

    let expected_variance = (1.0 - g.p) / (g.p * g.p);

    assert_eq!(g.variance(), expected_variance);
}

#[test]
pub fn test_geometric_skewness() {
    let g = stats::Geometric::new(0.5);

    let expected_skewness = (2.0 - g.p) / (1.0 - g.p).sqrt();

    let epsilon = 0.00000000001;
    let error = (g.skewness() - expected_skewness).abs();
    assert!(error < epsilon);
}
