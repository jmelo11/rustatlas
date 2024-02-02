use std::{cell::RefCell, collections::BTreeMap};

use crate::{
    cashflows::{cashflow::Side, traits::Payable},
    core::traits::HasCurrency,
    currencies::enums::Currency,
    time::date::Date,
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};

/// # CashAccount
/// Struct that defines a cash account. It is used to keep track of the cash inflows and outflows
/// of an account.
pub struct CashAccount {
    currency: Option<Currency>,
    pub amount: RefCell<BTreeMap<Date, f64>>,
}

impl HasCurrency for CashAccount {
    fn currency(&self) -> Result<Currency> {
        self.currency
            .ok_or(AtlasError::InvalidValueErr("Currency not set".to_string()))
    }
}

impl CashAccount {
    pub fn new() -> Self {
        Self {
            amount: RefCell::new(BTreeMap::new()),
            currency: None,
        }
    }

    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn add_flows_from_instrument(&self, instrument: &dyn HasCashflows) -> Result<()> {
        let account_currency = self.currency()?;
        instrument
            .cashflows()
            .iter()
            .try_for_each(|cf| -> Result<()> {
                let date = cf.payment_date();
                let side = cf.side();
                let amount = match side {
                    Side::Pay => -cf.amount()?,
                    Side::Receive => cf.amount()?,
                };
                let currency = cf.currency()?;
                if currency == account_currency {
                    let mut amount_map = self.amount.borrow_mut();
                    let entry = amount_map.entry(date).or_insert(0.0);
                    *entry += amount;
                }
                Ok(())
            })?;
        Ok(())
    }

    pub fn add_flows_from_map(&self, map: &BTreeMap<Date, f64>) -> Result<()> {
        let mut amount_map = self.amount.borrow_mut();
        for (date, amount) in map {
            let entry = amount_map.entry(*date).or_insert(0.0);
            *entry += amount;
        }
        Ok(())
    }

    pub fn add_flows_from_new_position(&self, date: Date, value: f64) -> Result<()> {
        let mut amount_map = self.amount.borrow_mut();
        let entry = amount_map.entry(date).or_insert(0.0);
        *entry += value;
        Ok(())
    }

    pub fn add_flows_from_cash_account(&self, cash_account: &CashAccount) -> Result<()> {
        let amount_map = cash_account.amount.borrow();
        if self.currency != cash_account.currency {
            return Err(AtlasError::InvalidValueErr(
                "Currencies do not match".to_string(),
            ));
        }
        self.add_flows_from_map(&amount_map)
    }

    pub fn cash_account_evolution(&self, evals_dates: Vec<Date>) -> Result<BTreeMap<Date, f64>> {
        let amount_map = self.amount.borrow();
        let mut dates = amount_map.keys().cloned().collect::<Vec<Date>>();
        dates.sort();
        let mut cash_account = BTreeMap::new();
        let mut amount = 0.0;
        for date in evals_dates {
            amount += amount_map.get(&date).unwrap_or(&0.0);
            cash_account.insert(date, amount);
        }
        Ok(cash_account)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruments::makefixedrateinstrument::MakeFixedRateInstrument,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::{
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
    };

    use super::*;

    #[test]
    fn test_single_instrument() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()?;

        let cash_account = CashAccount::new().with_currency(Currency::USD);
        cash_account.add_flows_from_instrument(&instrument)?;

        let evals_dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 7, 1),
            Date::new(2021, 1, 1),
            Date::new(2021, 7, 1),
            Date::new(2022, 1, 1),
            Date::new(2022, 7, 1),
            Date::new(2023, 1, 1),
            Date::new(2023, 7, 1),
            Date::new(2024, 1, 1),
            Date::new(2024, 7, 1),
            Date::new(2025, 1, 1),
            Date::new(2025, 7, 1),
        ];
        let cash_account = cash_account.cash_account_evolution(evals_dates)?;

        cash_account.iter().for_each(|(date, amount)| {
            println!("{}: {}", date, amount);
        });
        Ok(())
    }

    #[test]
    fn test_two_instruments() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument1 = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()?;

        let start_date = Date::new(2020, 2, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let instrument2 = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()?;

        let cash_account = CashAccount::new().with_currency(Currency::USD);
        cash_account.add_flows_from_instrument(&instrument1)?;
        cash_account.add_flows_from_instrument(&instrument2)?;
        let evals_dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 7, 1),
            Date::new(2021, 1, 1),
            Date::new(2021, 7, 1),
            Date::new(2022, 1, 1),
            Date::new(2022, 7, 1),
            Date::new(2023, 1, 1),
            Date::new(2023, 7, 1),
            Date::new(2024, 1, 1),
            Date::new(2024, 7, 1),
            Date::new(2025, 1, 1),
            Date::new(2025, 7, 1),
        ];
        let cash_account = cash_account.cash_account_evolution(evals_dates)?;

        cash_account.iter().for_each(|(date, amount)| {
            println!("{}: {}", date, amount);
        });
        Ok(())
    }
}
