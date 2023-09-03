use crate::{core::meta::*, time::date::Date};

/// # Model
/// A model that provides market data based in the current market state.
///
/// ## Parameters
/// * `market_store` - The market store.
/// * `market_request` - The market data request vector.
pub trait Model {
    fn gen_df_data(&self, df: DiscountFactorRequest, eval_date: Date) -> f64;
    fn gen_fx_data(&self, fx: ExchangeRateRequest, eval_date: Date) -> f64;
    fn gen_fwd_data(&self, fwd: ForwardRateRequest, eval_date: Date) -> f64;
    fn gen_node(&self, eval_date: Date, market_request: &MarketRequest) -> MarketData {
        let id = market_request.id();
        let df = match market_request.df() {
            Some(df) => Some(self.gen_df_data(df, eval_date)),
            None => None,
        };
        let fwd = match market_request.fwd() {
            Some(fwd) => Some(self.gen_fwd_data(fwd, eval_date)),
            None => None,
        };
        let fx = match market_request.fx() {
            Some(fx) => Some(self.gen_fx_data(fx, eval_date)),
            None => None,
        };
        return MarketData::new(id, df, fwd, fx);
    }
}
