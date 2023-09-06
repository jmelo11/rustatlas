use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentRoot,
};
use std::collections::{HashMap, HashSet};

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        fixedratecoupon::FixedRateCoupon,
        simplecashflow::SimpleCashflow,
        traits::{InterestAccrual, Payable},
    },
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::{date::Date, enums::Frequency, period::Period, schedule::MakeSchedule},
    visitors::traits::HasCashflows,
};

use super::{
    fixedrateinstrument::FixedRateInstrument,
    traits::{build_cashflows, calculate_outstanding, notionals_vector, CashflowType, Structure},
};

/// # MakeFixedRateLoan
/// MakeFixedRateLoan is a builder for FixedRateInstrument. Uses the builder pattern.
///

pub struct MakeFixedRateLoan {
    start_date: Option<Date>,
    end_date: Option<Date>,
    payment_frequency: Option<Frequency>,
    tenor: Option<Period>,
    currency: Currency,
    side: Side,
    notional: f64,
    structure: Structure,
    rate: Option<InterestRate>,
    discount_curve_id: Option<usize>,
    disbursements: Option<HashMap<Date, f64>>,
    redemptions: Option<HashMap<Date, f64>>,
    additional_coupon_dates: Option<HashSet<Date>>,
    rate_definition: Option<RateDefinition>,
    rate_value: Option<f64>,
}

impl MakeFixedRateLoan {
    pub fn new() -> MakeFixedRateLoan {
        MakeFixedRateLoan {
            start_date: None,
            end_date: None,
            payment_frequency: None,
            tenor: None,
            rate: None,
            notional: 1.0,
            side: Side::Receive,
            currency: Currency::USD,
            structure: Structure::Other,
            discount_curve_id: None,
            disbursements: None,
            redemptions: None,
            additional_coupon_dates: None,
            rate_definition: None,
            rate_value: None,
        }
    }

    pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> MakeFixedRateLoan {
        self.rate_definition = Some(rate_definition);
        match self.rate_value {
            Some(rate_value) => {
                self.rate = Some(InterestRate::new(
                    rate_value,
                    rate_definition.compounding(),
                    rate_definition.frequency(),
                    rate_definition.day_counter(),
                ));
            }
            None => match self.rate {
                Some(rate) => {
                    self.rate = Some(InterestRate::new(
                        rate.rate(),
                        rate_definition.compounding(),
                        rate_definition.frequency(),
                        rate_definition.day_counter(),
                    ));
                }
                None => (),
            },
        }
        self
    }

    pub fn with_rate_value(mut self, rate_value: f64) -> MakeFixedRateLoan {
        self.rate_value = Some(rate_value);
        match self.rate {
            Some(rate) => {
                self.rate = Some(InterestRate::new(
                    rate_value,
                    rate.compounding(),
                    rate.frequency(),
                    rate.day_counter(),
                ));
            }
            None => match self.rate_definition {
                Some(rate_definition) => {
                    self.rate = Some(InterestRate::new(
                        rate_value,
                        rate_definition.compounding(),
                        rate_definition.frequency(),
                        rate_definition.day_counter(),
                    ));
                }
                None => (),
            },
        }
        self
    }

    pub fn with_start_date(mut self, start_date: Date) -> MakeFixedRateLoan {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Date) -> MakeFixedRateLoan {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_disbursements(mut self, disbursements: HashMap<Date, f64>) -> MakeFixedRateLoan {
        self.disbursements = Some(disbursements);
        self
    }

    pub fn with_redemptions(mut self, redemptions: HashMap<Date, f64>) -> MakeFixedRateLoan {
        self.redemptions = Some(redemptions);
        self
    }

    pub fn with_additional_coupon_dates(
        mut self,
        additional_coupon_dates: HashSet<Date>,
    ) -> MakeFixedRateLoan {
        self.additional_coupon_dates = Some(additional_coupon_dates);
        self
    }

    pub fn with_rate(mut self, rate: InterestRate) -> MakeFixedRateLoan {
        self.rate = Some(rate);
        self
    }

    pub fn with_discount_curve_id(mut self, id: usize) -> MakeFixedRateLoan {
        self.discount_curve_id = Some(id);
        self
    }

    pub fn with_tenor(mut self, tenor: Period) -> MakeFixedRateLoan {
        self.tenor = Some(tenor);
        self
    }

    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeFixedRateLoan {
        self.payment_frequency = Some(frequency);
        self
    }

    pub fn with_side(mut self, side: Side) -> MakeFixedRateLoan {
        self.side = side;
        self
    }

    pub fn with_notional(mut self, notional: f64) -> MakeFixedRateLoan {
        self.notional = notional;
        self
    }

    pub fn with_currency(mut self, currency: Currency) -> MakeFixedRateLoan {
        self.currency = currency;
        self
    }

    pub fn bullet(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::Bullet;
        self
    }

    pub fn equal_redemptions(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::EqualRedemptions;
        self
    }

    pub fn zero(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::Zero;
        self
    }

    pub fn equal_payments(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::EqualPayments;
        self
    }

    pub fn other(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::Other;
        self.payment_frequency = Some(Frequency::OtherFrequency);
        self
    }

    pub fn with_structure(mut self, structure: Structure) -> MakeFixedRateLoan {
        self.structure = structure;
        self
    }

    pub fn build(self) -> FixedRateInstrument {
        let mut cashflows: Vec<Cashflow> = Vec::new();
        
        match self.structure {
            Structure::Bullet => {
                let start_date = self.start_date.expect("Start date not set");
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.expect("Tenor not set");
                        start_date + tenor
                    }
                };
                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build();

                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];
                let notional = vec![self.notional];
                let notionals =
                    notionals_vector(schedule.dates().len() - 1, self.notional, Structure::Bullet);

                let inv_side = match self.side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &notional,
                    inv_side,
                    self.currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    self.rate.expect("Rate not set"),
                    self.side,
                    self.currency,
                );
                build_cashflows(
                    &mut cashflows,
                    &last_date,
                    &notional,
                    self.side,
                    self.currency,
                    CashflowType::Redemption,
                );
                let mut instrument = FixedRateInstrument::new(
                    start_date,
                    end_date,
                    self.notional,
                    self.rate.expect("Rate not set"),
                    payment_frequency,
                    cashflows,
                    self.structure,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            }
            Structure::Other => {
                let disbursements = self.disbursements.expect("Disbursements not set");
                let redemptions = self.redemptions.expect("Redemptions not set");
                let notional = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                let redemtion = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                assert_eq!(notional, redemtion, "Notional must equal total redemption");

                let additional_dates = self
                    .additional_coupon_dates
                    .expect("Additional coupon dates not set");

                let timeline =
                    calculate_outstanding(&disbursements, &redemptions, &additional_dates);

                for (date, amount) in disbursements.iter() {
                    let cashflow = Cashflow::Disbursement(
                        SimpleCashflow::new(*date, self.currency, self.side).with_amount(*amount),
                    );
                    cashflows.push(cashflow);
                }
                for (start_date, end_date, notional) in &timeline {
                    let coupon = FixedRateCoupon::new(
                        *notional,
                        self.rate.expect("Rate not set"),
                        *start_date,
                        *end_date,
                        *end_date,
                        self.currency,
                        self.side,
                    );
                    cashflows.push(Cashflow::FixedRateCoupon(coupon));
                }

                for (date, amount) in redemptions.iter() {
                    let cashflow = Cashflow::Redemption(
                        SimpleCashflow::new(*date, self.currency, self.side).with_amount(*amount),
                    );
                    cashflows.push(cashflow);
                }
                let start_date = &timeline.first().expect("No start date").0;
                let end_date = &timeline.last().expect("No end date").1;
                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");
                let mut instrument = FixedRateInstrument::new(
                    *start_date,
                    *end_date,
                    notional,
                    self.rate.expect("Rate not set"),
                    payment_frequency,
                    cashflows,
                    self.structure,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            }
            Structure::EqualPayments => {
                let start_date = self.start_date.expect("Start date not set");
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.expect("Tenor not set");
                        start_date + tenor
                    }
                };
                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build();

                let rate = self.rate.expect("Rate not set");
                let redemptions =
                    calculate_redemptions(schedule.dates().clone(), rate, self.notional, self.side);
                let mut notionals = redemptions.iter().fold(vec![self.notional], |mut acc, x| {
                    acc.push(acc.last().unwrap() - x);
                    acc
                });
                notionals.pop();
                let first_date = vec![*schedule.dates().first().unwrap()];
                let notional = vec![self.notional];
                let inv_side = match self.side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &notional,
                    inv_side,
                    self.currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    self.side,
                    self.currency,
                );
                let redemption_dates: Vec<Date> =
                    schedule.dates().iter().skip(1).cloned().collect();
                build_cashflows(
                    &mut cashflows,
                    &redemption_dates,
                    &redemptions,
                    self.side,
                    self.currency,
                    CashflowType::Redemption,
                );
                let mut instrument = FixedRateInstrument::new(
                    start_date,
                    end_date,
                    self.notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    self.structure,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            }
            Structure::Zero => {
                let start_date = self.start_date.expect("Start date not set");
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.expect("Tenor not set");
                        start_date + tenor
                    }
                };
                let payment_frequency = Frequency::Once;
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build();
                let notionals =
                    notionals_vector(schedule.dates().len() - 1, self.notional, Structure::Bullet);

                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];
                let notional = vec![self.notional];
                let inv_side = match self.side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &notional,
                    inv_side,
                    self.currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    self.rate.expect("Rate not set"),
                    self.side,
                    self.currency,
                );
                build_cashflows(
                    &mut cashflows,
                    &last_date,
                    &notional,
                    self.side,
                    self.currency,
                    CashflowType::Redemption,
                );
                let mut instrument = FixedRateInstrument::new(
                    start_date,
                    end_date,
                    self.notional,
                    self.rate.expect("Rate not set"),
                    payment_frequency,
                    cashflows,
                    self.structure,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            }
            Structure::EqualRedemptions => {
                let start_date = self.start_date.expect("Start date not set");
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.expect("Tenor not set");
                        start_date + tenor
                    }
                };
                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build();

                let rate = self.rate.expect("Rate not set");
                let first_date = vec![*schedule.dates().first().unwrap()];
                let n = schedule.dates().len() - 1;
                let notionals = notionals_vector(n, self.notional, Structure::EqualRedemptions);
                let notional = vec![self.notional];
                let redemptions = vec![self.notional / n as f64; n];
                let inv_side = match self.side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &notional,
                    inv_side,
                    self.currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    self.side,
                    self.currency,
                );
                let redemption_dates: Vec<Date> =
                    schedule.dates().iter().skip(1).cloned().collect();
                build_cashflows(
                    &mut cashflows,
                    &redemption_dates,
                    &redemptions,
                    self.side,
                    self.currency,
                    CashflowType::Redemption,
                );
                let mut instrument = FixedRateInstrument::new(
                    start_date,
                    end_date,
                    self.notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    self.structure,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            } //_ => panic!("Not implemented"),
        }
    }
}

fn build_coupons_from_notionals(
    cashflows: &mut Vec<Cashflow>,
    dates: &Vec<Date>,
    notionals: &Vec<f64>,
    rate: InterestRate,
    side: Side,
    currency: Currency,
) {
    if dates.len() - 1 != notionals.len() {
        panic!("Dates and notionals must have the same length");
    }
    if dates.len() < 2 {
        panic!("Dates must have at least two elements");
    }
    for (date_pair, notional) in dates.windows(2).zip(notionals) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];
        let coupon = FixedRateCoupon::new(*notional, rate, d1, d2, d2, currency, side);
        cashflows.push(Cashflow::FixedRateCoupon(coupon));
    }
}

struct EqualPaymentCost {
    dates: Vec<Date>,
    rate: InterestRate,
}

impl CostFunction for EqualPaymentCost {
    type Param = f64;
    type Output = f64;
    fn cost(&self, payment: &Self::Param) -> Result<Self::Output, Error> {
        let mut total_amount = 1.0;
        for date_pair in self.dates.windows(2) {
            let d1 = date_pair[0];
            let d2 = date_pair[1];
            let interest = total_amount * (self.rate.compound_factor(d1, d2) - 1.0);
            total_amount -= payment - interest;
        }
        Ok(total_amount)
    }
}
fn calculate_redemptions(
    dates: Vec<Date>,
    rate: InterestRate,
    notional: f64,
    side: Side,
) -> Vec<f64> {
    let cost = EqualPaymentCost {
        dates: dates.clone(),
        rate: rate,
    };
    let solver = BrentRoot::new(0.0, 1.0, 1e-6);

    let init_param = 1.0 / (dates.len() as f64);
    let res = Executor::new(cost, solver)
        .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
        .run()
        .expect("Solver failed");

    let payment = res.state().best_param.expect("No best parameter found") * notional;

    let mut redemptions = Vec::new();
    let mut total_amount = notional;
    let flag = match side {
        Side::Pay => -1.0,
        Side::Receive => 1.0,
    };
    for date_pair in dates.windows(2) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];
        let interest = total_amount * (rate.compound_factor(d1, d2) - 1.0);
        let k = payment - interest;
        total_amount -= k;
        redemptions.push(k * flag);
    }
    redemptions
}

impl Into<MakeFixedRateLoan> for FixedRateInstrument {
    fn into(self) -> MakeFixedRateLoan {
        match self.structure() {
            Structure::Other => {
                let mut disbursements = HashMap::new();
                let mut redemptions = HashMap::new();
                let mut additional_coupon_dates = HashSet::new();
                for cashflow in self.cashflows() {
                    match cashflow {
                        Cashflow::Disbursement(c) => {
                            disbursements.insert(c.payment_date(), c.amount());
                        }
                        Cashflow::Redemption(c) => {
                            redemptions.insert(c.payment_date(), c.amount());
                        }
                        Cashflow::FixedRateCoupon(c) => {
                            additional_coupon_dates.insert(c.accrual_start_date());
                            additional_coupon_dates.insert(c.accrual_end_date());
                        }
                        _ => (),
                    }
                }
                MakeFixedRateLoan::new()
                    .with_start_date(self.start_date())
                    .with_end_date(self.end_date())
                    .with_disbursements(disbursements)
                    .with_redemptions(redemptions)
                    .with_additional_coupon_dates(additional_coupon_dates)
                    .with_rate(self.rate())
                    .with_notional(self.notional())
                    .with_structure(self.structure())
            }
            _ => MakeFixedRateLoan::new()
                .with_start_date(self.start_date())
                .with_end_date(self.end_date())
                .with_payment_frequency(self.payment_frequency())
                .with_rate(self.rate())
                .with_notional(self.notional())
                .with_structure(self.structure()),
        }
    }
}

impl From<&FixedRateInstrument> for MakeFixedRateLoan {
    fn from(val: &FixedRateInstrument) -> Self {
        val.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cashflows::{
            cashflow::{Cashflow, Side},
            traits::Payable,
        },
        currencies::enums::Currency,
        instruments::makefixedrateloan::MakeFixedRateLoan,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
        visitors::traits::HasCashflows,
    };
    use std::collections::{HashMap, HashSet};

    #[test]
    fn build_bullet() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn build_equal_payments() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let notional = 1000.0;
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(notional)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_payments()
            .build();

        assert_eq!(instrument.notional(), notional);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));

        let mut payments = HashMap::new();
        instrument.cashflows().iter().for_each(|cf| match cf {
            Cashflow::FixedRateCoupon(c) => {
                if payments.contains_key(&c.payment_date()) {
                    payments.insert(c.payment_date(), payments[&c.payment_date()] + c.amount());
                } else {
                    payments.insert(c.payment_date(), c.amount());
                }
            }
            Cashflow::Redemption(c) => {
                if payments.contains_key(&c.payment_date()) {
                    payments.insert(c.payment_date(), payments[&c.payment_date()] + c.amount());
                } else {
                    payments.insert(c.payment_date(), c.amount());
                }
            }
            _ => (),
        });

        //check if all equal
        let first = payments.values().next().unwrap();
        payments.values().for_each(|x| assert_eq!(first, x));
    }

    #[test]
    fn build_equal_redemptions() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_redemptions()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn build_equal_redemptions_with_tenor() {
        let start_date = Date::new(2020, 1, 1);

        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_tenor(Period::new(5, TimeUnit::Years))
            .with_payment_frequency(Frequency::Semiannual)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_redemptions()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn build_zero() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(1, TimeUnit::Years);

        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .zero()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn build_zero_with_tenor() {
        let start_date = Date::new(2020, 1, 1);
        let tenor = Period::new(1, TimeUnit::Years);

        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_tenor(tenor)
            .with_rate(rate)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .zero()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.start_date(), start_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn build_other() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(3, TimeUnit::Years);

        let mut disbursements = HashMap::new();
        disbursements.insert(start_date, 100.0);

        let mut redemptions = HashMap::new();
        redemptions.insert(start_date + Period::new(1, TimeUnit::Years), 30.0);
        redemptions.insert(end_date, 70.0);

        let mut additional_coupon_dates = HashSet::new();

        additional_coupon_dates.insert(start_date + Period::new(1, TimeUnit::Years));
        additional_coupon_dates.insert(start_date + Period::new(2, TimeUnit::Years));

        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_disbursements(disbursements)
            .with_redemptions(redemptions)
            .with_additional_coupon_dates(additional_coupon_dates)
            .with_rate(rate)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .other()
            .build();

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        instrument
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }

    #[test]
    fn into_test() {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let notional = 100.0;
        let instrument = super::MakeFixedRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(Frequency::Monthly)
            .with_rate(rate)
            .with_notional(notional)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_payments()
            .build();

        let builder: MakeFixedRateLoan = instrument.clone().into();
        let instrument2 = builder.build();
        assert_eq!(instrument2.notional(), instrument.notional());
        assert_eq!(instrument2.rate(), instrument.rate());

        assert_eq!(instrument2.payment_frequency(), Frequency::Monthly);
        assert_eq!(instrument2.start_date(), start_date);
        assert_eq!(instrument2.end_date(), end_date);

        instrument2
            .cashflows()
            .iter()
            .for_each(|cf| println!("{}", cf));
    }
}
