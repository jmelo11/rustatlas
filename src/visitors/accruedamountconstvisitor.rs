use std::{collections::BTreeMap, sync::Mutex};

use crate::{
    cashflows::{cashflow::Cashflow, traits::InterestAccrual},
    core::traits::HasCurrency,
    currencies::enums::Currency,
    time::{date::Date, enums::TimeUnit, period::Period, schedule::MakeSchedule},
    utils::errors::{AtlasError, Result},
};

use super::traits::{ConstVisit, HasCashflows};

/// # AccruedAmountConstVisitor
/// Visitor for calculating accrued amounts.
///
/// ## Parameters
/// * `evaluation_date` - The evaluation date
/// * `horizon` - The horizon of the calculation
/// * `validation_currency` - Flag to validate the currency of the instrument against the provided currency
pub struct AccruedAmountConstVisitor {
    accrued_amounts: Mutex<BTreeMap<Date, f64>>,
    validation_currency: Option<Currency>,
    evaluation_dates: Vec<Date>,
}

impl AccruedAmountConstVisitor {
    pub fn new(evaluation_date: Date, horizon: Period) -> Self {
        let schedule = MakeSchedule::new(evaluation_date, evaluation_date + horizon)
            .with_tenor(Period::new(1, TimeUnit::Days))
            .build()
            .unwrap();

        Self {
            accrued_amounts: Mutex::new(BTreeMap::new()),
            validation_currency: None,
            evaluation_dates: schedule.dates().clone(),
        }
    }

    pub fn with_validate_currency(mut self, currency: Currency) -> Self {
        self.validation_currency = Some(currency);
        self
    }

    pub fn accrued_amounts(&self) -> BTreeMap<Date, f64> {
        self.accrued_amounts.lock().unwrap().clone()
    }
}

impl<T: HasCurrency + HasCashflows> ConstVisit<T> for AccruedAmountConstVisitor {
    type Output = Result<()>;

    fn visit(&self, inst: &T) -> Self::Output {
        match self.validation_currency {
            Some(currency) => {
                if inst.currency()? != currency {
                    return Err(AtlasError::InvalidValueErr("Currency mismatch".to_string()));
                }
            }
            None => {}
        }
        self.evaluation_dates
            .windows(2)
            .try_for_each(|dates| -> Result<()> {
                let start_date = dates[0];
                let end_date = dates[1];
                let accrued_amount = inst
                    .cashflows()
                    .iter()
                    .filter(|cf| match cf {
                        Cashflow::FixedRateCoupon(_) | Cashflow::FloatingRateCoupon(_) => {
                            cf.accrual_start_date().unwrap() <= end_date
                                && cf.accrual_end_date().unwrap() >= start_date
                        }
                        _ => false,
                    })
                    .map(|cf| cf.accrued_amount(start_date, end_date).unwrap())
                    .sum();

                self.accrued_amounts
                    .lock()
                    .unwrap()
                    .entry(end_date)
                    .and_modify(|e| *e += accrued_amount)
                    .or_insert(accrued_amount);
                Ok(())
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cashflows::cashflow::Side,
        instruments::makefixedrateinstrument::MakeFixedRateInstrument,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::{daycounter::DayCounter, enums::Frequency},
    };

    use super::*;

    #[test]
    fn test_accrued_amount_const_visitor() -> Result<()> {
        let start_date = Date::new(2024, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.06,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        let yield_rate = InterestRate::new(
            0.07,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Thirty360,
        );

        let instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(5000000.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .with_yield_rate(yield_rate)
            .bullet()
            .build()?;

        let visitor = AccruedAmountConstVisitor::new(start_date, Period::new(5, TimeUnit::Years))
            .with_validate_currency(Currency::USD);

        visitor.visit(&instrument)?;
        let accrued_amounts = visitor.accrued_amounts();
        let size = start_date + Period::new(5, TimeUnit::Years) - start_date;
        assert_eq!(accrued_amounts.len(), size as usize);

        Ok(())
    }
}
