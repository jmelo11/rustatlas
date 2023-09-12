use thiserror::Error;

use crate::{cashflows::cashflow::Cashflow, core::traits::MarketRequestError};

pub trait Visit<T> {
    type Output;
    fn visit(&self, visitable: &mut T) -> Self::Output;
}

pub trait ConstVisit<T> {
    type Output;
    fn visit(&self, visitable: &T) -> Self::Output;
}

pub trait HasCashflows {
    fn cashflows(&self) -> &[Cashflow];
    fn mut_cashflows(&mut self) -> &mut [Cashflow];
    fn set_discount_curve_id(&mut self, id: Option<usize>) {
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_discount_curve_id(id));
    }
    fn set_forecast_curve_id(&mut self, id: Option<usize>) {
        self.mut_cashflows().iter_mut().for_each(|cf| match cf {
            Cashflow::FloatingRateCoupon(frcf) => frcf.set_forecast_curve_id(id),
            _ => (),
        });
    }
}

#[derive(Error, Debug)]
pub enum EvaluationError {
    #[error("No registry id")]
    NoRegistryId,
    #[error("No discount factor")]
    NoDiscountFactor,
    #[error("No forward rate")]
    NoForwardRate,
    #[error("No exchange rate")]
    NoExchangeRate,
    #[error("No amount set")]
    NoAmount,
    #[error("No fixing rate set")]
    NoFixingRate,
    #[error("No convergence")]
    NoConvergence,
    #[error("Market data error: {0}")]
    MarketDataError(#[from] MarketRequestError),
    #[error("No market data found")]
    NoMarketData,
}
