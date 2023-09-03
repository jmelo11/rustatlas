use crate::cashflows::cashflow::Cashflow;
use crate::rates::interestrate::InterestRate;
use crate::time::date::Date;

use crate::visitors::traits::HasCashflows;

/// # FixedRateInstrument
/// A fixed rate instrument.
///
/// ## Parameters
/// * `start_date` - The start date.
/// * `end_date` - The end date.
/// * `notional` - The notional.
/// * `rate` - The rate.
/// * `cashflows` - The cashflows.
pub struct FixedRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    rate: InterestRate,
    cashflows: Vec<Cashflow>,
}

impl FixedRateInstrument {
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        rate: InterestRate,
        cashflows: Vec<Cashflow>,
    ) -> Self {
        FixedRateInstrument {
            start_date: start_date,
            end_date: end_date,
            notional: notional,
            rate: rate,
            cashflows: cashflows,
        }
    }

    pub fn start_date(&self) -> Date {
        self.start_date
    }

    pub fn end_date(&self) -> Date {
        self.end_date
    }

    pub fn notional(&self) -> f64 {
        self.notional
    }

    pub fn rate(&self) -> InterestRate {
        self.rate
    }
}

impl HasCashflows for FixedRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}

#[cfg(test)]
mod dev {

    use crate::{
        cashflows::cashflow::Side,
        core::{marketstore::MarketStore, meta::MarketData},
        currencies::enums::Currency,
        instruments::makefixedrateloan::MakeFixedRateLoan,
        models::{simplemodel::SimpleModel, traits::Model},
        rates::{
            enums::Compounding,
            interestrate::InterestRate,
            interestrateindex::{enums::InterestRateIndex, overnightindex::OvernightIndex},
            yieldtermstructure::{
                enums::YieldTermStructure, flatforwardtermstructure::FlatForwardTermStructure,
            },
        },
        time::{date::Date, daycounter::DayCounter, enums::Frequency},
        visitors::{
            indexingvisitor::IndexingVisitor,
            npvconstvisitor::NPVConstVisitor,
            traits::{ConstVisit, HasCashflows, Visit},
        },
    };

    fn create_store() -> MarketStore {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let curve = YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(
            ref_date, rate,
        ));
        let index = OvernightIndex::new().with_term_structure(curve);
        market_store.mut_index_store().add_index(
            "Testing".to_string(),
            InterestRateIndex::OvernightIndex(index),
        );
        return market_store;
    }

    #[test]
    fn dev() {
        // visitors

        // instrument
        let start_date = Date::new(2023, 9, 1);
        let end_date = Date::new(2026, 9, 1);
        let notional = 100_000.0;
        let rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );
        let side = Side::Receive;

        let mut instrument = MakeFixedRateLoan::new(start_date, end_date, rate)
            .with_frequency(Frequency::Semiannual)
            .with_side(side)
            .bullet()
            .with_notional(notional)
            .build();

        instrument.set_discount_curve_id(0);

        for cf in instrument.cashflows() {
            println!("{}", cf);
        }

        let indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument);

        let market_store = create_store();

        let model = SimpleModel::new(market_store);

        let data: Vec<MarketData> = indexer
            .request()
            .iter()
            .map(|req| model.gen_node(start_date, req))
            .collect();

        data.iter().for_each(|d| println!("{:?}", d));

        let npv_visitor = NPVConstVisitor::new(data);

        let npv = npv_visitor.visit(&instrument);

        println!("NPV: {}", npv);
    }
}
