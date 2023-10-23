use super::meta::MarketRequest;
use crate::{currencies::enums::Currency, utils::errors::Result};

pub trait HasCurrency {
    fn currency(&self) -> Result<Currency>;
}

pub trait HasDiscountCurveId {
    fn discount_curve_id(&self) -> Result<usize>;
}

pub trait HasForecastCurveId {
    fn forecast_curve_id(&self) -> Result<usize>;
}

/// # Registrable
/// A trait for objects that can be registered for market data.
pub trait Registrable: HasDiscountCurveId + HasForecastCurveId + HasCurrency {
    fn id(&self) -> Result<usize>;
    fn set_id(&mut self, id: usize);
    fn market_request(&self) -> Result<MarketRequest>;
}
