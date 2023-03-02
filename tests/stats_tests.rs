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
