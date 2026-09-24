//! Monte Carlo sampler contract (cargo-only: feature `mc`).
#![cfg(feature = "mc")]

use singularity_econ::sampler::{sampled_values, Sampler};

// Every parameter `sampled_values` reports must actually vary across draws —
// otherwise `sa` would rank a constant, or the list has drifted from the draw
// code in `Sampler::params`.
#[test]
fn every_listed_parameter_is_actually_sampled() {
    let mut s = Sampler::new(3);
    let draws: Vec<_> = (0..400).map(|_| sampled_values(&s.params())).collect();
    for (i, (name, _)) in draws[0].iter().enumerate() {
        let first = draws[0][i].1;
        assert!(
            draws.iter().any(|d| (d[i].1 - first).abs() > 1e-12),
            "{name} is listed as sampled but never varies across 400 draws"
        );
    }
}

// Seeded determinism: the same seed must reproduce the same draw sequence.
#[test]
fn sampler_is_seed_deterministic() {
    let a: Vec<_> = (0..20).map({ let mut s = Sampler::new(9); move |_| sampled_values(&s.params()) }).collect();
    let b: Vec<_> = (0..20).map({ let mut s = Sampler::new(9); move |_| sampled_values(&s.params()) }).collect();
    assert_eq!(a, b);
}
