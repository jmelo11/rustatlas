use crate::{
    cashflows::cashflow::{Cashflow, Side},
    time::date::Date,
    visitors::traits::HasCashflows,
};

pub struct FloatingRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    spread: f64,
    side: Side,
    cashflows: Vec<Cashflow>,
}

impl FloatingRateInstrument {
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        spread: f64,
        side: Side,
        cashflows: Vec<Cashflow>,
    ) -> Self {
        FloatingRateInstrument {
            start_date: start_date,
            end_date: end_date,
            notional: notional,
            spread: spread,
            side: side,
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

    pub fn spread(&self) -> f64 {
        self.spread
    }

    pub fn side(&self) -> Side {
        self.side
    }
}

impl HasCashflows for FloatingRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}

#[cfg(test)]
mod dev {
    use std::rc::Rc;

    use crate::{
        cashflows::cashflow::Side,
        core::{marketstore::MarketStore, meta::MarketData},
        currencies::enums::Currency,
        instruments::makefloatingrateloan::MakeFloatingRateLoan,
        models::{simplemodel::SimpleModel, traits::Model},
        rates::{
            enums::Compounding,
            interestrate::InterestRate,
            interestrateindex::overnightindex::OvernightIndex,
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
        market_store
            .mut_yield_providers_store()
            .add_provider("Testing".to_string(), Rc::new(index));
        return market_store;
    }

    #[test]
    fn dev() {
        // market store
        let market_store = create_store();

        // instrument
        let start_date = Date::new(2023, 9, 1);
        let end_date = Date::new(2026, 9, 1);
        let notional = 100_000.0;

        let mut instrument = MakeFloatingRateLoan::new(start_date, end_date)
            .with_frequency(Frequency::Semiannual)
            .bullet()
            .with_notional(notional)
            .build();

        instrument.set_discount_curve_id(0);

        let mut indexer = IndexingVisitor::new();
        indexer.visit(&mut instrument);

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
