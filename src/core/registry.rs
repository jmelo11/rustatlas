use super::meta::MetaMarketData;

pub trait Registrable {
    fn registry_id(&self) -> Option<usize>;
    fn register_id(&mut self, id: usize);
    fn meta_market_data(&self) -> MetaMarketData;
}
