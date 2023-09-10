use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use rustatlas::{
    alm::enums::Instrument,
    cashflows::{cashflow::Cashflow, traits::Payable},
    time::date::Date,
    visitors::traits::HasCashflows,
};

mod common;
use crate::common::common::*;

fn maturing_redemptions(instruments: &[Instrument]) -> BTreeMap<Date, f64> {
    let map = Arc::new(Mutex::new(BTreeMap::new()));

    instruments.par_iter().for_each(|instrument| {
        let mut map = map.lock().unwrap();
        instrument.cashflows().iter().for_each(|cf| match cf {
            Cashflow::Redemption(f) => {
                let payment_date = f.payment_date();
                let amount = f.amount().unwrap();
                let entry = map.entry(payment_date).or_insert(0.0);
                *entry += amount;
            }
            _ => {}
        });
    });

    Arc::try_unwrap(map).unwrap().into_inner().unwrap()
}

fn outstanding(maturing_redemptions: &BTreeMap<Date, f64>) -> BTreeMap<Date, f64> {
    let mut outstanding = BTreeMap::new();
    let total = maturing_redemptions.values().sum::<f64>();
    let mut sum = 0.0;
    for (date, amount) in maturing_redemptions.iter() {
        sum += amount;
        outstanding.insert(*date, total - sum);
    }
    outstanding
}

fn main() {
    let eval_date = Date::new(2021, 9, 1);
    let n = 200_000;
    let instruments = MockMaker::generate_random_instruments(n, eval_date);

    let maturing_amount = maturing_redemptions(&instruments);
    maturing_amount.iter().for_each(|(date, amount)| {
        println!("{}: {}", date, amount);
    });

    let outstanding_amount = outstanding(&maturing_amount);
    outstanding_amount.iter().for_each(|(date, amount)| {
        println!("{}: {}", date, amount);
    });
}
