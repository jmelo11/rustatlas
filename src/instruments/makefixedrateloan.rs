use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentRoot,
};

use std::collections::{HashMap, HashSet};
use thiserror::Error;

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        fixedratecoupon::FixedRateCoupon,
        simplecashflow::SimpleCashflow,
        traits::{InterestAccrual, Payable},
    },
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::{
        date::Date,
        enums::Frequency,
        period::Period,
        schedule::{MakeSchedule, MakeScheduleError},
    },
    visitors::traits::HasCashflows,
};

use super::{
    fixedrateinstrument::FixedRateInstrument,
    traits::{build_cashflows, calculate_outstanding, notionals_vector, CashflowType, Structure},
};

/// # MakeFixedRateLoan
/// MakeFixedRateLoan is a builder for FixedRateInstrument. Uses the builder pattern.
#[derive(Debug, Clone)]
pub struct MakeFixedRateLoan {
    start_date: Option<Date>,
    end_date: Option<Date>,
    payment_frequency: Option<Frequency>,
    tenor: Option<Period>,
    currency: Option<Currency>,
    side: Option<Side>,
    notional: Option<f64>,
    structure: Option<Structure>,
    rate: Option<InterestRate>,
    discount_curve_id: Option<usize>,
    disbursements: Option<HashMap<Date, f64>>,
    redemptions: Option<HashMap<Date, f64>>,
    additional_coupon_dates: Option<HashSet<Date>>,
    rate_definition: Option<RateDefinition>,
    rate_value: Option<f64>,
}

/// # MakeFixedRateLoanError
/// MakeFixedRateLoanError is an enum that represents the possible errors that can occur when building a FixedRateInstrument.
#[derive(Error, Debug)]
pub enum MakeFixedRateLoanError {
    /// The start date is not set.
    #[error("Start date not set")]
    StartDateNotSet,
    /// The end date is not set.
    #[error("End date not set")]
    EndDateNotSet,
    /// The payment frequency is not set.
    #[error("Payment frequency not set")]
    PaymentFrequencyNotSet,
    /// The tenor is not set.
    #[error("Tenor not set")]
    TenorNotSet,
    /// The rate is not set.
    #[error("Rate not set")]
    RateNotSet,
    /// The rate definition is not set.
    #[error("Rate definition not set")]
    RateDefinitionNotSet,
    /// The rate value is not set.
    #[error("Rate value not set")]
    RateValueNotSet,
    /// The disbursements are not set.
    #[error("Disbursements not set")]
    DisbursementsNotSet,
    /// The redemptions are not set.
    #[error("Redemptions not set")]
    RedemptionsNotSet,
    /// The additional coupon dates are not set.
    #[error("Additional coupon dates not set")]
    AdditionalCouponDatesNotSet,
    /// The schedule could not be built.
    #[error("Schedule build error: {0}")]
    ScheduleBuildError(String),
    /// Currency not set.
    #[error("Currency not set")]
    CurrencyNotSet,
    /// Side not set.
    #[error("Side not set")]
    SideNotSet,
    /// Notional not set.
    #[error("Notional not set")]
    NotionalNotSet,
    /// Redemptions and disbursements do not match.
    #[error("Redemptions and disbursements do not match")]
    RedemptionsAndDisbursementsDoNotMatch,
    /// The structure is not set.
    #[error("Structure not set")]
    StructureNotSet,
}

impl From<MakeScheduleError> for MakeFixedRateLoanError {
    fn from(e: MakeScheduleError) -> Self {
        MakeFixedRateLoanError::ScheduleBuildError(format!("{}", e))
    }
}

// impl Display for MakeFixedRateLoanError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             MakeFixedRateLoanError::StartDateNotSet => write!(f, "Start date not set"),
//             MakeFixedRateLoanError::EndDateNotSet => write!(f, "End date not set"),
//             MakeFixedRateLoanError::PaymentFrequencyNotSet => {
//                 write!(f, "Payment frequency not set")
//             }
//             MakeFixedRateLoanError::TenorNotSet => write!(f, "Tenor not set"),
//             MakeFixedRateLoanError::RateNotSet => write!(f, "Rate not set"),
//             MakeFixedRateLoanError::RateDefinitionNotSet => write!(f, "Rate definition not set"),
//             MakeFixedRateLoanError::RateValueNotSet => write!(f, "Rate value not set"),
//             MakeFixedRateLoanError::DisbursementsNotSet => write!(f, "Disbursements not set"),
//             MakeFixedRateLoanError::RedemptionsNotSet => write!(f, "Redemptions not set"),
//             MakeFixedRateLoanError::AdditionalCouponDatesNotSet => {
//                 write!(f, "Additional coupon dates not set")
//             }
//             MakeFixedRateLoanError::ScheduleBuildError(e) => write!(f, "{}", e),
//             MakeFixedRateLoanError::CurrencyNotSet => write!(f, "Currency not set"),
//             MakeFixedRateLoanError::SideNotSet => write!(f, "Side not set"),
//             MakeFixedRateLoanError::NotionalNotSet => write!(f, "Notional not set"),
//             MakeFixedRateLoanError::RedemptionsAndDisbursementsDoNotMatch => {
//                 write!(f, "Redemptions and disbursements do not match")
//             }
//             MakeFixedRateLoanError::StructureNotSet => write!(f, "Structure not set"),
//         }
//     }
// }

/// New, setters and getters
impl MakeFixedRateLoan {
    pub fn new() -> MakeFixedRateLoan {
        MakeFixedRateLoan {
            start_date: None,
            end_date: None,
            payment_frequency: None,
            tenor: None,
            rate: None,
            notional: None,
            side: None,
            currency: None,
            structure: None,
            discount_curve_id: None,
            disbursements: None,
            redemptions: None,
            additional_coupon_dates: None,
            rate_definition: None,
            rate_value: None,
        }
    }

    /// Sets the currency.
    pub fn with_currency(mut self, currency: Currency) -> MakeFixedRateLoan {
        self.currency = Some(currency);
        self
    }

    /// Sets the side.
    pub fn with_side(mut self, side: Side) -> MakeFixedRateLoan {
        self.side = Some(side);
        self
    }

    /// Sets the notional.
    pub fn with_notional(mut self, notional: f64) -> MakeFixedRateLoan {
        self.notional = Some(notional);
        self
    }

    /// Sets the rate definition.
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

    /// Sets the rate value.
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

    /// Sets the start date.
    pub fn with_start_date(mut self, start_date: Date) -> MakeFixedRateLoan {
        self.start_date = Some(start_date);
        self
    }

    /// Sets the end date.
    pub fn with_end_date(mut self, end_date: Date) -> MakeFixedRateLoan {
        self.end_date = Some(end_date);
        self
    }

    /// Sets the disbursements.
    pub fn with_disbursements(mut self, disbursements: HashMap<Date, f64>) -> MakeFixedRateLoan {
        self.disbursements = Some(disbursements);
        self
    }

    /// Sets the redemptions.
    pub fn with_redemptions(mut self, redemptions: HashMap<Date, f64>) -> MakeFixedRateLoan {
        self.redemptions = Some(redemptions);
        self
    }

    /// Sets the additional coupon dates.
    pub fn with_additional_coupon_dates(
        mut self,
        additional_coupon_dates: HashSet<Date>,
    ) -> MakeFixedRateLoan {
        self.additional_coupon_dates = Some(additional_coupon_dates);
        self
    }

    /// Sets the rate.
    pub fn with_rate(mut self, rate: InterestRate) -> MakeFixedRateLoan {
        self.rate = Some(rate);
        self
    }

    /// Sets the discount curve id.
    pub fn with_discount_curve_id(mut self, id: Option<usize>) -> MakeFixedRateLoan {
        self.discount_curve_id = id;
        self
    }

    /// Sets the tenor.
    pub fn with_tenor(mut self, tenor: Period) -> MakeFixedRateLoan {
        self.tenor = Some(tenor);
        self
    }

    /// Sets the payment frequency.
    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeFixedRateLoan {
        self.payment_frequency = Some(frequency);
        self
    }

    /// Sets the structure to bullet.
    pub fn bullet(mut self) -> MakeFixedRateLoan {
        self.structure = Some(Structure::Bullet);
        self
    }

    /// Sets the structure to equal redemptions.
    pub fn equal_redemptions(mut self) -> MakeFixedRateLoan {
        self.structure = Some(Structure::EqualRedemptions);
        self
    }

    /// Sets the structure to zero.
    pub fn zero(mut self) -> MakeFixedRateLoan {
        self.structure = Some(Structure::Zero);
        self.payment_frequency = Some(Frequency::Once);
        self
    }

    /// Sets the structure to equal payments.
    pub fn equal_payments(mut self) -> MakeFixedRateLoan {
        self.structure = Some(Structure::EqualPayments);
        self
    }

    /// Sets the structure to other.
    pub fn other(mut self) -> MakeFixedRateLoan {
        self.structure = Some(Structure::Other);
        self.payment_frequency = Some(Frequency::OtherFrequency);
        self
    }

    /// Sets the structure.
    pub fn with_structure(mut self, structure: Structure) -> MakeFixedRateLoan {
        self.structure = Some(structure);
        self
    }
}

impl MakeFixedRateLoan {
    pub fn build(self) -> Result<FixedRateInstrument, MakeFixedRateLoanError> {
        let mut cashflows = Vec::new();
        let structure = self
            .structure
            .ok_or(MakeFixedRateLoanError::StructureNotSet)?;
        let rate = self.rate.ok_or(MakeFixedRateLoanError::RateNotSet)?;
        let payment_frequency = self
            .payment_frequency
            .ok_or(MakeFixedRateLoanError::PaymentFrequencyNotSet)?;

        let side = self.side.ok_or(MakeFixedRateLoanError::SideNotSet)?;
        let currency = self
            .currency
            .ok_or(MakeFixedRateLoanError::CurrencyNotSet)?;

        match structure {
            Structure::Bullet => {
                let start_date = self
                    .start_date
                    .ok_or(MakeFixedRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFixedRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;

                let notional = self
                    .notional
                    .ok_or(MakeFixedRateLoanError::NotionalNotSet)?;
                let side = self.side.ok_or(MakeFixedRateLoanError::SideNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };

                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];
                let notionals =
                    notionals_vector(schedule.dates().len() - 1, notional, Structure::Bullet);

                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    inv_side,
                    currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    side,
                    currency,
                );
                build_cashflows(
                    &mut cashflows,
                    &last_date,
                    &vec![notional],
                    side,
                    currency,
                    CashflowType::Redemption,
                );
                cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(self.discount_curve_id));

                Ok(FixedRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    structure,
                    side,
                    currency,
                    self.discount_curve_id,
                ))
            }
            Structure::Other => {
                let disbursements = self
                    .disbursements
                    .ok_or(MakeFixedRateLoanError::DisbursementsNotSet)?;
                let redemptions = self
                    .redemptions
                    .ok_or(MakeFixedRateLoanError::RedemptionsNotSet)?;
                let notional = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                let redemption = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                if notional != redemption {
                    return Err(MakeFixedRateLoanError::RedemptionsAndDisbursementsDoNotMatch);
                }

                let additional_dates = self.additional_coupon_dates.unwrap_or_default();

                let timeline =
                    calculate_outstanding(&disbursements, &redemptions, &additional_dates);

                for (date, amount) in disbursements.iter() {
                    let cashflow = Cashflow::Disbursement(
                        SimpleCashflow::new(*date, currency, side).with_amount(*amount),
                    );
                    cashflows.push(cashflow);
                }
                for (start_date, end_date, notional) in &timeline {
                    let coupon = FixedRateCoupon::new(
                        *notional,
                        rate,
                        *start_date,
                        *end_date,
                        *end_date,
                        currency,
                        side,
                    );
                    cashflows.push(Cashflow::FixedRateCoupon(coupon));
                }
                for (date, amount) in redemptions.iter() {
                    let cashflow = Cashflow::Redemption(
                        SimpleCashflow::new(*date, currency, side).with_amount(*amount),
                    );
                    cashflows.push(cashflow);
                }
                let start_date = &timeline.first().expect("No start date").0;
                let end_date = &timeline.last().expect("No end date").1;

                cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(self.discount_curve_id));

                Ok(FixedRateInstrument::new(
                    *start_date,
                    *end_date,
                    notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    structure,
                    side,
                    currency,
                    self.discount_curve_id,
                ))
            }
            Structure::EqualPayments => {
                let start_date = self
                    .start_date
                    .ok_or(MakeFixedRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFixedRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;

                let notional = self
                    .notional
                    .ok_or(MakeFixedRateLoanError::NotionalNotSet)?;
                let side = self.side.ok_or(MakeFixedRateLoanError::SideNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };

                let redemptions =
                    calculate_redemptions(schedule.dates().clone(), rate, notional, side);
                let mut notionals = redemptions.iter().fold(vec![notional], |mut acc, x| {
                    acc.push(acc.last().unwrap() - x);
                    acc
                });
                notionals.pop();
                let first_date = vec![*schedule.dates().first().unwrap()];

                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    inv_side,
                    currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    side,
                    currency,
                );
                let redemption_dates: Vec<Date> =
                    schedule.dates().iter().skip(1).cloned().collect();
                build_cashflows(
                    &mut cashflows,
                    &redemption_dates,
                    &redemptions,
                    side,
                    currency,
                    CashflowType::Redemption,
                );

                cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(self.discount_curve_id));

                Ok(FixedRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    structure,
                    side,
                    currency,
                    self.discount_curve_id,
                ))
            }
            Structure::Zero => {
                let start_date = self
                    .start_date
                    .ok_or(MakeFixedRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFixedRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;

                let notional = self
                    .notional
                    .ok_or(MakeFixedRateLoanError::NotionalNotSet)?;
                let side = self.side.ok_or(MakeFixedRateLoanError::SideNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };

                let notionals =
                    notionals_vector(schedule.dates().len() - 1, notional, Structure::Bullet);

                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];

                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    inv_side,
                    currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    side,
                    currency,
                );
                build_cashflows(
                    &mut cashflows,
                    &last_date,
                    &vec![notional],
                    side,
                    currency,
                    CashflowType::Redemption,
                );

                cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(self.discount_curve_id));

                Ok(FixedRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    structure,
                    side,
                    currency,
                    self.discount_curve_id,
                ))
            }
            Structure::EqualRedemptions => {
                let start_date = self
                    .start_date
                    .ok_or(MakeFixedRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFixedRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;

                let notional = self
                    .notional
                    .ok_or(MakeFixedRateLoanError::NotionalNotSet)?;
                let side = self.side.ok_or(MakeFixedRateLoanError::SideNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };

                let first_date = vec![*schedule.dates().first().unwrap()];

                let n = schedule.dates().len() - 1;
                let notionals = notionals_vector(n, notional, Structure::EqualRedemptions);
                let redemptions = vec![notional / n as f64; n];

                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    inv_side,
                    currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    rate,
                    side,
                    currency,
                );
                let redemption_dates: Vec<Date> =
                    schedule.dates().iter().skip(1).cloned().collect();
                build_cashflows(
                    &mut cashflows,
                    &redemption_dates,
                    &redemptions,
                    side,
                    currency,
                    CashflowType::Redemption,
                );

                cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(self.discount_curve_id));

                Ok(FixedRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    rate,
                    payment_frequency,
                    cashflows,
                    structure,
                    side,
                    currency,
                    self.discount_curve_id,
                ))
            }
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
                            disbursements.insert(c.payment_date(), c.amount().unwrap());
                        }
                        Cashflow::Redemption(c) => {
                            redemptions.insert(c.payment_date(), c.amount().unwrap());
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
                    .with_discount_curve_id(self.discount_curve_id())
                    .with_structure(self.structure())
                    .with_side(self.side())
                    .with_currency(self.currency())
            }
            _ => MakeFixedRateLoan::new()
                .with_start_date(self.start_date())
                .with_end_date(self.end_date())
                .with_payment_frequency(self.payment_frequency())
                .with_rate(self.rate())
                .with_notional(self.notional())
                .with_discount_curve_id(self.discount_curve_id())
                .with_structure(self.structure())
                .with_side(self.side())
                .with_currency(self.currency()),
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
        instruments::makefixedrateloan::{MakeFixedRateLoan, self},
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

    use super::MakeFixedRateLoanError;

    #[test]
    fn build_bullet() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn build_equal_payments() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

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
                    payments.insert(c.payment_date(), payments[&c.payment_date()] + c.amount().unwrap());
                } else {
                    payments.insert(c.payment_date(), c.amount().unwrap());
                }
            }
            Cashflow::Redemption(c) => {
                if payments.contains_key(&c.payment_date()) {
                    payments.insert(c.payment_date(), payments[&c.payment_date()] + c.amount().unwrap());
                } else {
                    payments.insert(c.payment_date(), c.amount().unwrap());
                }
            }
            _ => (),
        });

        //check if all equal
        let first = payments.values().next().unwrap();
        payments.values().for_each(|x| assert_eq!(first, x));

        Ok(())
    }

    #[test]
    fn build_equal_redemptions() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn build_equal_redemptions_with_tenor() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.payment_frequency(), Frequency::Semiannual);
        assert_eq!(instrument.start_date(), start_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn build_zero() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn build_zero_with_tenor() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.rate(), rate);
        assert_eq!(instrument.start_date(), start_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn build_other() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.start_date(), start_date);
        assert_eq!(instrument.end_date(), end_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    fn into_test() -> Result<(), MakeFixedRateLoanError> {
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
            .build()?;

        let builder: MakeFixedRateLoan = instrument.clone().into();
        let instrument2 = builder.build()?;
        assert_eq!(instrument2.notional(), instrument.notional());
        assert_eq!(instrument2.rate(), instrument.rate());

        assert_eq!(instrument2.payment_frequency(), Frequency::Monthly);
        assert_eq!(instrument2.start_date(), start_date);
        assert_eq!(instrument2.end_date(), end_date);

        // instrument2
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));

        Ok(())
    }

    #[test]
    // test the From traint 
    fn from_test(){
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

        let builder: MakeFixedRateLoan =  makefixedrateloan::MakeFixedRateLoan::from(&instrument);
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
