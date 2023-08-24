// use crate::{
//     cashflows::{
//         cashflow::SimpleCashflow,
//         enums::{Cashflow, Side},
//         fixedratecoupon::FixedRateCoupon,
//     },
//     currencies::enums::Currency,
//     rates::interestrate::InterestRate,
//     time::{date::Date, enums::Frequency, schedule::Schedule},
// };

// pub struct FixedRateInstrument {
//     start_date: Date,
//     end_date: Date,
//     payment_frequency: Frequency,
//     rate: InterestRate,
//     notional: f64,
//     currency: Currency,
//     cashflows: Vec<Cashflow>,
// }

// pub fn as_bullet(
//     start_date: Date,
//     end_date: Date,
//     payment_frequency: Frequency,
//     rate: InterestRate,
//     notional: f64,
//     discount_curve_id: usize,
//     currency: Currency,
//     side: Side,
// ) -> FixedRateInstrument {
//     let schedule =
//         Schedule::generate_schedule_with_frequency(start_date, end_date, payment_frequency);
//     let dates = schedule.dates();
//     let mut cashflows = Vec::new();

//     let flip_side = match side {
//         Side::Receive => Side::Pay,
//         Side::Pay => Side::Receive,
//     };
//     let redemption =
//         SimpleCashflow::new(notional, start_date, discount_curve_id, currency, flip_side);
//     cashflows.push(Cashflow::Disbursement(redemption));
//     for i in 0..dates.len() - 1 {
//         let coupon = FixedRateCoupon::new(
//             notional,
//             rate,
//             dates[i],
//             dates[i + 1],
//             dates[i + 1],
//             discount_curve_id,
//             currency,
//             side,
//         );
//         cashflows.push(Cashflow::FixedRateCoupon(coupon));
//     }
//     let redemption = SimpleCashflow::new(notional, end_date, discount_curve_id, currency, side);
//     cashflows.push(Cashflow::Redemption(redemption));

//     FixedRateInstrument {
//         start_date,
//         end_date,
//         payment_frequency,
//         rate,
//         notional,
//         currency,
//         cashflows,
//     }
// }
