use crate::core::meta::*;

/// # Model
/// A model that provides market data based in the current market state.
pub trait Model {
    fn gen_df_data(&self, df: DiscountFactorRequest) -> f64;
    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> f64;
    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> f64;
    fn gen_node(&self, market_request: &MarketRequest) -> MarketData {
        let id = market_request.id();
        let df = match market_request.df() {
            Some(df) => Some(self.gen_df_data(df)),
            None => None,
        };
        let fwd = match market_request.fwd() {
            Some(fwd) => Some(self.gen_fwd_data(fwd)),
            None => None,
        };
        let fx = match market_request.fx() {
            Some(fx) => Some(self.gen_fx_data(fx)),
            None => None,
        };
        return MarketData::new(id, df, fwd, fx);
    }

    fn gen_market_data(&self, market_request: &[MarketRequest]) -> Vec<MarketData> {
        market_request.iter().map(|x| self.gen_node(x)).collect()
    }
}
