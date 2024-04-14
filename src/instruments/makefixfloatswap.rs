use std::collections::{HashMap, HashSet};

use crate::{
    cashflows::cashflow::Side,
    currencies::enums::Currency,
    rates::interestrate::{InterestRate, RateDefinition},
    time::{
        calendar::Calendar,
        date::Date,
        enums::{BusinessDayConvention, Frequency},
        period::Period,
    },
    utils::errors::{AtlasError, Result},
    visitors::traits::HasCashflows,
};

use super::{
    fixfloatswap::FixFloatSwap, makefixedrateinstrument::MakeFixedRateInstrument,
    makefloatingrateinstrument::MakeFloatingRateInstrument, traits::Structure,
};

pub struct MakeFixFloatSwap {
    start_date: Option<Date>,
    end_date: Option<Date>,
    fixed_first_coupon_date: Option<Date>,
    floating_first_coupon_date: Option<Date>,
    fixed_leg_payment_frequency: Option<Frequency>,
    floating_leg_payment_frequency: Option<Frequency>,
    fixed_leg_tenor: Option<Period>,
    floating_leg_tenor: Option<Period>,
    currency: Option<Currency>,
    side: Option<Side>,
    notional: Option<f64>,
    structure: Option<Structure>,
    rate: Option<InterestRate>,
    spread: Option<f64>,
    discount_curve_id: Option<usize>,
    forecast_curve_id: Option<usize>,

    fixed_leg_disbursements: Option<HashMap<Date, f64>>,
    fixed_leg_redemptions: Option<HashMap<Date, f64>>,
    fixed_leg_additional_coupon_dates: Option<HashSet<Date>>,

    floating_leg_disbursements: Option<HashMap<Date, f64>>,
    floating_leg_redemptions: Option<HashMap<Date, f64>>,
    floating_leg_additional_coupon_dates: Option<HashSet<Date>>,

    spread_rate_definition: Option<RateDefinition>,

    id: Option<String>,
    issue_date: Option<Date>,
    calendar: Option<Calendar>,
    business_day_convention: Option<BusinessDayConvention>,
}

impl MakeFixFloatSwap {
    pub fn new() -> Self {
        MakeFixFloatSwap {
            start_date: None,
            end_date: None,
            fixed_first_coupon_date: None,
            floating_first_coupon_date: None,
            fixed_leg_payment_frequency: None,
            floating_leg_payment_frequency: None,
            fixed_leg_tenor: None,
            floating_leg_tenor: None,
            currency: None,
            side: None,
            notional: None,
            structure: None,
            rate: None,
            spread: None,
            discount_curve_id: None,
            forecast_curve_id: None,
            fixed_leg_disbursements: None,
            fixed_leg_redemptions: None,
            fixed_leg_additional_coupon_dates: None,
            floating_leg_disbursements: None,
            floating_leg_redemptions: None,
            floating_leg_additional_coupon_dates: None,
            spread_rate_definition: None,
            id: None,
            issue_date: None,
            calendar: None,
            business_day_convention: None,
        }
    }

    pub fn with_start_date(mut self, start_date: Date) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: Date) -> Self {
        self.end_date = Some(end_date);
        self
    }

    pub fn with_fixed_first_coupon_date(mut self, fixed_first_coupon_date: Date) -> Self {
        self.fixed_first_coupon_date = Some(fixed_first_coupon_date);
        self
    }

    pub fn with_floating_first_coupon_date(mut self, floating_first_coupon_date: Date) -> Self {
        self.floating_first_coupon_date = Some(floating_first_coupon_date);
        self
    }

    pub fn with_fixed_leg_payment_frequency(
        mut self,
        fixed_leg_payment_frequency: Frequency,
    ) -> Self {
        self.fixed_leg_payment_frequency = Some(fixed_leg_payment_frequency);
        self
    }

    pub fn with_floating_leg_payment_frequency(
        mut self,
        floating_leg_payment_frequency: Frequency,
    ) -> Self {
        self.floating_leg_payment_frequency = Some(floating_leg_payment_frequency);
        self
    }

    pub fn with_fixed_leg_tenor(mut self, fixed_leg_tenor: Period) -> Self {
        self.fixed_leg_tenor = Some(fixed_leg_tenor);
        self
    }

    pub fn with_floating_leg_tenor(mut self, floating_leg_tenor: Period) -> Self {
        self.floating_leg_tenor = Some(floating_leg_tenor);
        self
    }

    pub fn with_currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn with_side(mut self, side: Side) -> Self {
        self.side = Some(side);
        self
    }

    pub fn with_notional(mut self, notional: f64) -> Self {
        self.notional = Some(notional);
        self
    }

    pub fn with_structure(mut self, structure: Structure) -> Self {
        self.structure = Some(structure);
        self
    }

    pub fn with_rate(mut self, rate: InterestRate) -> Self {
        self.rate = Some(rate);
        self
    }

    pub fn with_spread(mut self, spread: f64) -> Self {
        self.spread = Some(spread);
        self
    }

    pub fn with_discount_curve_id(mut self, discount_curve_id: Option<usize>) -> Self {
        self.discount_curve_id = discount_curve_id;
        self
    }

    pub fn with_forecast_curve_id(mut self, forecast_curve_id: Option<usize>) -> Self {
        self.forecast_curve_id = forecast_curve_id;
        self
    }

    pub fn with_fixed_leg_disbursements(
        mut self,
        fixed_leg_disbursements: HashMap<Date, f64>,
    ) -> Self {
        self.fixed_leg_disbursements = Some(fixed_leg_disbursements);
        self
    }

    pub fn with_fixed_leg_redemptions(mut self, fixed_leg_redemptions: HashMap<Date, f64>) -> Self {
        self.fixed_leg_redemptions = Some(fixed_leg_redemptions);
        self
    }

    pub fn with_fixed_leg_additional_coupon_dates(
        mut self,
        fixed_leg_additional_coupon_dates: HashSet<Date>,
    ) -> Self {
        self.fixed_leg_additional_coupon_dates = Some(fixed_leg_additional_coupon_dates);
        self
    }

    pub fn with_floating_leg_disbursements(
        mut self,
        floating_leg_disbursements: HashMap<Date, f64>,
    ) -> Self {
        self.floating_leg_disbursements = Some(floating_leg_disbursements);
        self
    }

    pub fn with_floating_leg_redemptions(
        mut self,
        floating_leg_redemptions: HashMap<Date, f64>,
    ) -> Self {
        self.floating_leg_redemptions = Some(floating_leg_redemptions);
        self
    }

    pub fn with_floating_leg_additional_coupon_dates(
        mut self,
        floating_leg_additional_coupon_dates: HashSet<Date>,
    ) -> Self {
        self.floating_leg_additional_coupon_dates = Some(floating_leg_additional_coupon_dates);
        self
    }

    pub fn with_spread_rate_definition(mut self, spread_rate_definition: RateDefinition) -> Self {
        self.spread_rate_definition = Some(spread_rate_definition);
        self
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_issue_date(mut self, issue_date: Date) -> Self {
        self.issue_date = Some(issue_date);
        self
    }

    pub fn with_calendar(mut self, calendar: Calendar) -> Self {
        self.calendar = Some(calendar);
        self
    }

    pub fn with_business_day_convention(
        mut self,
        business_day_convention: BusinessDayConvention,
    ) -> Self {
        self.business_day_convention = Some(business_day_convention);
        self
    }

    pub fn build(self) -> Result<FixFloatSwap> {
        let start_date = self
            .start_date
            .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
        let end_date = self
            .end_date
            .ok_or(AtlasError::ValueNotSetErr("End date".into()))?;
        let fixed_leg_payment_frequency =
            self.fixed_leg_payment_frequency
                .ok_or(AtlasError::ValueNotSetErr(
                    "Fixed leg payment frequency".into(),
                ))?;
        let floating_leg_payment_frequency =
            self.floating_leg_payment_frequency
                .ok_or(AtlasError::ValueNotSetErr(
                    "Floating leg payment frequency".into(),
                ))?;
        let currency = self
            .currency
            .ok_or(AtlasError::ValueNotSetErr("Currency".into()))?;
        let side = self.side.ok_or(AtlasError::ValueNotSetErr("Side".into()))?;
        let notional = self
            .notional
            .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;
        let structure = self
            .structure
            .ok_or(AtlasError::ValueNotSetErr("Structure".into()))?;
        let rate = self.rate.ok_or(AtlasError::ValueNotSetErr("Rate".into()))?;
        let spread = self
            .spread
            .ok_or(AtlasError::ValueNotSetErr("Spread".into()))?;
        let spread_rate_definition = self
            .spread_rate_definition
            .ok_or(AtlasError::ValueNotSetErr("Spread rate definition".into()))?;

        let fixed_leg = MakeFixedRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(fixed_leg_payment_frequency)
            .with_rate(rate)
            .with_notional(notional)
            .with_side(side)
            .with_currency(currency)
            .with_structure(structure)
            .with_discount_curve_id(self.discount_curve_id)
            .with_first_coupon_date(self.fixed_first_coupon_date)
            .with_issue_date(self.issue_date.unwrap_or(start_date))
            .with_calendar(self.calendar.clone())
            .with_business_day_convention(self.business_day_convention)
            .build()?;

        let floating_leg = MakeFloatingRateInstrument::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_payment_frequency(floating_leg_payment_frequency)
            .with_rate_definition(spread_rate_definition)
            .with_notional(notional)
            .with_spread(spread)
            .with_side(side.inverse())
            .with_currency(currency)
            .with_structure(structure)
            .with_discount_curve_id(self.discount_curve_id)
            .with_forecast_curve_id(self.forecast_curve_id)
            .with_first_coupon_date(self.floating_first_coupon_date)
            .with_issue_date(self.issue_date.unwrap_or(start_date))
            .with_calendar(self.calendar.clone())
            .with_business_day_convention(self.business_day_convention)
            .build()?;

        Ok(FixFloatSwap::new(
            fixed_leg.cashflows().to_vec(),
            floating_leg.cashflows().to_vec(),
            structure,
            currency,
            side,
            rate,
            spread,
            notional,
            start_date,
            end_date,
            Some(self.issue_date.unwrap_or(start_date)),
            spread_rate_definition,
            self.discount_curve_id,
            self.forecast_curve_id,
            self.id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cashflows::cashflow::Side,
        currencies::enums::Currency,
        rates::{enums::Compounding, interestrate::InterestRate},
        time::daycounter::DayCounter,
    };

    #[test]
    fn test_make_fix_float_swap() -> Result<()> {
        let start_date = Date::new(2023, 6, 1);
        let end_date = Date::new(2028, 6, 1);
        let fixed_rate = InterestRate::new(
            0.05,
            Compounding::Compounded,
            Frequency::Semiannual,
            DayCounter::Thirty360,
        );
        let spread_rate_definition = RateDefinition::new(
            DayCounter::Actual360,
            Compounding::Compounded,
            Frequency::Quarterly,
        );
        let notional = 1_000_000.0;
        let spread = 0.01;

        let fix_float_swap = MakeFixFloatSwap::new()
            .with_start_date(start_date)
            .with_end_date(end_date)
            .with_fixed_leg_payment_frequency(Frequency::Semiannual)
            .with_floating_leg_payment_frequency(Frequency::Quarterly)
            .with_currency(Currency::USD)
            .with_side(Side::Receive)
            .with_notional(notional)
            .with_structure(Structure::Bullet)
            .with_rate(fixed_rate)
            .with_spread(spread)
            .with_spread_rate_definition(spread_rate_definition)
            .with_discount_curve_id(Some(1))
            .with_forecast_curve_id(Some(2))
            .build()?;

        assert_eq!(fix_float_swap.start_date(), start_date);
        assert_eq!(fix_float_swap.end_date(), end_date);
        assert_eq!(fix_float_swap.currency(), Currency::USD);
        assert_eq!(fix_float_swap.side(), Side::Receive);
        assert_eq!(fix_float_swap.notional(), notional);
        assert_eq!(fix_float_swap.structure(), Structure::Bullet);
        assert_eq!(fix_float_swap.rate(), fixed_rate);
        assert_eq!(fix_float_swap.spread(), spread);
        assert_eq!(
            fix_float_swap.spread_rate_definition(),
            spread_rate_definition
        );
        assert_eq!(fix_float_swap.discount_curve_id(), Some(1));
        assert_eq!(fix_float_swap.forecast_curve_id(), Some(2));

        Ok(())
    }
}
