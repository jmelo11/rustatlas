use std::collections::BTreeMap;

use crate::{
    core::traits::HasCurrency,
    currencies::enums::Currency,
    time::{date::Date, period::Period},
    utils::errors::Result,
};

use super::traits::{ConstVisit, HasCashflows};

pub struct AccruedAmountConstVisitor {
    accrued_amounts: BTreeMap<Date, f64>,
    validation_currency: Option<Currency>,
    evaluation_date: Date,
    horizon: Period,
}

impl AccruedAmountConstVisitor {
    pub fn new(evaluation_date: Date, horizon: Period) -> Self {
        Self {
            accrued_amounts: BTreeMap::new(),
            validation_currency: None,
            evaluation_date,
            horizon: horizon,
        }
    }

    pub fn with_validate_currency(mut self, currency: Currency) -> Self {
        self.validation_currency = Some(currency);
        self
    }

    pub fn accrued_amounts(&self) -> BTreeMap<Date, f64> {
        self.accrued_amounts.clone()
    }
}

impl<T: HasCashflows + HasCurrency> ConstVisit<T> for AccruedAmountConstVisitor {
    type Output = Result<()>;

    fn visit(&self, _: &T) -> Self::Output {
        Ok(())
    }
}
