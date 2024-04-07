use serde::{Deserialize, Serialize};

use super::traits::Structure;
use crate::cashflows::cashflow::{Cashflow, Side};
use crate::core::traits::HasCurrency;
use crate::currencies::enums::Currency;
use crate::prelude::{AtlasError, Payable};
use crate::rates::interestrate::InterestRate;
use crate::time::date::Date;
use crate::time::enums::Frequency;
use crate::utils::errors::Result;
use crate::visitors::traits::HasCashflows;

/// # FixedRateInstrument
/// A fixed rate instrument.
///
/// ## Parameters
/// * `start_date` - The start date.
/// * `end_date` - The end date.
/// * `notional` - The notional.
/// * `rate` - The rate.
/// * `cashflows` - The cashflows.
/// * `structure` - The structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FixedRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    rate: InterestRate,
    payment_frequency: Frequency,
    cashflows: Vec<Cashflow>,
    structure: Structure,
    side: Side,
    currency: Currency,
    discount_curve_id: Option<usize>,
    id: Option<String>,
    issue_date: Option<Date>,
    yield_rate: Option<InterestRate>,
}

impl FixedRateInstrument {
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        rate: InterestRate,
        payment_frequency: Frequency,
        cashflows: Vec<Cashflow>,
        structure: Structure,
        side: Side,
        currency: Currency,
        discount_curve_id: Option<usize>,
        id: Option<String>,
        issue_date: Option<Date>,
        yield_rate: Option<InterestRate>,
    ) -> Self {
        FixedRateInstrument {
            start_date,
            end_date,
            notional,
            rate,
            payment_frequency,
            cashflows,
            structure,
            side,
            currency,
            discount_curve_id,
            id,
            issue_date,
            yield_rate,
        }
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn notional(&self) -> f64 {
        self.notional
    }

    pub fn rate(&self) -> InterestRate {
        self.rate
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    pub fn set_discount_curve_id(mut self, discount_curve_id: usize) -> Self {
        self.discount_curve_id = Some(discount_curve_id);
        self.mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_discount_curve_id(discount_curve_id));

        self
    }
}

impl HasCurrency for FixedRateInstrument {
    fn currency(&self) -> Result<Currency> {
        Ok(self.currency)
    }
}

/// # BondAccrual
/// Implements fixed rate bond accrual using a yield rate.  
/// The yield rate is used to discount the cashflows to between the start and end dates and calculate the accrued amount.
pub trait BondAccrual: HasCashflows {
    fn yield_rate(&self) -> Option<InterestRate>;

    fn bond_accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        let ini_pv = self.discounted_cashflows(start_date)?;
        let end_pv = self.discounted_cashflows(end_date)?;
        let accrual = self.matured_amount_accrual(start_date, end_date)?;
        // let maturing_cashflows = self
        //     .cashflows()
        //     .iter()
        //     .filter(|cf| cf.payment_date() == end_date)
        //     .fold(0.0, |acc, cf| acc + cf.amount().unwrap());
        // Ok(end_pv - ini_pv + maturing_cashflows)
        Ok(end_pv - ini_pv + accrual)
    }

    /// Calculates the accrual of cash paid between two dates.
    fn matured_amount_accrual(&self, from: Date, to: Date) -> Result<f64> {
        // let rate = self
        //     .yield_rate()
        //     .ok_or(AtlasError::NotFoundErr("Yield rate".to_string()))?;

        let cashflows = self
            .cashflows()
            .iter()
            .filter(|cf| cf.payment_date() >= from && cf.payment_date() < to)
            .collect::<Vec<&Cashflow>>();

        let mut amount = 0.0;
        cashflows.iter().for_each(|cf| {
            //amount += cf.amount().unwrap() / rate.discount_factor(cf.payment_date(), to);
            amount += cf.amount().unwrap();
        });
        Ok(amount)
    }

    fn discounted_cashflows(&self, evaluation_date: Date) -> Result<f64> {
        let rate = self
            .yield_rate()
            .ok_or(AtlasError::NotFoundErr("Yield rate".to_string()))?;

        Ok(self
            .cashflows()
            .iter()
            .filter(|cf| cf.payment_date() >= evaluation_date)
            .fold(0.0, |acc, cf| {
                let npv = cf.amount().unwrap()
                    * rate.discount_factor(evaluation_date, cf.payment_date())
                    * cf.side().sign();
                acc + npv
            }))
    }
}

impl BondAccrual for FixedRateInstrument {
    fn yield_rate(&self) -> Option<InterestRate> {
        self.yield_rate
    }
}

impl HasCashflows for FixedRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cashflows::cashflow::Side,
        currencies::enums::Currency,
        instruments::makefixedrateinstrument::MakeFixedRateInstrument,
        prelude::BondAccrual,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
        utils::errors::Result,
    };

    #[test]
    fn bond_accrual_bullet_instrument() -> Result<()> {
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
            .with_notional(5_000_000.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .with_yield_rate(yield_rate)
            .bullet()
            .build()?;

        let date = start_date + Period::new(1, TimeUnit::Months);
        let mut accrual_aux =
            instrument.bond_accrued_amount(date, date + Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27_385.1934467).abs() < 1e-6);

        let date = start_date + Period::new(2, TimeUnit::Months);
        accrual_aux =
            instrument.bond_accrued_amount(date, date + Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27_540.0333112).abs() < 1e-6);

        let date = start_date + Period::new(3, TimeUnit::Months);
        accrual_aux =
            instrument.bond_accrued_amount(date, date + Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 165_982.433650).abs() < 1e-6);

        let date = start_date + Period::new(54, TimeUnit::Months);
        accrual_aux =
            instrument.bond_accrued_amount(date, date + Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 171_307.0814148).abs() < 1e-6);

        Ok(())
    }

    // #[test]
    // fn bond_accrual_other_instrument_with_tir() -> Result<()> {
    //     let start_date = Date::new(2024, 1, 1);
    //     let end_date = start_date + Period::new(5, TimeUnit::Years);

    //     let mut disbursements = HashMap::new();
    //     disbursements.insert(start_date, 5000000.0);

    //     let mut redemptions = HashMap::new();
    //     redemptions.insert(start_date + Period::new(1, TimeUnit::Years), 1000000.0);
    //     redemptions.insert(start_date + Period::new(3, TimeUnit::Years), 1000000.0);
    //     redemptions.insert(end_date, 3000000.0);

    //     let mut additional_coupon_dates = HashSet::new();

    //     additional_coupon_dates.insert(start_date + Period::new(6, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(12, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(18, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(24, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(30, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(36, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(42, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(48, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(54, TimeUnit::Months));
    //     additional_coupon_dates.insert(start_date + Period::new(60, TimeUnit::Months));

    //     let rate = InterestRate::new(
    //         0.06,
    //         Compounding::Simple,
    //         Frequency::Annual,
    //         DayCounter::Thirty360,
    //     );

    //     let yield_rate = InterestRate::new(
    //         0.07,
    //         Compounding::Compounded,
    //         Frequency::Annual,
    //         DayCounter::Thirty360,
    //     );

    //     let instrument = MakeFixedRateInstrument::new()
    //         .with_start_date(start_date)
    //         .with_disbursements(disbursements)
    //         .with_redemptions(redemptions)
    //         .with_additional_coupon_dates(additional_coupon_dates)
    //         .with_rate(rate)
    //         .with_side(Side::Receive)
    //         .with_currency(Currency::USD)
    //         .with_yield_rate(yield_rate)
    //         .other()
    //         .build()?;

    //     let date = start_date + Period::new(1, TimeUnit::Months);
    //     let mut accrual_aux =
    //         instrument.bond_accrued_amount(date, date + Period::new(1, TimeUnit::Months))?;
    //     assert!((accrual_aux - 27621.871414).abs() < 1e-6);

    //     let date = start_date + Period::new(2, TimeUnit::Months);
    //     accrual_aux =
    //         instrument.bond_accrued_amount(date, date + Period::new(1, TimeUnit::Months))?;
    //     assert!((accrual_aux - 27778.049491).abs() < 1e-6);

    //     let date = start_date + Period::new(3, TimeUnit::Months);
    //     accrual_aux =
    //         instrument.bond_accrued_amount(date, date + Period::new(6, TimeUnit::Months))?;
    //     assert!((accrual_aux - 167439.059899).abs() < 1e-6);

    //     let date = start_date + Period::new(54, TimeUnit::Months);
    //     accrual_aux =
    //         instrument.bond_accrued_amount(date, date + Period::new(6, TimeUnit::Months))?;
    //     assert!((accrual_aux - 102784.2488489).abs() < 1e-6);

    //     Ok(())
    // }
}
