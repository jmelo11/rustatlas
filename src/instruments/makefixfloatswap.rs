// use crate::{
//     cashflows::{
//         cashflow::{Cashflow, Side},
//         fixedratecoupon::FixedRateCoupon,
//         floatingratecoupon::FloatingRateCoupon,
//         simplecashflow::SimpleCashflow,
//     },
//     currencies::enums::Currency,
//     rates::interestrate::{InterestRate, RateDefinition},
//     time::{date::Date, enums::Frequency, schedule::MakeSchedule},
//     utils::errors::{AtlasError, Result},
// };

// use super::fixfloatswap::FixFloatSwap;

// pub struct MakeFixFloatSwap {
//     fixed_rate: Option<InterestRate>,
//     spread: Option<f64>,

//     currency: Option<Currency>,
//     notional: Option<f64>,
//     start_date: Option<Date>,
//     end_date: Option<Date>,
//     fix_leg_frequency: Option<Frequency>,
//     floating_leg_frequency: Option<Frequency>,
//     rate_definition: Option<RateDefinition>,
//     side: Option<Side>,
//     forecast_curve: Option<usize>,
//     discount_curve: Option<usize>,
//     id: Option<usize>,
// }

// impl MakeFixFloatSwap {
//     pub fn new() -> Self {
//         Self {
//             fixed_rate: None,
//             currency: None,
//             notional: None,
//             start_date: None,
//             end_date: None,
//             fix_leg_frequency: None,
//             floating_leg_frequency: None,
//             rate_definition: None,
//             spread: None,
//             side: None,
//             forecast_curve: None,
//             discount_curve: None,
//             id: None,
//         }
//     }

//     pub fn with_id(mut self, id: Option<usize>) -> Self {
//         self.id = id;
//         self
//     }

//     pub fn with_spread(mut self, spread: f64) -> Self {
//         self.spread = Some(spread);
//         self
//     }

//     pub fn with_fixed_rate(mut self, fixed_rate: InterestRate) -> Self {
//         self.fixed_rate = Some(fixed_rate);
//         self
//     }

//     pub fn with_rate_definition(mut self, rate_definition: RateDefinition) -> Self {
//         self.rate_definition = Some(rate_definition);
//         self
//     }

//     pub fn with_currency(mut self, currency: Currency) -> Self {
//         self.currency = Some(currency);
//         self
//     }

//     pub fn with_notional(mut self, notional: f64) -> Self {
//         self.notional = Some(notional);
//         self
//     }

//     pub fn with_start_date(mut self, start_date: Date) -> Self {
//         self.start_date = Some(start_date);
//         self
//     }

//     pub fn with_end_date(mut self, end_date: Date) -> Self {
//         self.end_date = Some(end_date);
//         self
//     }

//     pub fn with_fixed_leg_frequency(mut self, fix_leg_frequency: Frequency) -> Self {
//         self.fix_leg_frequency = Some(fix_leg_frequency);
//         self
//     }

//     pub fn with_floating_leg_frequency(mut self, floating_leg_frequency: Frequency) -> Self {
//         self.floating_leg_frequency = Some(floating_leg_frequency);
//         self
//     }

//     pub fn with_side(mut self, side: Side) -> Self {
//         self.side = Some(side);
//         self
//     }

//     pub fn with_forecast_curve(mut self, forecast_curve: usize) -> Self {
//         self.forecast_curve = Some(forecast_curve);
//         self
//     }

//     pub fn with_discount_curve(mut self, discount_curve: usize) -> Self {
//         self.discount_curve = Some(discount_curve);
//         self
//     }

//     pub fn build(self) -> Result<FixFloatSwap> {
//         let fixed_rate = self
//             .fixed_rate
//             .ok_or(AtlasError::ValueNotSetErr("Fixed rate".into()))?;
//         let currency = self
//             .currency
//             .ok_or(AtlasError::ValueNotSetErr("Currency".into()))?;
//         let notional = self
//             .notional
//             .ok_or(AtlasError::ValueNotSetErr("Notional".into()))?;
//         let start_date = self
//             .start_date
//             .ok_or(AtlasError::ValueNotSetErr("Start date".into()))?;
//         let side = self.side.ok_or(AtlasError::ValueNotSetErr("Side".into()))?;

//         let fix_leg_frequency = self
//             .fix_leg_frequency
//             .ok_or(AtlasError::ValueNotSetErr("Fixed leg frequency".into()))?;

//         let floating_leg_frequency = self
//             .floating_leg_frequency
//             .ok_or(AtlasError::ValueNotSetErr("Floating leg frequency".into()))?;

//         let end_date = self
//             .end_date
//             .ok_or(AtlasError::ValueNotSetErr("End date".into()))?;

//         let fix_leg_schedule = MakeSchedule::new(start_date, end_date)
//             .with_frequency(fix_leg_frequency)
//             .build()?;

//         let floating_leg_schedule = MakeSchedule::new(start_date, end_date)
//             .with_frequency(floating_leg_frequency)
//             .build()?;

//         let spread = self.spread.unwrap_or(0.0);

//         let rate_definition = self
//             .rate_definition
//             .ok_or(AtlasError::ValueNotSetErr("Rate definition".into()))?;

//         let mut fix_cashflows = Vec::new();

//         for date_pair in fix_leg_schedule.dates().windows(2) {
//             let accrual_start_date = date_pair[0];
//             let accrual_end_date = date_pair[1];
//             let coupon = FixedRateCoupon::new(
//                 notional,
//                 fixed_rate,
//                 accrual_start_date,
//                 accrual_end_date,
//                 accrual_end_date,
//                 currency,
//                 side,
//             );
//             fix_cashflows.push(Cashflow::FixedRateCoupon(coupon));
//         }

//         let redemption = SimpleCashflow::new(end_date, currency, side).with_amount(notional);
//         fix_cashflows.push(Cashflow::Redemption(redemption));

//         let mut float_cashflows = Vec::new();

//         let inv_side = match side {
//             Side::Pay => Side::Receive,
//             Side::Receive => Side::Pay,
//         };

//         match self.discount_curve {
//             Some(id) => fix_cashflows.iter_mut().for_each(|cf| {
//                 cf.set_discount_curve_id(id);
//             }),
//             None => (),
//         }

//         for date_pair in floating_leg_schedule.dates().windows(2) {
//             let accrual_start_date = date_pair[0];
//             let accrual_end_date = date_pair[1];
//             let coupon = FloatingRateCoupon::new(
//                 notional,
//                 spread,
//                 accrual_start_date,
//                 accrual_end_date,
//                 accrual_end_date,
//                 rate_definition,
//                 currency,
//                 inv_side,
//             );
//             float_cashflows.push(Cashflow::FloatingRateCoupon(coupon));
//         }

//         match self.forecast_curve {
//             Some(id) => float_cashflows.iter_mut().for_each(|cf| {
//                 cf.set_forecast_curve_id(id);
//             }),
//             None => (),
//         }

//         let redemption = SimpleCashflow::new(end_date, currency, side).with_amount(notional);
//         float_cashflows.push(Cashflow::Redemption(redemption));

//         Ok(FixFloatSwap::new(fix_cashflows, float_cashflows))
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         currencies::enums::Currency,
//         rates::{enums::Compounding, interestrate::InterestRate},
//         time::{date::Date, daycounter::DayCounter, enums::Frequency},
//     };

//     #[test]
//     fn test_make_fix_float_swap() -> Result<()> {
//         let start_date = Date::new(2021, 1, 1);
//         let end_date = Date::new(2025, 1, 1);
//         let notional = 100.0;
//         let fixed_rate = InterestRate::new(
//             0.05,
//             Compounding::Compounded,
//             Frequency::Annual,
//             DayCounter::Actual360,
//         );
//         let rate_definition = RateDefinition::default();
//         let currency = Currency::USD;

//         let fix_leg_frequency = Frequency::Semiannual;
//         let floating_leg_frequency = Frequency::Quarterly;

//         let _ = MakeFixFloatSwap::new()
//             .with_start_date(start_date)
//             .with_end_date(end_date)
//             .with_fixed_rate(fixed_rate)
//             .with_currency(currency)
//             .with_notional(notional)
//             .with_fixed_leg_frequency(fix_leg_frequency)
//             .with_floating_leg_frequency(floating_leg_frequency)
//             .with_side(Side::Pay)
//             .with_rate_definition(rate_definition)
//             .build()?;

//         Ok(())
//     }
// }
