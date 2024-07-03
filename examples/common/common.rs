extern crate rustatlas;
use rustatlas::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};


#[allow(dead_code)]
pub fn print_separator() {
    println!("------------------------");
}

#[allow(dead_code)]
pub fn print_title(title: &str) {
    print_separator();
    println!("{}", title);
    print_separator();
}

#[allow(dead_code)]
pub fn print_table(cashflows: &[Cashflow], market_data: &[MarketData]) {
    println!(
        "{:10} | {:10} | {:10} | {:10}| {:10}",
        "Date", "Amount", "DF", "FWD", "FX"
    );
    for (cf, md) in cashflows.iter().zip(market_data) {
        let date = format!("{:10}", cf.payment_date().to_string());
        let amount = format!("{:10.2}", cf.amount().unwrap()); // Assuming `cf.amount()` is a float

        let df = match md.df() {
            Ok(df) => format!("{:10.2}", df),
            _ => "None      ".to_string(), // 10 characters wide
        };

        let fx = match md.fx() {
            Ok(fx) => format!("{:10.2}", fx),
            _ => "None      ".to_string(), // 10 characters wide
        };

        let fwd = match md.fwd() {
            Ok(fwd) => format!("{:9.3}", fwd),
            _ => "None      ".to_string(), // 10 characters wide
        };

        println!("{} | {} | {} | {} | {}", date, amount, df, fwd, fx);
    }
}

#[allow(dead_code)]
fn make_fixings(start: Date, end: Date, rate: f64) -> HashMap<Date, f64> {
    let mut fixings = HashMap::new();
    let mut seed = start;
    let mut init = 100.0;
    while seed <= end {
        fixings.insert(seed, init);
        seed = seed + Period::new(1, TimeUnit::Days);
        init = init * (1.0 + rate * 1.0 / 360.0);
    }
    return fixings;
}

#[allow(dead_code)]
pub fn create_store() -> Result<MarketStore> {
    let ref_date = Date::new(2021, 9, 1);
    let local_currency = Currency::USD;
    let mut market_store = MarketStore::new(ref_date, local_currency);

    let forecast_curve_1 = Arc::new(FlatForwardTermStructure::new(
        ref_date,
        Numeric::new(0.02),
        RateDefinition::default(),
    ));

    let forecast_curve_2 = Arc::new(FlatForwardTermStructure::new(
        ref_date,
        Numeric::new(0.03),
        RateDefinition::default(),
    ));

    let discount_curve = Arc::new(FlatForwardTermStructure::new(
        ref_date,
        Numeric::new(0.05),
        RateDefinition::default(),
    ));

    let mut ibor_fixings = HashMap::new();
    ibor_fixings.insert(Date::new(2021, 9, 1), 0.02); // today
    ibor_fixings.insert(Date::new(2021, 8, 31), 0.02); // yesterday

    let ibor_index = IborIndex::new(forecast_curve_1.reference_date())
        .with_fixings(ibor_fixings)
        .with_term_structure(forecast_curve_1)
        .with_frequency(Frequency::Annual);

    let overnight_fixings =
        make_fixings(ref_date - Period::new(1, TimeUnit::Years), ref_date, 0.06);
    let overnigth_index = OvernightIndex::new(forecast_curve_2.reference_date())
        .with_term_structure(forecast_curve_2)
        .with_fixings(overnight_fixings);

    market_store
        .mut_index_store()
        .add_index(0, Arc::new(RwLock::new(ibor_index)))?;

    market_store
        .mut_index_store()
        .add_index(1, Arc::new(RwLock::new(overnigth_index)))?;

    let discount_index =
        IborIndex::new(discount_curve.reference_date()).with_term_structure(discount_curve);

    market_store
        .mut_index_store()
        .add_index(2, Arc::new(RwLock::new(discount_index)))?;
    return Ok(market_store);
}
