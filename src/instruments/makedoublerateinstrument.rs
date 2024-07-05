use argmin::{core::{CostFunction, Error, Executor}, solver::brent::BrentRoot};

use crate::{
    cashflows::{
        cashflow::{Cashflow, CashflowType, Side}, fixedratecoupon::FixedRateCoupon, floatingratecoupon::FloatingRateCoupon
    }, currencies::enums::Currency, rates::interestrate::{InterestRate, RateDefinition}, time::{calendar::Calendar, calendars::nullcalendar::NullCalendar, 
        date::Date, 
        enums::{BusinessDayConvention, DateGenerationRule, Frequency}, 
        period::Period, schedule::MakeSchedule}, utils::errors::{AtlasError, Result}
};

use super::{instrument::RateType, doublerateinstrument::DoubleRateInstrument, traits::add_cashflows_to_vec};

/// MakeDoubleRateInstrument
/// MakeDoubleRateInstrument is a builder for DoubleRateInstrument struct.
/// Three different types of rates are supported: FixedThenFixed, FixedThenFloating and FloatingThenFixed
/// In the case of floating part, te values "first_part_rate_definition", "first_part_rate" or "second_part_rate_definition" and "second_part_rate" make reference to the spread over the fixing rate
/// In the case of fixed part, theses values make reference to the fixed rate

pub struct MakeDoubleRateInstrument {
    start_date: Option<Date>,
    end_date: Option<Date>,
    change_rate_date: Option<Date>,
    first_coupon_date: Option<Date>,
    payment_frequency: Option<Frequency>,
    tenor: Option<Period>,
    tenor_change_rate: Option<Period>,
    tenor_grace_period: Option<Period>,
    currency: Option<Currency>,
    side: Option<Side>,
    notional: Option<f64>,
    discount_curve_id: Option<usize>,
    forecast_curve_id: Option<usize>,
    rate_type: Option<RateType>,
    first_part_rate_definition: Option<RateDefinition>, 
    first_part_rate: Option<f64>,
    second_part_rate_definition: Option<RateDefinition>,
    second_part_rate: Option<f64>,
    issue_date: Option<Date>,
    calendar: Option<Calendar>,
    business_day_convention: Option<BusinessDayConvention>,
    date_generation_rule: Option<DateGenerationRule>,
    id: Option<String>,
}

impl MakeDoubleRateInstrument {
    pub fn new() -> Self {
        MakeDoubleRateInstrument {
            start_date: None,
            end_date: None,
            change_rate_date: None,
            first_coupon_date: None,
            payment_frequency: None,
            tenor: None,
            tenor_change_rate: None,
            tenor_grace_period: None,
            currency: None,
            side: None,
            notional: None,
            discount_curve_id: None,
            forecast_curve_id: None,
            rate_type: None,
            first_part_rate_definition: None,
            first_part_rate: None,
            second_part_rate_definition: None,
            second_part_rate: None,
            id: None,
            issue_date: None,
            calendar: None,
            business_day_convention: None,
            date_generation_rule: None,
        }
    }

    /// Sets the issue date.
    pub fn with_issue_date(mut self, issue_date: Date) -> MakeDoubleRateInstrument {
        self.issue_date = Some(issue_date);
        self
    }

    /// Sets the first coupon date.
    pub fn with_first_coupon_date(mut self, first_coupon_date: Date) -> MakeDoubleRateInstrument {
        self.first_coupon_date = Some(first_coupon_date);
        self
    }

   /// Sets the currency.
    pub fn with_currency(mut self, currency: Currency) -> MakeDoubleRateInstrument {
       self.currency = Some(currency);
       self
    }

   /// Sets the side.
    pub fn with_side(mut self, side: Side) -> MakeDoubleRateInstrument {
        self.side = Some(side);
        self
    }

    /// Sets the notional.
    pub fn with_notional(mut self, notional: f64) -> MakeDoubleRateInstrument {
        self.notional = Some(notional);
        self
    }

    pub fn with_id(mut self, id: String) -> MakeDoubleRateInstrument {
        self.id = Some(id);
        self
    }

    /// Sets the start date.
    pub fn with_start_date(mut self, start_date: Date) -> MakeDoubleRateInstrument {
        self.start_date = Some(start_date);
        self
    }

    /// Sets the end date.
    pub fn with_end_date(mut self, end_date: Date) -> MakeDoubleRateInstrument {
        self.end_date = Some(end_date);
        self
    }

    /// Sets the discount curve id.
    pub fn with_discount_curve_id(mut self, id: Option<usize>) -> MakeDoubleRateInstrument {
        self.discount_curve_id = id;
        self
    }

    /// Sets the forecast curve id.
    pub fn with_forecast_curve_id(mut self, id: Option<usize>) -> MakeDoubleRateInstrument {
        self.forecast_curve_id = id;
        self
    }

    /// Sets the tenor.
    pub fn with_tenor(mut self, tenor: Period) -> MakeDoubleRateInstrument {
        self.tenor = Some(tenor);
        self
    }

    /// Sets the change rate date.
    pub fn with_tenor_change_rate(mut self, tenor_change_rate: Period) -> MakeDoubleRateInstrument {
        self.tenor_change_rate = Some(tenor_change_rate);
        self
    }

    /// Sets the tenor grace period.
    pub fn with_tenor_grace_period(mut self, tenor_grace_period: Period) -> MakeDoubleRateInstrument {
        self.tenor_grace_period = Some(tenor_grace_period);
        self
    }

    /// Sets the payment frequency.
    pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeDoubleRateInstrument {
        self.payment_frequency = Some(frequency);
        self
    }

    /// Sets the change rate date.
    pub fn with_calendar(mut self, calendar: Calendar) -> MakeDoubleRateInstrument {
        self.calendar = Some(calendar);
        self
    }

    /// Sets the business day convention.
    pub fn with_business_day_convention(mut self, business_day_convention: BusinessDayConvention) -> MakeDoubleRateInstrument {
        self.business_day_convention = Some(business_day_convention);
        self
    }

    /// Sets the date generation rule.
    pub fn with_date_generation_rule(mut self, date_generation_rule: DateGenerationRule) -> MakeDoubleRateInstrument {
        self.date_generation_rule = Some(date_generation_rule);
        self
    }

    /// Sets the rate type.
    pub fn with_rate_type(mut self, rate_type: RateType) -> MakeDoubleRateInstrument {
        self.rate_type = Some(rate_type);
        self
    }

    /// Sets the rate definition for the first part.
    pub fn with_first_part_rate_definition(mut self, rate_definition: RateDefinition) -> MakeDoubleRateInstrument {
        self.first_part_rate_definition = Some(rate_definition);
        self
    }

    /// Sets the rate value for the first part.
    pub fn with_first_part_rate(mut self, rate: f64) -> MakeDoubleRateInstrument {
        self.first_part_rate = Some(rate);
        self
    }

    /// Sets the rate definition for the second part.
    pub fn with_second_part_rate_definition(mut self, rate_definition: RateDefinition) -> MakeDoubleRateInstrument {
        self.second_part_rate_definition = Some(rate_definition);
        self
    }

    /// Sets the rate value for the second part.
    pub fn with_second_part_rate(mut self, rate: f64) -> MakeDoubleRateInstrument {
        self.second_part_rate = Some(rate);
        self
    }

}


impl MakeDoubleRateInstrument {
    pub fn build(self) -> Result<DoubleRateInstrument>{
        // vector to store cashflows
        let mut cashflows = Vec::new();

        // rate_type should be FixedThenFixed, FixedThenFloating or FloatingThenFixed
        let rate_type = self
            .rate_type
            .ok_or(AtlasError::ValueNotSetErr("Rate type is necessary in MakeDoubleRateInstrument".into()))?;

        // Definition of rate use to construct the redemption profile
        let rate = match rate_type {
            RateType::FixedThenFixed =>{
                let rate_definition = self.first_part_rate_definition.ok_or(AtlasError::ValueNotSetErr("Rate definition".into()))?;
                let rate_value = self.first_part_rate.ok_or(AtlasError::ValueNotSetErr("Rate value".into()))?;
                InterestRate::from_rate_definition(rate_value, rate_definition)
            }
            RateType::FixedThenFloating => {
                let rate_definition = self.first_part_rate_definition.ok_or(AtlasError::ValueNotSetErr("Rate definition".into()))?;
                let rate_value = self.first_part_rate.ok_or(AtlasError::ValueNotSetErr("Rate value".into()))?;
                InterestRate::from_rate_definition(rate_value, rate_definition)
            }
            RateType::FloatingThenFixed => {
                let rate_definition = self.second_part_rate_definition.ok_or(AtlasError::ValueNotSetErr("Rate definition".into()))?;
                let rate_value = self.second_part_rate.ok_or(AtlasError::ValueNotSetErr("Rate value".into()))?;
                InterestRate::from_rate_definition(rate_value, rate_definition)
            }
            _ => Err(AtlasError::NotImplementedErr("Rate type in mixed rate instrument".into()))?
        };

        let payment_frequency = self
            .payment_frequency
            .ok_or(AtlasError::ValueNotSetErr("Payment frequency".into()))?;

        let currency = self
            .currency
            .ok_or(AtlasError::ValueNotSetErr("Currency".into()))?;

        let start_date = self
            .start_date
            .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
        
        // end_date is required, if not set, it will be calculated using tenor
        let end_date = match self.end_date {
            Some(date) => date,
            None => {
                let tenor = self
                    .tenor
                    .ok_or(AtlasError::ValueNotSetErr("Tenor or end date is required".into()))?;
                start_date + tenor
            }
        };
                
        // change_rate_date is required, if not set, it will be calculated using tenor_change_rate
        let change_rate_date = match self.change_rate_date {
            Some(date) => date,
            None => {
                let tenor_change_rate = self
                    .tenor_change_rate
                    .ok_or(AtlasError::ValueNotSetErr("Tenor change rate or change rate date is required".into()))?;
                start_date + tenor_change_rate
            },
        };
                
        // schedule builder for period between start date and change rate date
        let mut schedule_builder_first_part = MakeSchedule::new(start_date, change_rate_date)
            .with_frequency(payment_frequency)
            .with_calendar(
                self.calendar.clone()
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

        // first coupon date is calculated using first_coupon_date or tenor_grace_period
        let first_coupon_date = match self.first_coupon_date {
            Some(date) => {
                Some(date)
            }
            None => {
                match self.tenor_grace_period {
                    Some(tenor_grace_period) => {
                        Some(start_date + tenor_grace_period)
                    }
                    None => None
                }
            }
        };

        let schedule_first_part = match first_coupon_date {
            Some(date) => {
                if date > start_date {
                    schedule_builder_first_part.with_first_date(date).build()?
                } else {
                    Err(AtlasError::InvalidValueErr(
                        "First coupon date must be after start date".into(),
                    ))?
                }
            }
            None => schedule_builder_first_part.build()?,
        };

        // schedule builder for period between change rate date and end date
        let mut schedule_builder_second_part = MakeSchedule::new(change_rate_date, end_date)
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

            let schedule_second_part = match self.first_coupon_date {
                Some(date) => {
                    if date > change_rate_date {
                        schedule_builder_second_part.with_first_date(date).build()?
                    } else {
                        Err(AtlasError::InvalidValueErr(
                            "First coupon date must be after change rate date".into(),
                        ))?
                    }
                }
                None => schedule_builder_second_part.build()?,
            };
            
        
            let dates_first_part = schedule_first_part.dates().clone();
            let dates_second_part = schedule_second_part.dates().clone();

            // combine dates from first and second part
            let mut dates = dates_first_part.clone();
            dates.pop();
            dates.extend(dates_second_part.clone());
                
            let notional = self
                .notional
                .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;
            
            let redemptions_raw: Vec<f64> = calculate_equal_payment_redemptions(
                dates.clone(),
                rate,
                notional,
            )?;

            let mut notionals = redemptions_raw.iter().fold(vec![notional], |mut acc, x| {
                acc.push(acc.last().unwrap() - x);
                acc
            });

            notionals.pop();

            let first_part_notionals: Vec<f64> = notionals.iter().take(dates_first_part.len() - 1).cloned().collect();
            let second_part_notionals: Vec<f64> = notionals.iter().skip(dates_first_part.len() - 1).cloned().collect();

            let notional_at_change_rate = second_part_notionals.first().ok_or(AtlasError::ValueNotSetErr("Notional at change rate".into()))?;

            // create coupon cashflows 
            let side = self
                .side
                .ok_or(AtlasError::ValueNotSetErr("Side".into()))?;

            let first_parte_rate = self
                .first_part_rate
                .ok_or(AtlasError::ValueNotSetErr("First part rate".into()))?;
            let first_part_rate_definition = self
                .first_part_rate_definition
                .ok_or(AtlasError::ValueNotSetErr("First part rate definition".into()))?;
            let second_part_rate = self
                .second_part_rate
                .ok_or(AtlasError::ValueNotSetErr("Second part rate".into()))?;
            let second_part_rate_definition = self
                .second_part_rate_definition
                .ok_or(AtlasError::ValueNotSetErr("Second part rate definition".into()))?;

            build_coupons_from_notionals(
                &mut cashflows,
                rate_type,
                &dates_first_part,
                &dates_second_part,
                &first_part_notionals,
                &second_part_notionals,
                first_parte_rate,
                second_part_rate,
                first_part_rate_definition,
                second_part_rate_definition,
                side,
                currency,
            )?;
                
            let first_date = vec![*dates.first().unwrap()];
            add_cashflows_to_vec(
                &mut cashflows,
                &first_date,
                &vec![notional],
                side.inverse(),
                currency,
                CashflowType::Disbursement,
            );

            let mut redemption_dates = vec![];
            let mut disbursement_dates = vec![];
            let mut redemptions = vec![];
            let mut disbursements = vec![];

            let aux_dates: Vec<Date> = dates.iter().skip(1).cloned().collect();
            aux_dates.iter().zip(redemptions_raw.iter()).for_each(|(date, amount)| {
                if *amount >= 0.0 {
                    redemption_dates.push(*date);
                    redemptions.push(*amount);
                } else {
                    disbursement_dates.push(*date);
                    disbursements.push(-*amount);
                }
            });

            add_cashflows_to_vec(
                &mut cashflows,
                &redemption_dates,
                &redemptions,
                side,
                currency,
                CashflowType::Redemption,
            );

            if disbursements.len() > 0 {
                add_cashflows_to_vec(
                    &mut cashflows,
                    &disbursement_dates,
                    &disbursements,
                    side.inverse(),
                    currency,
                    CashflowType::Disbursement,
                );
            }

            match self.discount_curve_id {
                Some(id) => cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_discount_curve_id(id)),
                None => (),
            }

            match self.forecast_curve_id {
                Some(id) => cashflows
                    .iter_mut()
                    .for_each(|cf| cf.set_forecast_curve_id(id)),
                None => (),
            }

            Ok(DoubleRateInstrument::new(
                start_date,
                end_date,
                notional,
                Some(*notional_at_change_rate),
                payment_frequency,
                side,
                currency,
                self.id,
                self.issue_date,  
                change_rate_date,  
                rate_type,
                self.first_part_rate_definition,
                self.first_part_rate,
                self.second_part_rate_definition,
                self.second_part_rate,
                self.forecast_curve_id,
                self.discount_curve_id,
                cashflows,
            ))
        
    }
    
}


fn build_coupons_from_notionals(
    cashflows: &mut Vec<Cashflow>,
    rate_type: RateType,
    dates_first_part: &Vec<Date>,
    dates_second_part: &Vec<Date>,
    first_part_notionals: &Vec<f64>,
    second_part_notionals: &Vec<f64>,
    first_part_rate: f64,
    second_part_rate: f64,
    first_part_rate_definition: RateDefinition,
    second_part_rate_definition: RateDefinition,
    side: Side,
    currency: Currency,
) -> Result<()> {

    if dates_first_part.len() + dates_second_part.len() - 2 != first_part_notionals.len() + second_part_notionals.len() {
        Err(AtlasError::InvalidValueErr(
            "Dates and notionals must have the same length".to_string(),
        ))?;
    }
    if dates_first_part.len() + dates_second_part.len() < 3 {
        Err(AtlasError::InvalidValueErr(
            "Dates must have at least two elements".to_string(),
        ))?;
    }

    for (date_pair, notional) in dates_first_part.windows(2).zip(first_part_notionals.iter()) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];

        match rate_type {
            RateType::FixedThenFixed => {
                let rate = InterestRate::from_rate_definition(first_part_rate, first_part_rate_definition);
                let coupon = FixedRateCoupon::new(*notional, rate, d1, d2, d2, currency, side);
                cashflows.push(Cashflow::FixedRateCoupon(coupon));
            }
            RateType::FixedThenFloating => {
                let rate = InterestRate::from_rate_definition(first_part_rate, first_part_rate_definition);
                let coupon = FixedRateCoupon::new(*notional, rate, d1, d2, d2, currency, side);
                cashflows.push(Cashflow::FixedRateCoupon(coupon));
            }
            RateType::FloatingThenFixed => {
                let coupon = FloatingRateCoupon::new(*notional,first_part_rate,d1,d2,d2,Some(d1),first_part_rate_definition,currency,side);
                cashflows.push(Cashflow::FloatingRateCoupon(coupon));
            }
            _ => Err(AtlasError::NotImplementedErr("Rate type".into()))?,
        }
    }

    for (date_pair, notional) in dates_second_part.windows(2).zip(second_part_notionals.iter()) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];

        match rate_type {
            RateType::FixedThenFixed => {
                let rate = InterestRate::from_rate_definition(second_part_rate, second_part_rate_definition);
                let coupon = FixedRateCoupon::new(*notional, rate, d1, d2, d2, currency, side);
                cashflows.push(Cashflow::FixedRateCoupon(coupon));
            }
            RateType::FixedThenFloating => {
                let coupon = FloatingRateCoupon::new(*notional,second_part_rate,d1,d2,d2,Some(d1),second_part_rate_definition,currency,side);
                cashflows.push(Cashflow::FloatingRateCoupon(coupon));
            }
            RateType::FloatingThenFixed => {
                let rate = InterestRate::from_rate_definition(second_part_rate, second_part_rate_definition);
                let coupon = FixedRateCoupon::new(*notional, rate, d1, d2, d2, currency, side);
                cashflows.push(Cashflow::FixedRateCoupon(coupon));
            }
            _ => Err(AtlasError::NotImplementedErr("Rate type".into()))?,
        }
    }

    Ok(())
}

struct EqualPaymentCost {
    dates: Vec<Date>,
    rate: InterestRate,
}

impl CostFunction for EqualPaymentCost {
    type Param = f64;
    type Output = f64;
    fn cost(&self, payment: &Self::Param) -> std::result::Result<Self::Output, Error> {
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

//  function to calculate equal payment redemptions, always returns a vector of positive values 
fn calculate_equal_payment_redemptions(
    dates: Vec<Date>,
    rate: InterestRate,
    notional: f64,
) -> Result<Vec<f64>> {
    let cost = EqualPaymentCost {
        dates: dates.clone(),
        rate: rate,
    };
    let (min, max) = (-0.2 , 1.5 );
    let solver = BrentRoot::new(min, max, 1e-6);

    let init_param = 1.0 / (dates.len() as f64);
    let res = Executor::new(cost, solver)
        .configure(|state| state.param(init_param).max_iters(100).target_cost(0.0))
        .run()?;

    let payment = res
        .state()
        .best_param
        .ok_or(AtlasError::EvaluationErr("Solver failed".into()))?
        * notional;

    let mut redemptions = Vec::new();
    let mut total_amount = notional;
    
    for date_pair in dates.windows(2) {
        let d1 = date_pair[0];
        let d2 = date_pair[1];
        let interest = total_amount * (rate.compound_factor(d1, d2) - 1.0);
        let k = payment - interest;
        total_amount -= k;
        redemptions.push(k);
    }
    Ok(redemptions)
}

#[cfg(test)]
mod test {
    use crate::{cashflows::{cashflow::{Cashflow, Side}, traits::RequiresFixingRate}, currencies::enums::Currency, instruments::{instrument::RateType, makedoublerateinstrument::MakeDoubleRateInstrument}, rates::interestrate::RateDefinition, time::{date::Date, enums::{Frequency, TimeUnit}, period::Period}, utils::errors::Result, visitors::traits::HasCashflows};


    #[test]
    fn build_fixed_then_floating_instrument() -> Result<()> {
        let start_date = Date::new(2020, 1, 1);

        let rate_type = RateType::FixedThenFloating;
        let first_part_rate_definition = RateDefinition::default();
        let first_part_rate = 0.05;
        let second_part_rate_definition = RateDefinition::default();
        let second_part_rate = 0.02;

        let mut instrument = MakeDoubleRateInstrument::new()
            .with_start_date(start_date)
            .with_tenor(Period::new(9, TimeUnit::Years))
            .with_tenor_change_rate(Period::new(4, TimeUnit::Years))
            .with_tenor_grace_period(Period::new(2, TimeUnit::Years))
            .with_rate_type(rate_type)
            .with_first_part_rate_definition(first_part_rate_definition)
            .with_first_part_rate(first_part_rate)
            .with_second_part_rate_definition(second_part_rate_definition)
            .with_second_part_rate(second_part_rate)
            .with_payment_frequency(Frequency::Semiannual)
            .with_notional(100.0)
            .with_side(Side::Receive)
            .with_currency(Currency::USD)
            .build()?;

        instrument.mut_cashflows().iter_mut().for_each(|cf| cf.set_fixing_rate(0.03));
        
        instrument.cashflows().iter().for_each(|cf| 
            match cf {
                Cashflow::FixedRateCoupon(coupon) => {
                    assert!((coupon.rate().rate()-0.05).abs() < 1e-6 );  
                }
                Cashflow::FloatingRateCoupon(coupon) => {
                    assert!((coupon.spread() - 0.02).abs() < 1e-6);
                }
                _ => ()
            });

        Ok(())
    }

}