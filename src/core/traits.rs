use thiserror::Error;

use super::meta::MarketRequest;

#[derive(Error, Debug)]
pub enum MarketRequestError {
    #[error("No registry id")]
    NoRegistryId,
    #[error("No discount curve id")]
    NoDiscountCurveId,
    #[error("No forecast curve id")]
    NoForecastCurveId,
    #[error("No discount factor request")]
    NoDiscountRequest,
    #[error("No forward rate request")]
    NoForwardRateRequest,
    #[error("No fx rate request")]
    NoFxRequest,
}

/// # Registrable
/// A trait for objects that can be registered for market data.
pub trait Registrable {
    fn registry_id(&self) -> Option<usize>;
    fn register_id(&mut self, id: usize);
    fn market_request(&self) -> Result<MarketRequest, MarketRequestError>;
}
