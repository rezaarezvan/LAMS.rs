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
