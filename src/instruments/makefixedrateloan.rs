use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        fixedratecoupon::FixedRateCoupon,
    },
    currencies::enums::Currency,
    prelude::HasCashflows,
    rates::interestrate::InterestRate,
    time::{date::Date, enums::Frequency, period::Period, schedule::MakeSchedule},
};

use super::{
    fixedrateinstrument::FixedRateInstrument,
    traits::{build_cashflows, notionals_vector, CashflowType, Structure},
};

pub struct MakeFixedRateLoan {
    start_date: Date,
    end_date: Date,
    period: Period,
    rate: InterestRate,
    currency: Currency,
    side: Side,
    notional: f64,
    structure: Structure,
    discount_curve_id: Option<usize>,
}

impl MakeFixedRateLoan {
    pub fn new(start_date: Date, end_date: Date, rate: InterestRate) -> MakeFixedRateLoan {
        MakeFixedRateLoan {
            start_date: start_date,
            end_date: end_date,
            period: Period::empty(),
            rate: rate,
            notional: 1.0,
            side: Side::Receive,
            currency: Currency::USD,
            structure: Structure::Other,
            discount_curve_id: None,
        }
    }

    pub fn with_discount_curve_id(mut self, id: usize) -> MakeFixedRateLoan {
        self.discount_curve_id = Some(id);
        self
    }

    pub fn with_period(mut self, period: Period) -> MakeFixedRateLoan {
        self.period = period;
        self
    }

    pub fn with_side(mut self, side: Side) -> MakeFixedRateLoan {
        self.side = side;
        self
    }

    pub fn with_notional(mut self, notional: f64) -> MakeFixedRateLoan {
        self.notional = notional;
        self
    }

    pub fn with_currency(mut self, currency: Currency) -> MakeFixedRateLoan {
        self.currency = currency;
        self
    }

    pub fn bullet(mut self) -> MakeFixedRateLoan {
        self.structure = Structure::Bullet;
        self
    }

    pub fn with_frequency(mut self, frequency: Frequency) -> MakeFixedRateLoan {
        let period = Period::from_frequency(frequency);
        match period {
            Ok(p) => {
                self.period = p;
                self
            }
            Err(_) => panic!("Invalid frequency"),
        }
    }

    pub fn build(self) -> FixedRateInstrument {
        match self.structure {
            Structure::Bullet => {
                let mut cashflows = Vec::new();
                let schedule =
                    MakeSchedule::new(self.start_date, self.end_date, self.period).build();
                let notionals =
                    notionals_vector(schedule.dates().len() - 1, self.notional, Structure::Bullet);

                let first_date = vec![*schedule.dates().first().unwrap()];
                let last_date = vec![*schedule.dates().last().unwrap()];
                let notional = vec![self.notional];
                let inv_side = match self.side {
                    Side::Pay => Side::Receive,
                    Side::Receive => Side::Pay,
                };
                build_cashflows(
                    &mut cashflows,
                    &first_date,
                    &notional,
                    inv_side,
                    self.currency,
                    CashflowType::Disbursement,
                );
                build_coupons_from_notionals(
                    &mut cashflows,
                    schedule.dates(),
                    &notionals,
                    self.rate,
                    self.side,
                    self.currency,
                );
                build_cashflows(
                    &mut cashflows,
                    &last_date,
                    &notional,
                    self.side,
                    self.currency,
                    CashflowType::Redemption,
                );
                let mut instrument = FixedRateInstrument::new(
                    self.start_date,
                    self.end_date,
                    self.notional,
                    self.rate,
                    cashflows,
                );
                match self.discount_curve_id {
                    Some(id) => instrument.set_discount_curve_id(id),
                    None => (),
                };
                instrument
            }
            _ => panic!("Not implemented"),
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
