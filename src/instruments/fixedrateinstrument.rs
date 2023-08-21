use super::cashflowstream::CashflowStream;
use crate::{
    cashflows::{cashflow::SimpleCashflow, enums::{Cashflow, Side}, fixedratecoupon::FixedRateCoupon},
    currencies::enums::Currency,
    rates::interestrate::InterestRate,
    time::{date::Date, enums::Frequency, schedule::Schedule},
};

pub struct FixedRateInstrument {
    start_date: Date,
    end_date: Date,
    payment_frequency: Frequency,
    rate: InterestRate,
    notional: f64,
    currency: Currency,
    cashflow_stream: CashflowStream,
}

impl FixedRateInstrument {
    pub fn new(
        start_date: Date,
        end_date: Date,
        payment_frequency: Frequency,
        rate: InterestRate,
        notional: f64,
        currency: Currency,
        cashflow_stream: CashflowStream,
    ) -> FixedRateInstrument {
        FixedRateInstrument {
            start_date,
            end_date,
            payment_frequency,
            rate,
            notional,
            currency,
            cashflow_stream,
        }
    }

    pub fn cashflow_stream(&mut self) -> &mut CashflowStream {
        return &mut self.cashflow_stream;
    }

    pub fn cashflows(&mut self) -> &mut Vec<Cashflow> {
        return self.cashflow_stream.cashflows();
    }

    pub fn as_bullet(
        start_date: Date,
        end_date: Date,
        payment_frequency: Frequency,
        rate: InterestRate,
        notional: f64,
        discount_curve_id: usize,
        currency: Currency,
        side: Side,
    ) -> FixedRateInstrument {
        let schedule =
            Schedule::generate_schedule_with_frequency(start_date, end_date, payment_frequency);
        let dates = schedule.dates();
        let mut cashflows = Vec::new();

        let flip_side = match side {
            Side::Receive => Side::Pay,
            Side::Pay => Side::Receive,
        };
        let redemption =
            SimpleCashflow::new(notional, start_date, discount_curve_id, currency, flip_side);
        cashflows.push(Cashflow::Disbursement(redemption));
        for i in 0..dates.len() - 1 {
            let coupon = FixedRateCoupon::new(
                notional,
                rate,
                dates[i],
                dates[i + 1],
                dates[i + 1],
                discount_curve_id,
                currency,
                side,
            );
            cashflows.push(Cashflow::FixedRateCoupon(coupon));
        }
        let redemption = SimpleCashflow::new(notional, end_date, discount_curve_id, currency, side);
        cashflows.push(Cashflow::Redemption(redemption));

        let cashflow_stream = CashflowStream::new(cashflows);

        FixedRateInstrument::new(
            start_date,
            end_date,
            payment_frequency,
            rate,
            notional,
            currency,
            cashflow_stream,
        )
    }
}
