use super::meta::MetaMarketDataNode;

/// # Registrable
/// A trait for objects that can be registered for market data.
pub trait Registrable {
    fn registry_id(&self) -> Option<usize>;
    fn register_id(&mut self, id: usize);
    fn meta_market_data(&self) -> MetaMarketDataNode;
}
