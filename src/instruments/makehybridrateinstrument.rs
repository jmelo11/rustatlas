// use std::collections::{HashMap, HashSet};

// use crate::{
//     cashflows::{
//         cashflow::{Cashflow, Side},
//         floatingratecoupon::FloatingRateCoupon,
//         simplecashflow::SimpleCashflow,
//         traits::{InterestAccrual, Payable},
//     },
//     core::traits::HasCurrency,
//     currencies::enums::Currency,
//     rates::interestrate::RateDefinition,
//     time::{date::Date, enums::Frequency, period::Period, schedule::MakeSchedule},
//     utils::errors::{AtlasError, Result},
//     visitors::traits::HasCashflows,
// };

// use super::{
//     floatingrateinstrument::FloatingRateInstrument,
//     instrument::RateType,
//     traits::{build_cashflows, calculate_outstanding, notionals_vector, Structure},
// };

// pub struct MakeMixedRateInstrument {
//     start_date: Option<Date>,
//     end_date: Option<Date>,
//     first_coupon_date: Option<Date>,
//     payment_frequency: Option<Frequency>,
//     tenor: Option<Period>,
//     currency: Option<Currency>,
//     side: Option<Side>,
//     notional: Option<f64>,
//     structure: Option<Structure>,
//     discount_curve_id: Option<usize>,
//     forecast_curve_id: Option<usize>,
//     disbursements: Option<HashMap<Date, f64>>,
//     redemptions: Option<HashMap<Date, f64>>,
//     additional_coupon_dates: Option<HashSet<Date>>,
//     first_part_rate_type: Option<RateType>,
//     first_part_rate_definition: Option<RateDefinition>,
//     first_part_rate: Option<f64>,
//     second_part_rate_type: Option<RateType>,
//     second_part_rate_definition: Option<RateDefinition>,
//     second_part_rate: Option<f64>,
//     id: Option<usize>,
//     issue_date: Option<Date>,
// }

// impl MakeMixedRateInstrument {
//     pub fn new() -> Self {
//         MakeMixedRateInstrument {
//             start_date: None,
//             end_date: None,
//             first_coupon_date: None,
//             payment_frequency: None,
//             tenor: None,
//             currency: None,
//             side: None,
//             notional: None,
//             structure: None,
//             discount_curve_id: None,
//             forecast_curve_id: None,
//             disbursements: None,
//             redemptions: None,
//             additional_coupon_dates: None,
//             first_part_rate_type: None,
//             first_part_rate_definition: None,
//             first_part_rate: None,
//             second_part_rate_type: None,
//             second_part_rate_definition: None,
//             second_part_rate: None,
//             id: None,
//             issue_date: None,
//         }
//     }

//     pub fn with_issue_date(mut self, issue_date: Date) -> MakeMixedRateInstrument {
//         self.issue_date = Some(issue_date);
//         self
//     }

//     /// Sets the first coupon date.
//     pub fn with_first_coupon_date(mut self, first_coupon_date: Date) -> MakeMixedRateInstrument {
//         self.first_coupon_date = Some(first_coupon_date);
//         self
//     }

//     /// Sets the currency.
//     pub fn with_currency(mut self, currency: Currency) -> MakeMixedRateInstrument {
//         self.currency = Some(currency);
//         self
//     }

//     /// Sets the side.
//     pub fn with_side(mut self, side: Side) -> MakeMixedRateInstrument {
//         self.side = Some(side);
//         self
//     }

//     /// Sets the notional.
//     pub fn with_notional(mut self, notional: f64) -> MakeMixedRateInstrument {
//         self.notional = Some(notional);
//         self
//     }

//     pub fn with_id(mut self, id: Option<usize>) -> MakeMixedRateInstrument {
//         self.id = id;
//         self
//     }

//     /// Sets the start date.
//     pub fn with_start_date(mut self, start_date: Date) -> MakeMixedRateInstrument {
//         self.start_date = Some(start_date);
//         self
//     }

//     /// Sets the end date.
//     pub fn with_end_date(mut self, end_date: Date) -> MakeMixedRateInstrument {
//         self.end_date = Some(end_date);
//         self
//     }

//     /// Sets the disbursements.
//     pub fn with_disbursements(
//         mut self,
//         disbursements: HashMap<Date, f64>,
//     ) -> MakeMixedRateInstrument {
//         self.disbursements = Some(disbursements);
//         self
//     }

//     /// Sets the redemptions.
//     pub fn with_redemptions(mut self, redemptions: HashMap<Date, f64>) -> MakeMixedRateInstrument {
//         self.redemptions = Some(redemptions);
//         self
//     }

//     /// Sets the additional coupon dates.
//     pub fn with_additional_coupon_dates(
//         mut self,
//         additional_coupon_dates: HashSet<Date>,
//     ) -> MakeMixedRateInstrument {
//         self.additional_coupon_dates = Some(additional_coupon_dates);
//         self
//     }

//     /// Sets the discount curve id.
//     pub fn with_discount_curve_id(mut self, id: Option<usize>) -> MakeMixedRateInstrument {
//         self.discount_curve_id = id;
//         self
//     }

//     /// Sets the tenor.
//     pub fn with_tenor(mut self, tenor: Period) -> MakeMixedRateInstrument {
//         self.tenor = Some(tenor);
//         self
//     }

//     /// Sets the payment frequency.
//     pub fn with_payment_frequency(mut self, frequency: Frequency) -> MakeMixedRateInstrument {
//         self.payment_frequency = Some(frequency);
//         self
//     }

//     /// Sets the structure.
//     pub fn with_structure(mut self, structure: Structure) -> MakeMixedRateInstrument {
//         self.structure = Some(structure);
//         self
//     }
// }
