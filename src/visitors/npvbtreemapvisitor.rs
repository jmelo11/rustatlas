use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::{meta::MarketData, traits::Registrable},
    utils::errors::{AtlasError, Result},
    time::date::Date,
};

use std::collections::BTreeMap;
use super::traits::{ConstVisit, HasCashflows};

/// # NPVConstVisitor
/// NPVConstVisitor is a visitor that calculates the NPV of an instrument.
/// It assumes that the cashflows of the instrument have already been indexed and fixed.
pub struct NPVBTreeMapVisitor<'a> {
    market_data: &'a [MarketData],
    include_today_cashflows: bool,
}

impl<'a> NPVBTreeMapVisitor<'a> {
    pub fn new(market_data: &'a [MarketData], include_today_cashflows: bool) -> Self {
        NPVBTreeMapVisitor {
            market_data: market_data,
            include_today_cashflows,
        }
    }
    pub fn set_include_today_cashflows(&mut self, include_today_cashflows: bool) {
        self.include_today_cashflows = include_today_cashflows;
    }
}

impl<'a, T: HasCashflows> ConstVisit<T> for NPVBTreeMapVisitor<'a> {
    type Output = Result<BTreeMap<Date, f64>>;
    fn visit(&self, visitable: &T) -> Self::Output {
        let npv_result = visitable.cashflows().iter().try_fold(BTreeMap::new(), |mut acc, cf| {
            let id = cf.id()?;

            let cf_market_data =
                self.market_data
                    .get(id)
                    .ok_or(AtlasError::NotFoundErr(format!(
                        "Market data for cashflow with id {}",
                        id
                    )))?;

            if cf_market_data.reference_date() == cf.payment_date() && !self.include_today_cashflows
                || cf.payment_date() < cf_market_data.reference_date()
            {
                return Ok(acc);
            }

            let df = cf_market_data.df()?;
            let fx = cf_market_data.fx()?;
            let flag = match cf.side() {
                Side::Pay => -1.0,
                Side::Receive => 1.0,
            };
            let amount = cf.amount()?;
            acc.insert(cf.payment_date(), df * amount / fx * flag);
            Ok(acc)
        });
        
        return npv_result
    }
}

#[cfg(test)]
mod tests {

    
}
