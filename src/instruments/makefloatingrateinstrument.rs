use std::collections::{HashMap, HashSet};

use crate::{
    cashflows::{
        cashflow::{Cashflow, CashflowType, Side},
        floatingratecoupon::FloatingRateCoupon,
        simplecashflow::SimpleCashflow,
        traits::{InterestAccrual, Payable},
    },
    core::traits::HasCurrency,
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{
        calendar::Calendar,
        calendars::nullcalendar::NullCalendar,
        date::Date,
        enums::{BusinessDayConvention, DateGenerationRule, Frequency},
        period::Period,
        schedule::MakeSchedule,
    },
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};

use super::{
    floatingrateinstrument::FloatingRateInstrument,
    traits::{add_cashflows_to_vec, calculate_outstanding, notionals_vector, Structure},
};

/// # MakeFloatingRateInstrument
/// Builder for a floating rate loan.
#[derive(Debug, Clone)]
pub struct MakeFloatingRateInstrument {
    start_date: Option<Date>,
    end_date: Option<Date>,
    first_coupon_date: Option<Date>,
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
    id: Option<String>,
    issue_date: Option<Date>,
    calendar: Option<Calendar>,
    business_day_convention: Option<BusinessDayConvention>,
    date_generation_rule: Option<DateGenerationRule>,
}

/// Constructor, setters and getters.
impl MakeFloatingRateInstrument {
    pub fn new() -> MakeFloatingRateInstrument {
        MakeFloatingRateInstrument {
            start_date: None,
            end_date: None,
            first_coupon_date: None,
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
            id: None,
            issue_date: None,
            calendar: None,
            business_day_convention: None,
            date_generation_rule: None,
        }
    }

    pub fn with_calendar(mut self, calendar: Option<Calendar>) -> MakeFloatingRateInstrument {
        self.calendar = calendar;
        self
    }

    pub fn with_business_day_convention(
        mut self,
        business_day_convention: Option<BusinessDayConvention>,
    ) -> MakeFloatingRateInstrument {
        self.business_day_convention = business_day_convention;
        self
    }

    pub fn with_date_generation_rule(
        mut self,
        date_generation_rule: Option<DateGenerationRule>,
    ) -> MakeFloatingRateInstrument {
        self.date_generation_rule = date_generation_rule;
        self
    }

    pub fn with_issue_date(mut self, issue_date: Date) -> MakeFloatingRateInstrument {
        self.issue_date = Some(issue_date);
        self
    }

    pub fn with_id(mut self, id: Option<String>) -> MakeFloatingRateInstrument {
        self.id = id;
        self
    }

    pub fn with_first_coupon_date(
        mut self,
        first_coupon_date: Option<Date>,
    ) -> MakeFloatingRateInstrument {
        self.first_coupon_date = first_coupon_date;
        self
    }

    pub fn with_start_date(mut self, start_date: Date) -> MakeFloatingRateInstrument {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Date) -> MakeFloatingRateInstrument {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_tenor(mut self, tenor: Period) -> MakeFloatingRateInstrument {
        self.tenor = Some(tenor);
        return self;
    }

    pub fn with_disbursements(
        mut self,
        disbursements: HashMap<Date, f64>,
    ) -> MakeFloatingRateInstrument {
        self.disbursements = Some(disbursements);
        self
    }

    pub fn with_redemptions(
        mut self,
        redemptions: HashMap<Date, f64>,
    ) -> MakeFloatingRateInstrument {
        self.redemptions = Some(redemptions);
        self
    }

    pub fn with_additional_coupon_dates(
        mut self,
        additional_coupon_dates: HashSet<Date>,
    ) -> MakeFloatingRateInstrument {
        self.additional_coupon_dates = Some(additional_coupon_dates);
        self
    }

    pub fn with_forecast_curve_id(
        mut self,
        forecast_curve_id: Option<usize>,
    ) -> MakeFloatingRateInstrument {
        self.forecast_curve_id = forecast_curve_id;
        return self;
    }

    pub fn with_discount_curve_id(
        mut self,
        discount_curve_id: Option<usize>,
    ) -> MakeFloatingRateInstrument {
        self.discount_curve_id = discount_curve_id;
        return self;
    }

    pub fn with_rate_definition(
        mut self,
        rate_definition: RateDefinition,
    ) -> MakeFloatingRateInstrument {
        self.rate_definition = Some(rate_definition);
        return self;
    }

    pub fn with_notional(mut self, notional: f64) -> MakeFloatingRateInstrument {
        self.notional = Some(notional);
        return self;
    }

    pub fn with_currency(mut self, currency: Currency) -> MakeFloatingRateInstrument {
        self.currency = Some(currency);
        return self;
    }

    pub fn with_spread(mut self, spread: f64) -> MakeFloatingRateInstrument {
        self.spread = Some(spread);
        return self;
    }

    pub fn bullet(mut self) -> MakeFloatingRateInstrument {
        self.structure = Some(Structure::Bullet);
        return self;
    }

    pub fn equal_redemptions(mut self) -> MakeFloatingRateInstrument {
        self.structure = Some(Structure::EqualRedemptions);
        self
    }

    pub fn zero(mut self) -> MakeFloatingRateInstrument {
        self.structure = Some(Structure::Zero);
        self.payment_frequency = Some(Frequency::Once);
        self
    }

    pub fn other(mut self) -> MakeFloatingRateInstrument {
        self.structure = Some(Structure::Other);
        self.payment_frequency = Some(Frequency::OtherFrequency);
        self
    }

    pub fn with_side(mut self, side: Side) -> MakeFloatingRateInstrument {
        self.side = Some(side);
        return self;
    }

    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeFloatingRateInstrument {
        self.payment_frequency = Some(frequency);
        return self;
    }

    pub fn with_structure(mut self, structure: Structure) -> MakeFloatingRateInstrument {
        self.structure = Some(structure);
        return self;
    }
}

/// Build
impl MakeFloatingRateInstrument {
    pub fn build(self) -> Result<FloatingRateInstrument> {
        let mut cashflows = Vec::new();
        let structure = self
            .structure
            .ok_or(AtlasError::ValueNotSetErr("Structure".into()))?;
        let rate_definition = self
            .rate_definition
            .ok_or(AtlasError::ValueNotSetErr("Rate definition".into()))?;
        let spread = self
            .spread
            .ok_or(AtlasError::ValueNotSetErr("Spread".into()))?;
        let currency = self
            .currency
            .ok_or(AtlasError::ValueNotSetErr("Currency".into()))?;
        let side = self.side.ok_or(AtlasError::ValueNotSetErr("Side".into()))?;
        let payment_frequency = self
            .payment_frequency
            .ok_or(AtlasError::ValueNotSetErr("Payment frequency".into()))?;
        match structure {
            Structure::Bullet => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self
                            .tenor
                            .ok_or(AtlasError::ValueNotSetErr("Tenor".into()))?;
                        start_date + tenor
                    }
                };
                let mut schedule_builder = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .with_calendar(
                        self.calendar
                            .unwrap_or(Calendar::NullCalendar(NullCalendar::new())),
                    )
                    .with_convention(
                        self.business_day_convention
                            .unwrap_or(BusinessDayConvention::Unadjusted),
                    )
                    .with_rule(
                        self.date_generation_rule
                            .unwrap_or(DateGenerationRule::Backward),
                    );

                let schedule = match self.first_coupon_date {
                    Some(date) => {
                        if date > start_date {
                            schedule_builder.with_first_date(date).build()?
                        } else {
                            Err(AtlasError::InvalidValueErr(
                                "First coupon date must be after start date".into(),
                            ))?
                        }
                    }
                    None => schedule_builder.build()?,
                };

                let notional = self
                    .notional
                    .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;

                // end common
                let notionals =
                    notionals_vector(schedule.dates().len() - 1, notional, Structure::Bullet);
                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];

                add_cashflows_to_vec(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    side.inverse(),
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
                add_cashflows_to_vec(
                    &mut cashflows,
                    &last_date,
                    &vec![notional],
                    side,
                    currency,
                    CashflowType::Redemption,
                );

                match self.discount_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_discount_curve_id(id);
                    }),
                    None => (),
                }
                match self.forecast_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_forecast_curve_id(id);
                    }),
                    None => (),
                }

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
                    self.id,
                    self.issue_date,
                ))
            }
            Structure::Zero => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self
                            .tenor
                            .ok_or(AtlasError::ValueNotSetErr("Tenor".into()))?;
                        start_date + tenor
                    }
                };
                let schedule = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .with_calendar(
                        self.calendar
                            .unwrap_or(Calendar::NullCalendar(NullCalendar::new())),
                    )
                    .with_convention(
                        self.business_day_convention
                            .unwrap_or(BusinessDayConvention::Unadjusted),
                    )
                    .with_rule(
                        self.date_generation_rule
                            .unwrap_or(DateGenerationRule::Backward),
                    )
                    .build()?;
                let notional = self
                    .notional
                    .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;

                // end common

                let notionals =
                    notionals_vector(schedule.dates().len() - 1, notional, Structure::Zero);
                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];

                add_cashflows_to_vec(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    side.inverse(),
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
                add_cashflows_to_vec(
                    &mut cashflows,
                    &last_date,
                    &vec![notional],
                    side,
                    currency,
                    CashflowType::Redemption,
                );

                match self.discount_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_discount_curve_id(id);
                    }),
                    None => (),
                }
                match self.forecast_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_forecast_curve_id(id);
                    }),
                    None => (),
                }

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
                    self.id,
                    self.issue_date,
                ))
            }
            Structure::EqualRedemptions => {
                // common
                let start_date = self
                    .start_date
                    .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
                let end_date = match self.end_date {
                    Some(date) => date,
                    None => {
                        let tenor = self
                            .tenor
                            .ok_or(AtlasError::ValueNotSetErr("Tenor".into()))?;
                        start_date + tenor
                    }
                };
                let mut schedule_builder = MakeSchedule::new(start_date, end_date)
                    .with_frequency(payment_frequency)
                    .with_calendar(
                        self.calendar
                            .unwrap_or(Calendar::NullCalendar(NullCalendar::new())),
                    )
                    .with_convention(
                        self.business_day_convention
                            .unwrap_or(BusinessDayConvention::Unadjusted),
                    )
                    .with_rule(
                        self.date_generation_rule
                            .unwrap_or(DateGenerationRule::Backward),
                    );

                let schedule = match self.first_coupon_date {
                    Some(date) => {
                        if date > start_date {
                            schedule_builder.with_first_date(date).build()?
                        } else {
                            Err(AtlasError::InvalidValueErr(
                                "First coupon date must be after start date".into(),
                            ))?
                        }
                    }
                    None => schedule_builder.build()?,
                };

                let notional = self
                    .notional
                    .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;

                // end common

                let n = schedule.dates().len() - 1;
                let notionals = notionals_vector(n, notional, Structure::EqualRedemptions);
                let redemptions = vec![notional / n as f64; n];

                let first_date = vec![*schedule.dates().first().unwrap()];

                add_cashflows_to_vec(
                    &mut cashflows,
                    &first_date,
                    &vec![notional],
                    side.inverse(),
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
                add_cashflows_to_vec(
                    &mut cashflows,
                    &redemption_dates,
                    &redemptions,
                    side,
                    currency,
                    CashflowType::Redemption,
                );
                match self.discount_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_discount_curve_id(id);
                    }),
                    None => (),
                }
                match self.forecast_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_forecast_curve_id(id);
                    }),
                    None => (),
                }

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
                    self.id,
                    self.issue_date,
                ))
            }
            Structure::Other => {
                let disbursements = self
                    .disbursements
                    .ok_or(AtlasError::ValueNotSetErr("Disbursements".into()))?;
                let redemptions = self
                    .redemptions
                    .ok_or(AtlasError::ValueNotSetErr("Redemptions".into()))?;
                let notional = disbursements.values().fold(0.0, |acc, x| acc + x).abs();
                let redemption = redemptions.values().fold(0.0, |acc, x| acc + x).abs();
                if (notional - redemption).abs() > 0.000001 {
                    Err(AtlasError::InvalidValueErr(
                        "Redemption amount must equal disbursement amount".into(),
                    ))?;
                }

                let additional_dates = self.additional_coupon_dates.unwrap_or_default();

                let timeline =
                    calculate_outstanding(&disbursements, &redemptions, &additional_dates);

                for (date, amount) in disbursements.iter() {
                    let cashflow = Cashflow::Disbursement(
                        SimpleCashflow::new(*date, currency, side.inverse()).with_amount(*amount),
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
                        Some(*start_date),
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

                let start_date = &timeline
                    .first()
                    .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?
                    .0;
                let end_date = &timeline
                    .last()
                    .ok_or(AtlasError::ValueNotSetErr("End date".into()))?
                    .1;

                let payment_frequency = self.payment_frequency.expect("Payment frequency not set");

                match self.discount_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_discount_curve_id(id);
                    }),
                    None => (),
                }
                match self.forecast_curve_id {
                    Some(id) => cashflows.iter_mut().for_each(|cf| {
                        cf.set_forecast_curve_id(id);
                    }),
                    None => (),
                }
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
                    self.id,
                    self.issue_date,
                ))
            }
            _ => Err(AtlasError::InvalidValueErr(
                "Invalid structure for floating rate loan".into(),
            ))?,
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
            Some(d1),
            rate_definition,
            currency,
            side,
        );
        cashflows.push(Cashflow::FloatingRateCoupon(coupon));
    }
}

impl Into<MakeFloatingRateInstrument> for FloatingRateInstrument {
    fn into(self) -> MakeFloatingRateInstrument {
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
                    additional_coupon_dates.insert(c.accrual_start_date().unwrap());
                    additional_coupon_dates.insert(c.accrual_end_date().unwrap());
                }
                _ => (),
            }
        }

        MakeFloatingRateInstrument::new()
            .with_start_date(self.start_date())
            .with_end_date(self.end_date())
            .with_notional(self.notional())
            .with_spread(self.spread())
            .with_side(self.side())
            .with_rate_definition(self.rate_definition())
            .with_forecast_curve_id(self.forecast_curve_id())
            .with_discount_curve_id(self.discount_curve_id())
            .with_payment_frequency(self.payment_frequency())
            .with_currency(self.currency().unwrap())
            .with_disbursements(disbursements)
            .with_redemptions(redemptions)
            .with_additional_coupon_dates(additional_coupon_dates)
            .other()

    }
}

impl From<&FloatingRateInstrument> for MakeFloatingRateInstrument {
    fn from(instrument: &FloatingRateInstrument) -> MakeFloatingRateInstrument {
        instrument.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{
        cashflows::{cashflow::Side, traits::RequiresFixingRate}, core::traits::HasCurrency, currencies::enums::Currency, instruments::{makefloatingrateinstrument::MakeFloatingRateInstrument, traits::Structure}, rates::{enums::Compounding, interestrate::RateDefinition}, time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        }, utils::errors::Result, visitors::traits::HasCashflows
    };

    #[test]
    fn build_bullet() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = MakeFloatingRateInstrument::new()
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

        Ok(())
    }

    #[test]
    fn build_zero() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = super::MakeFloatingRateInstrument::new()
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

        Ok(())
    }

    #[test]
    fn build_equal_redemptions() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = MakeFloatingRateInstrument::new()
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

        Ok(())
    }

    #[test]
    fn build_equal_redemptions_with_tenor() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );

        let mut instrument = MakeFloatingRateInstrument::new()
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

        Ok(())
    }

    #[test]
    fn build_other() -> Result<()> {
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

        let mut instrument = MakeFloatingRateInstrument::new()
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

        Ok(())
    }

    #[test]
    fn into_test_1() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);
        let end_date = start_date + Period::new(5, TimeUnit::Years);
        let rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Annual,
        );
        let notional = 100.0;

        let instrument = MakeFloatingRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_rate_definition(rate_definition)
            .with_payment_frequency(Frequency::Semiannual)
            .with_spread(0.05)
            .with_notional(notional)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .bullet()
            .build()?;

        let builder = MakeFloatingRateInstrument::from(&instrument);
        let instrument2 = builder.build()?;

        assert_eq!(instrument2.notional(), instrument.notional());
        assert_eq!(instrument2.start_date(), instrument.start_date());
        assert_eq!(instrument2.end_date(), instrument.end_date());
        assert_eq!(instrument2.rate_definition(), instrument.rate_definition());
        assert_ne!(instrument2.payment_frequency(), instrument.payment_frequency());
        assert_eq!(instrument2.spread(), instrument.spread());
        assert_eq!(instrument2.side(), instrument.side());
        assert_eq!(instrument2.currency().unwrap(), instrument.currency().unwrap());
        assert_eq!(instrument2.discount_curve_id(), instrument.discount_curve_id());
        assert_eq!(instrument2.forecast_curve_id(), instrument.forecast_curve_id());
        assert_eq!(instrument2.structure(), Structure::Other);
        assert_eq!(instrument.structure(), Structure::Bullet);

        Ok(())

    }

}
