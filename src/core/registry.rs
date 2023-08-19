use super::meta::MetaMarketData;

pub trait Registrable {
    fn registry_id(&self) -> Option<u64>;
    fn register_id(&mut self, id: u64);
    fn meta_market_data(&self) -> MetaMarketData;
}
