use crate::core::meta::MarketData;

/// # MonteCarloModel
/// A Monte Carlo model that simulates a path of market data.
pub trait MonteCarloEngine {
    fn simulate_path(&self) -> Option<Vec<MarketData>>;
}
