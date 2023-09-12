use thiserror::Error;

use crate::{
    core::{meta::*, traits::MarketRequestError},
    rates::traits::YieldProviderError,
};

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("YieldProviderError {0}")]
    YieldProviderError(#[from] YieldProviderError),
    #[error("MarketRequestError {0}")]
    MarketDataError(#[from] MarketRequestError),
    #[error("No curve found for id {0}")]
    NoCurveFound(String),
    #[error("No fx rate found for ccy pair {0} {1}")]
    NoFxRateFound(String, String),
    #[error("No curve found for ccy {0}")]
    NoCurveFoundForCcy(String),
}

/// # Model
/// A model that provides market data based in the current market state.
pub trait Model {
    fn gen_df_data(&self, df: DiscountFactorRequest) -> Result<f64, ModelError>;
    fn gen_fx_data(&self, fx: ExchangeRateRequest) -> Result<f64, ModelError>;
    fn gen_fwd_data(&self, fwd: ForwardRateRequest) -> Result<f64, ModelError>;
    fn gen_node(&self, market_request: &MarketRequest) -> Result<MarketData, ModelError> {
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
        return Ok(MarketData::new(id, df, fwd, fx));
    }

    fn gen_market_data(
        &self,
        market_request: &[MarketRequest],
    ) -> Result<Vec<MarketData>, ModelError> {
        market_request.iter().map(|x| self.gen_node(x)).collect()
    }
}
