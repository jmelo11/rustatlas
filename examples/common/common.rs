
extern crate rustatlas;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rustatlas::{
    alm::enums::Instrument,
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::Payable,
    },
    core::{marketstore::MarketStore, meta::MarketData},
    currencies::enums::Currency,
    instruments::makefixedrateloan::MakeFixedRateLoan,
    rates::{
        enums::Compounding,
        interestrate::{InterestRate, RateDefinition},
        interestrateindex::{
            enums::InterestRateIndex, iborindex::IborIndex, overnightindex::OvernightIndex,
        },
        yieldtermstructure::{
            enums::YieldTermStructure, flatforwardtermstructure::FlatForwardTermStructure,
        }, traits::HasReferenceDate,
    },
    time::{
        date::Date,
        daycounter::DayCounter,
        enums::{Frequency, TimeUnit},
        period::Period,
    },
};
use std::{collections::HashMap, ops::Deref, rc::Rc};

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
pub fn print_table(cashflows: &[Cashflow], market_data: Rc<Vec<MarketData>>) {
    println!(
        "{:10} | {:10} | {:10} | {:10}| {:10}",
        "Date", "Amount", "DF", "FWD", "FX"
    );
    for (cf, md) in cashflows.iter().zip(market_data.deref()) {
        let date = format!("{:10}", cf.payment_date().to_string());
        let amount = format!("{:10.2}", cf.amount().unwrap()); // Assuming `cf.amount()` is a float

        let df = match md.df() {
            Some(df) => format!("{:10.2}", df),
            None => "None      ".to_string(), // 10 characters wide
        };

        let fx = match md.fx() {
            Some(fx) => format!("{:10.2}", fx),
            None => "None      ".to_string(), // 10 characters wide
        };

        let fwd = match md.fwd() {
            Some(fwd) => format!("{:9.3}", fwd),
            None => "None      ".to_string(), // 10 characters wide
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
pub fn create_store() -> MarketStore {
    let ref_date = Date::new(2021, 9, 1);
    let local_currency = Currency::USD;
    let mut market_store = MarketStore::new(ref_date, local_currency);

    let forecast_rate_1 = InterestRate::new(
        0.02,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let forecast_rate_2 = InterestRate::new(
        0.03,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let discount_rate = InterestRate::new(
        0.05,
        Compounding::Simple,
        Frequency::Annual,
        DayCounter::Actual360,
    );

    let forecast_curve_1 = YieldTermStructure::FlatForward(
        FlatForwardTermStructure::new(ref_date, forecast_rate_1),
    );

    let forecast_curve_2 = YieldTermStructure::FlatForward(
        FlatForwardTermStructure::new(ref_date, forecast_rate_2),
    );

    let discount_curve = YieldTermStructure::FlatForward(
        FlatForwardTermStructure::new(ref_date, discount_rate),
    );

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

    market_store.mut_index_store().add_index(
        "ForecastCurve 1".to_string(),
        InterestRateIndex::IborIndex(ibor_index),
    );

    market_store.mut_index_store().add_index(
        "ForecastCurve 2".to_string(),
        InterestRateIndex::OvernightIndex(overnigth_index),
    );

    let discount_index = IborIndex::new(discount_curve.reference_date()).with_term_structure(discount_curve);

    market_store.mut_index_store().add_index(
        "DiscountCurve".to_string(),
        InterestRateIndex::IborIndex(discount_index),
    );
    return market_store;
}

use rand::Rng;

pub struct MockMaker;

pub trait Mock {
    fn random_frequency() -> Frequency;

    fn random_tenor() -> Period;

    fn random_start_date(today: Date) -> Date;

    fn random_notional() -> f64;

    fn random_rate_value() -> f64;

    fn random_currency() -> Currency;

    fn generate_random_instruments(n: usize, today: Date) -> Vec<Instrument>;
}

impl Mock for MockMaker {
    fn random_frequency() -> Frequency {
        let mut rng = rand::thread_rng();
        let freq = rng.gen_range(0..4);
        match freq {
            0 => Frequency::Annual,
            1 => Frequency::Semiannual,
            2 => Frequency::Quarterly,
            3 => Frequency::Monthly,
            _ => Frequency::Annual,
        }
    }

    fn random_tenor() -> Period {
        let mut rng = rand::thread_rng();
        let freq = rng.gen_range(0..4);
        match freq {
            0 => Period::new(1, TimeUnit::Years),
            1 => Period::new(3, TimeUnit::Years),
            2 => Period::new(5, TimeUnit::Years),
            3 => Period::new(7, TimeUnit::Years),
            _ => Period::new(10, TimeUnit::Years),
        }
    }

    fn random_start_date(today: Date) -> Date {
        let mut rng = rand::thread_rng();
        let day_shift = rng.gen_range(-365..365);
        today + day_shift
    }

    fn random_notional() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(50.0..150.0)
    }

    fn random_rate_value() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0.01..0.05)
    }

    fn random_currency() -> Currency {
        let mut rng = rand::thread_rng();
        let freq = rng.gen_range(0..4);
        match freq {
            0 => Currency::USD,
            1 => Currency::EUR,
            2 => Currency::CLP,
            3 => Currency::CLF,
            _ => Currency::USD,
        }
    }

    fn generate_random_instruments(n: usize, today: Date) -> Vec<Instrument> {
        let instruments = (0..n)
            .into_par_iter() // Create a parallel iterator
            .map(|_| {
                let start_date = MockMaker::random_start_date(today);
                let end_date = start_date + MockMaker::random_tenor();
                let rate = MockMaker::random_rate_value();
                let notional = MockMaker::random_notional();
                let random_currency = MockMaker::random_currency();
                let payment_frequency = MockMaker::random_frequency();
                let instrument = MakeFixedRateLoan::new()
                    .with_start_date(start_date)
                    .with_end_date(end_date)
                    .with_payment_frequency(payment_frequency)
                    .with_rate_value(rate)
                    .with_rate_definition(RateDefinition::default())
                    .with_currency(random_currency)
                    .with_notional(notional)
                    .with_side(Side::Receive)
                    .bullet()
                    .build()
                    .unwrap();

                Instrument::FixedRateInstrument(instrument)
            })
            .collect();
        instruments
    }
}
