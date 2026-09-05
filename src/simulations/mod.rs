//! Monte Carlo simulation engine.
//!
//! Path generation and simulation infrastructure for pricing
//! path-dependent instruments.

/// Monte Carlo simulation module.
pub mod simulation;
/// Concrete generated Monte Carlo simulation.
pub mod generatedsimulation;
/// Simulation builder driven by model configurations.
pub mod simulationbuilder;
