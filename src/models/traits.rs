use crate::{core::meta::*, time::date::Date, utils::{errors::Result, num::Real}};

/// # Model
/// A model that provides market data based in the current market state.
pub trait Model {
    type Num: Real;
    fn reference_date(&self) -> Date;
    fn gen_df_data(&self, df: DiscountFactorRequest) -> Result<Self::Num>;
    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> Result<Self::Num>;
    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> Result<Self::Num>;
    fn gen_numerarie(&self, market_request: &MarketRequest) -> Result<Self::Num>;
    fn gen_node(&self, market_request: &MarketRequest) -> Result<MarketData<Self::Num>> {
        let id = market_request.id();
        let df = match market_request.df() {
            Some(df) => Some(self.gen_df_data(df)?),
            None => None,
        };

        let fwd = match market_request.fwd() {
            Some(fwd) => Some(self.gen_fwd_data(fwd)?),
            None => None,
        };

        let fx = match market_request.fx() {
            Some(fx) => Some(self.gen_fx_data(fx)?),
            None => None,
        };

        let numerarie = self.gen_numerarie(market_request)?;

        return Ok(MarketData::new(
            id,
            self.reference_date(),
            df,
            fwd,
            fx,
            numerarie,
        ));
    }

    fn gen_market_data(&self, market_request: &[MarketRequest]) -> Result<Vec<MarketData<Self::Num>>> {
        market_request.iter().map(|x| self.gen_node(x)).collect()
    }
}
