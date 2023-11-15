use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::Payable,
    },
    core::traits::HasCurrency,
    time::date::Date,
    utils::errors::Result, currencies::enums::Currency,
};

use super::traits::{ConstVisit, HasCashflows};

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregationConstVisitor {
    currency: Currency,
    redemptions: Arc<Mutex<BTreeMap<Date, f64>>>,
    disbursements: Arc<Mutex<BTreeMap<Date, f64>>>,
    interest: Arc<Mutex<BTreeMap<Date, f64>>>,
}

impl AggregationConstVisitor {
    pub fn new(currency: Currency) -> Self {
        Self {
            currency,
            redemptions: Arc::new(Mutex::new(BTreeMap::new())),
            disbursements: Arc::new(Mutex::new(BTreeMap::new())),
            interest: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn redemptions(&self) -> BTreeMap<Date, f64> {
        self.redemptions.lock().unwrap().clone()
    }

    pub fn disbursements(&self) -> BTreeMap<Date, f64> {
        self.disbursements.lock().unwrap().clone()
    }

    pub fn interest(&self) -> BTreeMap<Date, f64> {
        self.interest.lock().unwrap().clone()
    }
}

impl<T: HasCashflows> ConstVisit<T> for AggregationConstVisitor {
    type Output = Result<()>;

    fn visit(&self, visitable: &T) -> Self::Output {
        visitable
            .cashflows()
            .iter()
            .try_for_each(|cf| -> Result<()> {
                if self.currency != cf.currency()? {
                    return Ok(());
                }
                let side = cf.side();
                let amount = match side {
                    Side::Pay => -cf.amount()?,
                    Side::Receive => cf.amount()?,
                };
                match cf {
                    Cashflow::FixedRateCoupon(cashflow) => {
                        let mut interest = self.interest.lock().unwrap();
                        interest
                            .entry(cashflow.payment_date())
                            .and_modify(|e| *e += amount)
                            .or_insert(amount);
                    }
                    Cashflow::FloatingRateCoupon(cashflow) => {
                        let mut interest = self.interest.lock().unwrap();
                        interest
                            .entry(cashflow.payment_date())
                            .and_modify(|e| *e += amount)
                            .or_insert(amount);
                    }
                    Cashflow::Disbursement(cashflow) => {
                        let mut disbursements = self.disbursements.lock().unwrap();
                        disbursements
                            .entry(cashflow.payment_date())
                            .and_modify(|e| *e += amount)
                            .or_insert(amount);
                    }
                    Cashflow::Redemption(cashflow) => {
                        let mut redemptions = self.redemptions.lock().unwrap();
                        redemptions
                            .entry(cashflow.payment_date())
                            .and_modify(|e| *e += amount)
                            .or_insert(amount);
                    }
                }
                Ok(())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cashflows::cashflow::Side;
    use crate::currencies::enums::Currency;
    use crate::instruments::makefixedrateloan::MakeFixedRateLoan;
    use crate::rates::enums::Compounding;
    use crate::rates::interestrate::InterestRate;
    use crate::time::date::Date;
    use crate::time::daycounter::DayCounter;
    use crate::time::enums::Frequency;
    use crate::time::enums::TimeUnit;
    use crate::time::period::Period;

    use super::AggregationConstVisitor;

    #[test]
    fn test_aggregation_const_visitor() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument_1 = MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()
            .unwrap();

        let instrument_2 = MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date + Period::new(1, TimeUnit::Years))
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(200.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()
            .unwrap();

        let visitor = AggregationConstVisitor::new(Currency::USD);
        let _ = visitor.visit(&instrument_1);
        let _ = visitor.visit(&instrument_2);

        let redemptions = visitor.redemptions();
        let interest = visitor.interest();

        assert!(interest.contains_key(&end_date));
        assert!(redemptions.contains_key(&end_date));

        assert!(interest.contains_key(&end_date));

        assert_eq!(*redemptions.get(&end_date).unwrap(), 100.0);
    }
}
