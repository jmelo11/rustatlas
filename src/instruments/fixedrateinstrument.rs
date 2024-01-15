use crate::cashflows::cashflow::{Cashflow, Side};
use crate::cashflows::traits::{InterestAccrual, Payable};
use crate::currencies::enums::Currency;
use crate::rates::interestrate::InterestRate;
use crate::time::date::Date;
use crate::time::enums::Frequency;
use crate::visitors::traits::HasCashflows;
use crate::utils::errors::Result;
use super::traits::Structure;

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

#[derive(Clone)]
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
    id: Option<usize>,
    issue_date: Option<Date>,
    yield_rate : Option<InterestRate>
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
        id: Option<usize>,
        issue_date: Option<Date>,
        yield_rate : Option<InterestRate>
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
            yield_rate
        }
    }

    pub fn id(&self) -> Option<usize> {
        self.id
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

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    pub fn yield_rate(&self) -> Option<InterestRate> {
        self.yield_rate
    }

}

impl InterestAccrual for FixedRateInstrument {
    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        match self.yield_rate {
            Some(rate) => {
                let ini_pv = self.discounted_cashflows(start_date, rate)?;
                let end_pv = self.discounted_cashflows(end_date, rate)?;
                let redemption_amount = self.sum_cashflows_between_dates(start_date, end_date)?;
                Ok(end_pv - ini_pv + redemption_amount)
            },
            None => {
                let total_accrued_amount = self.cashflows.iter().fold(0.0, |acc, cf| {
                    acc + cf.accrued_amount(start_date, end_date).unwrap_or(0.0)
                });
                Ok(total_accrued_amount)
            }
        }
    }
    fn accrual_start_date(&self) -> Date {
        self.start_date
    }
    fn accrual_end_date(&self) -> Date {
        self.end_date
    }
}

impl FixedRateInstrument { 
    fn discounted_cashflows(&self, date: Date, rate: InterestRate) -> Result<f64> {
        let mut pv = 0.0;
        for cf in self.cashflows.iter() {
            if cf.payment_date()>date {
                let amount = cf.amount()?;
                let payment_date = cf.payment_date();
                let df = rate.discount_factor(date, payment_date);
                pv += amount * df*cf.side().sign();
            }
        }
        Ok(pv)
    }

    fn sum_cashflows_between_dates(&self, initial_date: Date, final_date: Date) -> Result<f64> {
        let mut redemption_amount = 0.0;
        let initial_instrument_date = self.start_date();

        for cf in self.cashflows.iter() {
            if cf.payment_date()>initial_instrument_date && cf.payment_date()>initial_date && cf.payment_date()<=final_date {
                redemption_amount += cf.amount()? * cf.side().sign();
            }
        }
        Ok(redemption_amount)
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
        cashflows::{cashflow::Side, traits::InterestAccrual},
        currencies::enums::Currency,
        instruments::makefixedrateinstrument::MakeFixedRateInstrument,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
        utils::errors::Result,
    };
    use std::collections::{HashMap, HashSet};
    
    #[test]
    fn accrual_bullet_instrumen_with_tir() -> Result<()> {
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

        let date = start_date + Period::new(1, TimeUnit::Months);
        let mut accrual_aux = instrument.accrued_amount(date, date+Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27385.193447).abs() < 1e-6);

        let date = start_date + Period::new(2, TimeUnit::Months);
        accrual_aux = instrument.accrued_amount(date, date+Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27540.033312).abs() < 1e-6);

        let date = start_date + Period::new(3, TimeUnit::Months);   
        accrual_aux = instrument.accrued_amount(date, date+Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 165982.43365).abs() < 1e-6);

        let date = start_date + Period::new(54, TimeUnit::Months);
        accrual_aux = instrument.accrued_amount(date, date+Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 171307.0814148).abs() < 1e-6);

        Ok(())
    }


    #[test]
    fn accrual_other_instrument_with_tir() -> Result<()> {
        let start_date = Date::new(2024, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
       
        let mut disbursements = HashMap::new();
        disbursements.insert(start_date, 5000000.0);

        let mut redemptions = HashMap::new();
        redemptions.insert(start_date + Period::new(1, TimeUnit::Years), 1000000.0);
        redemptions.insert(start_date + Period::new(3, TimeUnit::Years), 1000000.0);
        redemptions.insert(end_date, 3000000.0);

        let mut additional_coupon_dates = HashSet::new();

        additional_coupon_dates.insert(start_date + Period::new(6, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(12, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(18, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(24, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(30, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(36, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(42, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(48, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(54, TimeUnit::Months));
        additional_coupon_dates.insert(start_date + Period::new(60, TimeUnit::Months));

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
            .with_disbursements(disbursements)
            .with_redemptions(redemptions)
            .with_additional_coupon_dates(additional_coupon_dates)
            .with_rate(rate)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .with_yield_rate(yield_rate)
            .other()
            .build()?;

        //let cashflows = instrument.cashflows();
        ////print cashflows
        //for cf in cashflows {
        //    println!("{:?} {:?}", cf.payment_date(), cf.amount());
        //}

        //for i in 0..=60 {
        //    let date = start_date + Period::new(i, TimeUnit::Months);
        //    println!("vp: {:?}", instrument.intrument_cashflow_discounted(date, yield_rate));
        //}

        //for i in 0..60 {
        //    let date = start_date + Period::new(i, TimeUnit::Months); 
        //    println!("Fecha: {:?}   Devengo: {:?}", instrument.accrued_amount(date, date+Period::new(1, TimeUnit::Months)), date+Period::new(1, TimeUnit::Months));
        //}
        
        let date = start_date + Period::new(1, TimeUnit::Months);
        let mut accrual_aux = instrument.accrued_amount(date, date+Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27621.871414).abs() < 1e-6);

        let date = start_date + Period::new(2, TimeUnit::Months);
        accrual_aux = instrument.accrued_amount(date, date+Period::new(1, TimeUnit::Months))?;
        assert!((accrual_aux - 27778.049491).abs() < 1e-6);

        let date = start_date + Period::new(3, TimeUnit::Months);   
        accrual_aux = instrument.accrued_amount(date, date+Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 167439.059899).abs() < 1e-6);

        let date = start_date + Period::new(54, TimeUnit::Months);
        accrual_aux = instrument.accrued_amount(date, date+Period::new(6, TimeUnit::Months))?;
        assert!((accrual_aux - 102784.2488489).abs() < 1e-6);

        Ok(())
    }

}



