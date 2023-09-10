use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        floatingratecoupon::FloatingRateCoupon,
        simplecashflow::SimpleCashflow,
        traits::{InterestAccrual, Payable},
    },
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{
        date::Date,
        enums::Frequency,
        period::Period,
        schedule::{MakeSchedule, MakeScheduleError},
    },
    visitors::traits::HasCashflows,
};

use super::{
    floatingrateinstrument::FloatingRateInstrument,
    traits::{build_cashflows, calculate_outstanding, notionals_vector, CashflowType, Structure},
};

/// # MakeFloatingRateLoan
/// Builder for a floating rate loan.
#[derive(Debug, Clone)]
pub struct MakeFloatingRateLoan {
    start_date: Option<Date>,
    end_date: Option<Date>,
    payment_frequency: Option<Frequency>,
    tenor: Option<Period>,
    rate_definition: Option<RateDefinition>,
    notional: Option<f64>,
    currency: Option<Currency>,
    side: Option<Side>,
    spread: Option<f64>,
    structure: Option<Structure>,
    disbursements: Option<HashMap<Date, f64>>,
    redemptions: Option<HashMap<Date, f64>>,
    additional_coupon_dates: Option<HashSet<Date>>,
    forecast_curve_id: Option<usize>,
    discount_curve_id: Option<usize>,
}

/// # MakeFloatingRateLoanError
/// Describes the errors that can occur when building a floating rate loan.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum MakeFloatingRateLoanError {
    /// Start date not set.
    #[error("Start date not set")]
    StartDateNotSet,
    /// End date not set.
    #[error("End date not set")]
    EndDateNotSet,
    /// Tenor not set.
    #[error("Tenor not set")]
    TenorNotSet,
    /// Payment frequency not set.
    #[error("Payment frequency not set")]
    PaymentFrequencyNotSet,
    /// Disbursements not set.
    #[error("Disbursements not set")]
    DisbursementsNotSet,
    /// Redemptions not set.
    #[error("Redemptions not set")]
    RedemptionsNotSet,
    /// Additional coupon dates not set.
    #[error("Additional coupon dates not set")]
    AdditionalCouponDatesNotSet,
    /// Notional not set.
    #[error("Notional not set")]
    NotionalNotSet,
    /// Spread not set.
    #[error("Spread not set")]
    SpreadNotSet,
    /// Currency not set.
    #[error("Currency not set")]
    CurrencyNotSet,
    /// Structure not set.
    #[error("Structure not set")]
    StructureNotSet,
    /// Rate definition not set.
    #[error("Rate definition not set")]
    RateDefinitionNotSet,
    /// Side not set.
    #[error("Side not set")]
    SideNotSet,
    /// Redemptions and disbursements do not match.
    #[error("Redemptions and disbursements do not match")]
    RedemptionsAndDisbursementsDoNotMatch,
    /// Invalid Structure.
    #[error("Invalid structure")]
    InvalidStructure,
    /// Schedule build error.
    #[error("Schedule build error: {0}")]
    ScheduleBuildError(String),
}

impl From<MakeScheduleError> for MakeFloatingRateLoanError {
    fn from(e: MakeScheduleError) -> Self {
        MakeFloatingRateLoanError::ScheduleBuildError(format!("{}", e))
    }
}

/// Constructor, setters and getters.
impl MakeFloatingRateLoan {
    pub fn new() -> MakeFloatingRateLoan {
        MakeFloatingRateLoan {
            start_date: None,
            end_date: None,
            payment_frequency: None,
            tenor: None,
            rate_definition: None,
            notional: None,
            spread: None,
            currency: None,
            side: None,
            structure: None,
            forecast_curve_id: None,
            discount_curve_id: None,
            disbursements: None,
            redemptions: None,
            additional_coupon_dates: None,
        }
    }

    pub fn with_start_date(mut self, start_date: Date) -> MakeFloatingRateLoan {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Date) -> MakeFloatingRateLoan {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_tenor(mut self, tenor: Period) -> MakeFloatingRateLoan {
        self.tenor = Some(tenor);
        return self;
    }

    pub fn with_disbursements(mut self, disbursements: HashMap<Date, f64>) -> MakeFloatingRateLoan {
        self.disbursements = Some(disbursements);
        self
    }

    pub fn with_redemptions(mut self, redemptions: HashMap<Date, f64>) -> MakeFloatingRateLoan {
        self.redemptions = Some(redemptions);
        self
    }

    pub fn with_additional_coupon_dates(
        mut self,
        additional_coupon_dates: HashSet<Date>,
    ) -> MakeFloatingRateLoan {
        self.additional_coupon_dates = Some(additional_coupon_dates);
        self
    }

    pub fn with_forecast_curve_id(
        mut self,
        forecast_curve_id: Option<usize>,
    ) -> MakeFloatingRateLoan {
        self.forecast_curve_id = forecast_curve_id;
        return self;
    }

    pub fn with_discount_curve_id(
        mut self,
        discount_curve_id: Option<usize>,
    ) -> MakeFloatingRateLoan {
        self.discount_curve_id = discount_curve_id;
        return self;
    }

    pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> MakeFloatingRateLoan {
        self.rate_definition = Some(rate_definition);
        return self;
    }

    pub fn with_notional(mut self, notional: f64) -> MakeFloatingRateLoan {
        self.notional = Some(notional);
        return self;
    }

    pub fn with_currency(mut self, currency: Currency) -> MakeFloatingRateLoan {
        self.currency = Some(currency);
        return self;
    }

    pub fn with_spread(mut self, spread: f64) -> MakeFloatingRateLoan {
        self.spread = Some(spread);
        return self;
    }

    pub fn bullet(mut self) -> MakeFloatingRateLoan {
        self.structure = Some(Structure::Bullet);
        return self;
    }

    pub fn equal_redemptions(mut self) -> MakeFloatingRateLoan {
        self.structure = Some(Structure::EqualRedemptions);
        self
    }

    pub fn zero(mut self) -> MakeFloatingRateLoan {
        self.structure = Some(Structure::Zero);
        self.payment_frequency = Some(Frequency::Once);
        self
    }

    pub fn other(mut self) -> MakeFloatingRateLoan {
        self.structure = Some(Structure::Other);
        self.payment_frequency = Some(Frequency::OtherFrequency);
        self
    }

    pub fn with_side(mut self, side: Side) -> MakeFloatingRateLoan {
        self.side = Some(side);
        return self;
    }

    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeFloatingRateLoan {
        self.payment_frequency = Some(frequency);
        return self;
    }

    pub fn with_structure(mut self, structure: Structure) -> MakeFloatingRateLoan {
        self.structure = Some(structure);
        return self;
    }
}

/// Build
impl MakeFloatingRateLoan {
    pub fn build(self) -> Result<FloatingRateInstrument, MakeFloatingRateLoanError> {
        let mut cashflows = Vec::new();
        let structure = self
            .structure
            .ok_or(MakeFloatingRateLoanError::StructureNotSet)?;
        let rate_definition = self
            .rate_definition
            .ok_or(MakeFloatingRateLoanError::RateDefinitionNotSet)?;
        let spread = self.spread.ok_or(MakeFloatingRateLoanError::SpreadNotSet)?;
        let currency = self
            .currency
            .ok_or(MakeFloatingRateLoanError::CurrencyNotSet)?;
        let side = self.side.ok_or(MakeFloatingRateLoanError::SideNotSet)?;
        let payment_frequency = self
            .payment_frequency
            .ok_or(MakeFloatingRateLoanError::PaymentFrequencyNotSet)?;
        match structure {
            Structure::Bullet => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(MakeFloatingRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFloatingRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;
                let notional = self
                    .notional
                    .ok_or(MakeFloatingRateLoanError::NotionalNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                // end common

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
                    &schedule.dates(),
                    &notionals,
                    spread,
                    rate_definition,
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

                cashflows.iter_mut().for_each(|cf| {
                    cf.set_discount_curve_id(self.discount_curve_id);
                    cf.set_forecast_curve_id(self.forecast_curve_id);
                });
                Ok(FloatingRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    spread,
                    side,
                    cashflows,
                    payment_frequency,
                    rate_definition,
                    structure,
                    currency,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                ))
            }
            Structure::Zero => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(MakeFloatingRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFloatingRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;
                let notional = self
                    .notional
                    .ok_or(MakeFloatingRateLoanError::NotionalNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                // end common

                let notionals =
                    notionals_vector(schedule.dates().len() - 1, notional, Structure::Zero);
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
                    &schedule.dates(),
                    &notionals,
                    spread,
                    rate_definition,
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

                cashflows.iter_mut().for_each(|cf| {
                    cf.set_discount_curve_id(self.discount_curve_id);
                    cf.set_forecast_curve_id(self.forecast_curve_id);
                });
                Ok(FloatingRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    spread,
                    side,
                    cashflows,
                    payment_frequency,
                    rate_definition,
                    structure,
                    currency,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                ))
            }
            Structure::EqualRedemptions => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(MakeFloatingRateLoanError::StartDateNotSet)?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self.tenor.ok_or(MakeFloatingRateLoanError::TenorNotSet)?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .build()?;
                let notional = self
                    .notional
                    .ok_or(MakeFloatingRateLoanError::NotionalNotSet)?;
                let inv_side = match side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                // end common

                let n = schedule.dates().len() - 1;
                let notionals = notionals_vector(n, notional, Structure::EqualRedemptions);
                let redemptions = vec![notional / n as f64; n];

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
                    &schedule.dates(),
                    &notionals,
                    spread,
                    rate_definition,
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
                cashflows.iter_mut().for_each(|cf| {
                    cf.set_discount_curve_id(self.discount_curve_id);
                    cf.set_forecast_curve_id(self.forecast_curve_id);
                });

                Ok(FloatingRateInstrument::new(
                    start_date,
                    end_date,
                    notional,
                    spread,
                    side,
                    cashflows,
                    payment_frequency,
                    rate_definition,
                    structure,
                    currency,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                ))
            }
            Structure::Other => {
                let disbursements = self
                    .disbursements
                    .ok_or(MakeFloatingRateLoanError::DisbursementsNotSet)?;
                let redemptions = self
                    .redemptions
                    .ok_or(MakeFloatingRateLoanError::RedemptionsNotSet)?;
                let notional = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                let redemption = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                if redemption != notional {
                    Err(MakeFloatingRateLoanError::RedemptionsAndDisbursementsDoNotMatch)?;
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
                    let coupon = FloatingRateCoupon::new(
                        *notional,
                        spread,
                        *start_date,
                        *end_date,
                        *end_date,
                        rate_definition,
                        currency,
                        side,
                    );
                    cashflows.push(Cashflow::FloatingRateCoupon(coupon));
                }

                for (date, amount) in redemptions.iter() {
                    let cashflow = Cashflow::Redemption(
                        SimpleCashflow::new(*date, currency, side).with_amount(*amount),
                    );
                    cashflows.push(cashflow);
                }
                let start_date = &timeline.first().expect("No start date").0;
                let end_date = &timeline.last().expect("No end date").1;
                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");

                cashflows.iter_mut().for_each(|cf| {
                    cf.set_discount_curve_id(self.discount_curve_id);
                    cf.set_forecast_curve_id(self.forecast_curve_id);
                });
                Ok(FloatingRateInstrument::new(
                    *start_date,
                    *end_date,
                    notional,
                    spread,
                    side,
                    cashflows,
                    payment_frequency,
                    rate_definition,
                    structure,
                    currency,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                ))
            }
            _ => Err(MakeFloatingRateLoanError::InvalidStructure)?,
        }
    }
}

fn build_coupons_from_notionals(
    cashflows: &mut Vec<Cashflow>,
    dates: &Vec<Date>,
    notionals: &Vec<f64>,
    spread: f64,
    rate_definition: RateDefinition,
    side: Side,
    currency: Currency,
) {
    for (date_pair, notional) in dates.windows(2).zip(notionals) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];
        let coupon = FloatingRateCoupon::new(
            *notional,
            spread,
            d1,
            d2,
            d2,
            rate_definition,
            currency,
            side,
        );
        cashflows.push(Cashflow::FloatingRateCoupon(coupon));
    }
}

impl Into<MakeFloatingRateLoan> for FloatingRateInstrument {
    fn into(self) -> MakeFloatingRateLoan {
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
                        Cashflow::FloatingRateCoupon(c) => {
                            additional_coupon_dates.insert(c.accrual_start_date());
                            additional_coupon_dates.insert(c.accrual_end_date());
                        }
                        _ => (),
                    }
                }
                MakeFloatingRateLoan::new()
                    .with_start_date(self.start_date())
                    .with_end_date(self.end_date())
                    .with_notional(self.notional())
                    .with_spread(self.spread())
                    .with_side(self.side())
                    .with_rate_definition(self.rate_definition())
                    .with_disbursements(disbursements)
                    .with_redemptions(redemptions)
                    .with_additional_coupon_dates(additional_coupon_dates)
                    .with_forecast_curve_id(self.forecast_curve_id())
                    .with_discount_curve_id(self.discount_curve_id())
                    .with_payment_frequency(self.payment_frequency())
                    .with_currency(self.currency())
                    .other()
            }
            _ => MakeFloatingRateLoan::new()
                .with_start_date(self.start_date())
                .with_end_date(self.end_date())
                .with_notional(self.notional())
                .with_spread(self.spread())
                .with_side(self.side())
                .with_payment_frequency(self.payment_frequency())
                .with_rate_definition(self.rate_definition())
                .with_forecast_curve_id(self.forecast_curve_id())
                .with_discount_curve_id(self.discount_curve_id())
                .with_currency(self.currency())
                .with_structure(self.structure()),
        }
    }
}

impl From<&FloatingRateInstrument> for MakeFloatingRateLoan {
    fn from(instrument: &FloatingRateInstrument) -> MakeFloatingRateLoan {
        instrument.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        cashflows::{cashflow::Side, traits::RequiresFixingRate},
        currencies::enums::Currency,
        rates::{enums::Compounding, interestrate::RateDefinition},
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
        visitors::traits::HasCashflows,
    };

    use super::MakeFloatingRateLoanError;

    #[test]
    fn build_bullet() -> Result<(), MakeFloatingRateLoanError> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_spread(0.05)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()?;

        instrument
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_fixing_rate(0.002));
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
    fn build_zero() -> Result<(), MakeFloatingRateLoanError> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate_definition(rate_definition)
            .with_spread(0.05)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .zero()
            .build()?;

        instrument
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_fixing_rate(0.002));
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
    fn build_equal_redemptions() -> Result<(), MakeFloatingRateLoanError> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateLoan::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_spread(0.05)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_redemptions()
            .build()?;

        instrument
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_fixing_rate(0.002));
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
    fn build_equal_redemptions_with_tenor() -> Result<(), MakeFloatingRateLoanError> {
        let start_date = Date::new(2020, 1, 1);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateLoan::new()
            .with_start_date(start_date)
            .with_tenor(Period::new(23, TimeUnit::Months))
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_spread(0.05)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .equal_redemptions()
            .build()?;

        instrument
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_fixing_rate(0.002));
        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.start_date(), start_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));
        Ok(())
    }

    #[test]
    fn build_other() -> Result<(), MakeFloatingRateLoanError> {
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

        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateLoan::new()
            .with_start_date(start_date)
            .with_disbursements(disbursements)
            .with_redemptions(redemptions)
            .with_additional_coupon_dates(additional_coupon_dates)
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_spread(0.05)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .other()
            .build()?;

        instrument
            .mut_cashflows()
            .iter_mut()
            .for_each(|cf| cf.set_fixing_rate(0.002));
        assert_eq!(instrument.notional(), 100.0);
        assert_eq!(instrument.start_date(), start_date);

        // instrument
        //     .cashflows()
        //     .iter()
        //     .for_each(|cf| println!("{}", cf));
        Ok(())
    }
}
