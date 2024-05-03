use std::collections::{HashMap, HashSet};

use crate::{
    cashflows::{
        cashflow::{Cashflow, CashflowType, Side},
        floatingratecoupon::FloatingRateCoupon,
        simplecashflow::SimpleCashflow,
    },
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
};

use super::{
    instrument::RateType,
    leg::Leg,
    traits::{add_cashflows_to_vec, calculate_outstanding, notionals_vector, Structure},
};

/// # MakeFloatingRateLeg
/// Builder for a floating rate loan.
#[derive(Debug, Clone)]
pub struct MakeFloatingRateLeg {
    start_date: Option<Date>,
    end_date: Option<Date>,
    first_coupon_date: Option<Date>,
    payment_frequency: Option<Frequency>,
    tenor: Option<Period>,
    rate_definition: Option<RateDefinition>,
    notional: Option<f64>,
    currency: Option<Currency>,
    side: Option<Side>,
    end_of_month: Option<bool>,
    spread: Option<f64>,
    structure: Option<Structure>,
    disbursements: Option<HashMap<Date, f64>>,
    redemptions: Option<HashMap<Date, f64>>,
    additional_coupon_dates: Option<HashSet<Date>>,
    forecast_curve_id: Option<usize>,
    discount_curve_id: Option<usize>,
    issue_date: Option<Date>,
    calendar: Option<Calendar>,
    business_day_convention: Option<BusinessDayConvention>,
    date_generation_rule: Option<DateGenerationRule>,
}

/// Constructor, setters and getters.
impl MakeFloatingRateLeg {
    pub fn new() -> MakeFloatingRateLeg {
        MakeFloatingRateLeg {
            start_date: None,
            end_date: None,
            first_coupon_date: None,
            payment_frequency: None,
            tenor: None,
            rate_definition: None,
            notional: None,
            end_of_month: None,
            spread: None,
            currency: None,
            side: None,
            structure: None,
            forecast_curve_id: None,
            discount_curve_id: None,
            disbursements: None,
            redemptions: None,
            additional_coupon_dates: None,
            issue_date: None,
            calendar: None,
            business_day_convention: None,
            date_generation_rule: None,
        }
    }

    pub fn with_end_of_month(mut self, end_of_month: Option<bool>) -> MakeFloatingRateLeg {
        self.end_of_month = end_of_month;
        self
    }

    pub fn with_calendar(mut self, calendar: Option<Calendar>) -> MakeFloatingRateLeg {
        self.calendar = calendar;
        self
    }

    pub fn with_business_day_convention(
        mut self,
        business_day_convention: Option<BusinessDayConvention>,
    ) -> MakeFloatingRateLeg {
        self.business_day_convention = business_day_convention;
        self
    }

    pub fn with_date_generation_rule(
        mut self,
        date_generation_rule: Option<DateGenerationRule>,
    ) -> MakeFloatingRateLeg {
        self.date_generation_rule = date_generation_rule;
        self
    }

    pub fn with_issue_date(mut self, issue_date: Date) -> MakeFloatingRateLeg {
        self.issue_date = Some(issue_date);
        self
    }

    pub fn with_first_coupon_date(
        mut self,
        first_coupon_date: Option<Date>,
    ) -> MakeFloatingRateLeg {
        self.first_coupon_date = first_coupon_date;
        self
    }

    pub fn with_start_date(mut self, start_date: Date) -> MakeFloatingRateLeg {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Date) -> MakeFloatingRateLeg {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_tenor(mut self, tenor: Period) -> MakeFloatingRateLeg {
        self.tenor = Some(tenor);
        return self;
    }

    pub fn with_disbursements(mut self, disbursements: HashMap<Date, f64>) -> MakeFloatingRateLeg {
        self.disbursements = Some(disbursements);
        self
    }

    pub fn with_redemptions(mut self, redemptions: HashMap<Date, f64>) -> MakeFloatingRateLeg {
        self.redemptions = Some(redemptions);
        self
    }

    pub fn with_additional_coupon_dates(
        mut self,
        additional_coupon_dates: HashSet<Date>,
    ) -> MakeFloatingRateLeg {
        self.additional_coupon_dates = Some(additional_coupon_dates);
        self
    }

    pub fn with_forecast_curve_id(
        mut self,
        forecast_curve_id: Option<usize>,
    ) -> MakeFloatingRateLeg {
        self.forecast_curve_id = forecast_curve_id;
        return self;
    }

    pub fn with_discount_curve_id(
        mut self,
        discount_curve_id: Option<usize>,
    ) -> MakeFloatingRateLeg {
        self.discount_curve_id = discount_curve_id;
        return self;
    }

    pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> MakeFloatingRateLeg {
        self.rate_definition = Some(rate_definition);
        return self;
    }

    pub fn with_notional(mut self, notional: f64) -> MakeFloatingRateLeg {
        self.notional = Some(notional);
        return self;
    }

    pub fn with_currency(mut self, currency: Currency) -> MakeFloatingRateLeg {
        self.currency = Some(currency);
        return self;
    }

    pub fn with_spread(mut self, spread: f64) -> MakeFloatingRateLeg {
        self.spread = Some(spread);
        return self;
    }

    pub fn bullet(mut self) -> MakeFloatingRateLeg {
        self.structure = Some(Structure::Bullet);
        return self;
    }

    pub fn equal_redemptions(mut self) -> MakeFloatingRateLeg {
        self.structure = Some(Structure::EqualRedemptions);
        self
    }

    pub fn zero(mut self) -> MakeFloatingRateLeg {
        self.structure = Some(Structure::Zero);
        self.payment_frequency = Some(Frequency::Once);
        self
    }

    pub fn other(mut self) -> MakeFloatingRateLeg {
        self.structure = Some(Structure::Other);
        self.payment_frequency = Some(Frequency::OtherFrequency);
        self
    }

    pub fn with_side(mut self, side: Side) -> MakeFloatingRateLeg {
        self.side = Some(side);
        return self;
    }

    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeFloatingRateLeg {
        self.payment_frequency = Some(frequency);
        return self;
    }

    pub fn with_structure(mut self, structure: Structure) -> MakeFloatingRateLeg {
        self.structure = Some(structure);
        return self;
    }
}

/// Build
impl MakeFloatingRateLeg {
    pub fn build(self) -> Result<Leg> {
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
                    .end_of_month(self.end_of_month.unwrap_or(false))
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

                Ok(Leg::new(
                    structure,
                    RateType::Floating,
                    spread,
                    rate_definition,
                    currency,
                    side,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                    cashflows,
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

                Ok(Leg::new(
                    structure,
                    RateType::Floating,
                    spread,
                    rate_definition,
                    currency,
                    side,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                    cashflows,
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
                    .end_of_month(self.end_of_month.unwrap_or(false))
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

                Ok(Leg::new(
                    structure,
                    RateType::Floating,
                    spread,
                    rate_definition,
                    currency,
                    side,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                    cashflows,
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
                Ok(Leg::new(
                    structure,
                    RateType::Floating,
                    spread,
                    rate_definition,
                    currency,
                    side,
                    self.discount_curve_id,
                    self.forecast_curve_id,
                    cashflows,
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
