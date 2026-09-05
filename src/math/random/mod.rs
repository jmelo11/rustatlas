//! Random sampling utilities for Monte Carlo simulation.

use rand::Rng;

/// Box–Muller standard-normal sample.
pub fn std_normal(rng: &mut impl Rng) -> f64 {
    let u1: f64 = rng.gen_range(f64::EPSILON..1.0);
    let u2: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
    (-2.0 * u1.ln()).sqrt() * u2.cos()
}

/// Fills `buffer` with standard-normal draws.
pub fn fill_std_normal(rng: &mut impl Rng, buffer: &mut [f64]) {
    for value in buffer {
        *value = std_normal(rng);
    }
}
