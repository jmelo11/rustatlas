use std::collections::{HashMap, HashSet};

use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        simplecashflow::SimpleCashflow,
    },
    currencies::enums::Currency,
    time::date::Date,
};

#[derive(Clone, Copy, Debug, PartialEq)]
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
        let cashflow = SimpleCashflow::new(*date, currency, side).with_amount(*amount);
        match cashflow_type {
            CashflowType::Redemption => cashflows.push(Cashflow::Redemption(cashflow)),
            CashflowType::Disbursement => cashflows.push(Cashflow::Disbursement(cashflow)),
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

pub fn calculate_outstanding(
    disbursements: &HashMap<Date, f64>,
    redemptions: &HashMap<Date, f64>,
    additional_dates: &HashSet<Date>,
) -> Vec<(Date, Date, f64)> {
    let mut outstanding = Vec::new();

    // Combine disbursements and redemptions into a timeline of events
    let mut timeline: Vec<(Date, f64)> =
        disbursements.iter().map(|(k, v)| (k.clone(), *v)).collect();

    for (date, amount) in redemptions.iter() {
        match timeline.iter_mut().find(|(d, _)| *d == *date) {
            Some((_, a)) => *a -= amount,
            None => timeline.push((date.clone(), -amount)),
        }
    }

    // Add zero-value entries for additional dates not in the timeline
    for date in additional_dates {
        if timeline.iter().all(|(d, _)| d != date) {
            timeline.push((date.clone(), 0.0));
        }
    }

    // Sort the timeline based on the date
    timeline.sort_by_key(|k| k.0);

    // Process the timeline
    let mut event_iter = timeline.iter();
    let (mut period_start, mut current_amount) = match event_iter.next() {
        Some((date, amount)) => (date.clone(), *amount),
        None => return Vec::new(),
    };

    for (date, amount) in event_iter {
        let period_end = date.clone();
        outstanding.push((period_start.clone(), period_end.clone(), current_amount));
        current_amount += amount;
        period_start = period_end;
    }

    outstanding
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn basic_test_cases() {
        let mut redemptions = HashMap::new();
        redemptions.insert(Date::new(2023, 8, 27), 50.0);
        redemptions.insert(Date::new(2023, 9, 27), 50.0);
        redemptions.insert(Date::new(2023, 10, 27), 50.0);

        let mut disbursements = HashMap::new();
        disbursements.insert(Date::new(2023, 6, 27), 150.0);

        let result = calculate_outstanding(&disbursements, &redemptions, &HashSet::new());

        assert_eq!(result.len(), 3);
        assert_eq!(
            result[0],
            (Date::new(2023, 6, 27), Date::new(2023, 8, 27), 150.0)
        );
        assert_eq!(
            result[1],
            (Date::new(2023, 8, 27), Date::new(2023, 9, 27), 100.0)
        );
        assert_eq!(
            result[2],
            (Date::new(2023, 9, 27), Date::new(2023, 10, 27), 50.0)
        );
    }

    #[test]
    fn additional_dates() {
        let mut redemptions = HashMap::new();
        redemptions.insert(Date::new(2023, 8, 27), 50.0);
        redemptions.insert(Date::new(2023, 9, 27), 50.0);
        redemptions.insert(Date::new(2023, 11, 27), 50.0);

        let mut disbursements = HashMap::new();
        disbursements.insert(Date::new(2023, 6, 27), 150.0);

        let mut additional_dates = HashSet::new();
        additional_dates.insert(Date::new(2023, 10, 27));

        let result = calculate_outstanding(&disbursements, &redemptions, &additional_dates);

        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0],
            (Date::new(2023, 6, 27), Date::new(2023, 8, 27), 150.0)
        );
        assert_eq!(
            result[1],
            (Date::new(2023, 8, 27), Date::new(2023, 9, 27), 100.0)
        );
        assert_eq!(
            result[2],
            (Date::new(2023, 9, 27), Date::new(2023, 10, 27), 50.0)
        );
        assert_eq!(
            result[3],
            (Date::new(2023, 10, 27), Date::new(2023, 11, 27), 50.0)
        );
    }
}
