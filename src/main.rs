mod cashflows;
mod core;
mod currencies;
mod rates;
mod time;

use crate::core::enums::Side;
use crate::time::daycounters::actual360::Actual360;
use crate::time::daycounters::thirty360::Thirty360;
use cashflows::cashflow;
use cashflows::coupon::Coupon;
use currencies::enums::Currency;
use currencies::structs::*;
use rates::enums::Compounding;
use rates::interestrate::InsterestRate;
use std::sync::Arc;
use time::date::Date;
use time::daycounters::enums::DayCounter;
use time::enums::Frequency;
use time::schedule::Schedule;

struct FixedRateBond {
    notional: f64,
    start_date: Date,
    end_date: Date,
    rate: InsterestRate,
    cashflows: Vec<Coupon>,
}

impl FixedRateBond {
    pub fn new(
        notional: f64,
        start_date: Date,
        end_date: Date,
        payment_frequency: Frequency,
        rate: InsterestRate,
        discount_curve_id: u16,
        currency: Currency,
        side: Side,
    ) -> FixedRateBond {
        let mut cashflows: Vec<Coupon> = Vec::new();
        let schedule =
            Schedule::generate_schedule_with_frequency(start_date, end_date, payment_frequency);
        let dates = schedule.dates();
        for i in 0..dates.len() - 1 {
            let start_date = dates[i];
            let end_date = dates[i + 1];
            let coupon = Coupon::new(
                notional,
                rate,
                start_date,
                end_date,
                end_date,
                discount_curve_id,
                currency.clone(),
                side,
            );
            cashflows.push(coupon);
        }
        FixedRateBond {
            notional,
            start_date,
            end_date,
            rate,
            cashflows,
        }
    }

    pub fn cashflows(&self) -> &Vec<Coupon> {
        return &self.cashflows;
    }
}

fn main() {
    let start_date = Date::from_ymd(2021, 1, 1);
    let end_date = Date::from_ymd(2022, 1, 1);
    let rate = InsterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Thirty360,
    );
    let bond = FixedRateBond::new(
        100.0,
        start_date,
        end_date,
        Frequency::Monthly,
        rate,
        1,
        Currency::USD,
        Side::Receive,
    );

    for cashflow in bond.cashflows() {
        println!(
            "Payment date: {}, amount: {}",
            cashflow.payment_date(),
            cashflow.amount()
        );
    }
}
