use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        simplecashflow::SimpleCashflow,
    },
    currencies::enums::Currency,
    time::date::Date,
};

pub enum Structure {
    Bullet,
    EqualRedemptions,
    Zero,
    EqualPayments,
    Other,
}

pub enum CashflowType {
    Redemption,
    Disbursement,
}

pub fn build_cashflows(
    cashflows: &mut Vec<Cashflow>,
    dates: &[Date],
    amounts: &[f64],
    side: Side,
    currency: Currency,
    cashflow_type: CashflowType,
) {
    for (date, amount) in dates.iter().zip(amounts) {
        let cashflow = SimpleCashflow::new_with_amount(*amount, *date, currency, side);
        match cashflow_type {
            CashflowType::Redemption => cashflows.push(Cashflow::Redemption(cashflow)),
            CashflowType::Disbursement => cashflows.push(Cashflow::Disbursement(cashflow)),
            _ => (),
        }
    }
}

pub fn notionals_vector(n: usize, notional: f64, structure: Structure) -> Vec<f64> {
    match structure {
        Structure::Bullet => vec![notional; n],
        Structure::EqualRedemptions => {
            let redemptions = vec![notional / n as f64; n];
            let mut results = Vec::new();
            let mut sum = 0.0;
            for r in redemptions {
                results.push(notional - sum);
                sum += r;
            }
            results
        }
        Structure::Zero => vec![notional; 1],
        _ => vec![],
    }
}
