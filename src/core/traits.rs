use super::meta::MarketRequest;
use crate::{currencies::enums::Currency, utils::errors::Result};

/// A trait for objects that have a currency.
pub trait HasCurrency {
    /// Returns the currency of the object.
    fn currency(&self) -> Result<Currency>;
}

/// A trait for objects that have a discount curve ID.
pub trait HasDiscountCurveId {
    /// Returns the discount curve ID of the object.
    fn discount_curve_id(&self) -> Result<usize>;
}

/// A trait for objects that have a forecast curve ID.
pub trait HasForecastCurveId {
    /// Returns the forecast curve ID of the object.
    fn forecast_curve_id(&self) -> Result<usize>;
}

/// A trait for objects that can be registered for market data.
pub trait Registrable: HasDiscountCurveId + HasForecastCurveId + HasCurrency {
    /// Returns the ID of the object.
    fn id(&self) -> Result<usize>;
    /// Sets the ID of the object.
    fn set_id(&mut self, id: usize);
    /// Returns the market request for the object.
    fn market_request(&self) -> Result<MarketRequest>;
}
