use super::meta::MarketRequest;

/// # Registrable
/// A trait for objects that can be registered for market data.
pub trait Registrable {
    fn registry_id(&self) -> Option<usize>;
    fn register_id(&mut self, id: usize);
    fn market_request(&self) -> MarketRequest;
}
