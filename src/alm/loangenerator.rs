use std::rc::Rc;

use argmin::core::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    cashflows::cashflow::Side,
    core::marketstore::MarketStore,
    currencies::enums::Currency,
    instruments::{
        makefixedrateloan::{MakeFixedRateLoan, MakeFixedRateLoanError},
        makefloatingrateloan::{MakeFloatingRateLoan, MakeFloatingRateLoanError},
        traits::Structure,
    },
    models::{
        simplemodel::SimpleModel,
        traits::{Model, ModelError},
    },
    rates::{interestrate::RateDefinition, traits::HasReferenceDate},
    time::{enums::Frequency, period::Period},
    visitors::{
        indexingvisitor::IndexingVisitor,
        parvaluevisitor::ParValueConstVisitor,
        traits::{ConstVisit, Visit},
    }, prelude::Date,
};

use super::enums::Instrument;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RateType {
    Fixed,
    Floating,
}

/// # LoanGenerator
/// Generates a loan based on a configuration and a market store.
pub struct LoanGenerator {
    amount: f64,
    date: Date,
    configs: Rc<Vec<LoanConfiguration>>,
    market_store: Rc<MarketStore>,
}

#[derive(Error, Debug)]
pub enum LoanGeneratorError {
    #[error("Invalid configuration")]
    InvalidConfiguration,
    #[error("Error fixed rate loan build error {0}")]
    FixedRateLoanBuildError(#[from] MakeFixedRateLoanError),
    #[error("Error floating rate loan build error {0}")]
    FloatingRateLoanBuildError(#[from] MakeFloatingRateLoanError),
    #[error("Error par value calculation")]
    ParValueError(#[from] Error),
    #[error("Model error {0}")]
    ModelError(#[from] ModelError),
}

/// # LoanConfiguration
/// Configuration for a loan. Represents the meta data required to generate a loan.
pub struct LoanConfiguration {
    weight: f64,
    structure: Structure,
    payment_frequency: Frequency,
    tenor: Period,
    currency: Currency,
    side: Side,
    rate_type: RateType,
    rate_definition: RateDefinition,
    discount_curve_id: usize,
    forecast_curve_id: Option<usize>,
}

impl LoanConfiguration {
    pub fn new(
        weight: f64,
        structure: Structure,
        payment_frequency: Frequency,
        tenor: Period,
        currency: Currency,
        side: Side,
        rate_type: RateType,
        rate_definition: RateDefinition,
        discount_curve_id: usize,
        forecast_curve_id: Option<usize>,
    ) -> LoanConfiguration {
        LoanConfiguration {
            weight,
            structure,
            payment_frequency,
            tenor,
            currency,
            side,
            rate_type,
            rate_definition,
            discount_curve_id,
            forecast_curve_id,
        }
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    pub fn tenor(&self) -> Period {
        self.tenor
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn rate_type(&self) -> RateType {
        self.rate_type
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn discount_curve_id(&self) -> usize {
        self.discount_curve_id
    }

    pub fn forecast_curve_id(&self) -> usize {
        self.forecast_curve_id.expect("No forecast curve id")
    }
}

impl LoanGenerator {
    pub fn new(
        amount: f64,
        date: Date,
        configs: Rc<Vec<LoanConfiguration>>,
        market_store: Rc<MarketStore>,
    ) -> LoanGenerator {
        LoanGenerator {
            amount,
            date,
            configs,
            market_store,
        }
    }

    fn calculate_par_spread(
        &self,
        builder: MakeFloatingRateLoan,
    ) -> Result<f64, LoanGeneratorError> {
        let mut instrument = builder.with_spread(0.01).build()?;
        let indexing_visitor = IndexingVisitor::new();
        let _ = indexing_visitor.visit(&mut instrument);
        let model = SimpleModel::new(self.market_store.clone());
        let data = model.gen_market_data(&indexing_visitor.request())?;
        let par_visitor = ParValueConstVisitor::new(Rc::new(data));
        Ok(par_visitor.visit(&mut instrument)?)
    }

    fn calculate_par_rate(&self, builder: MakeFixedRateLoan) -> Result<f64, LoanGeneratorError> {
        let mut instrument = builder.with_rate_value(0.03).build()?;
        let indexing_visitor = IndexingVisitor::new();              
        let _ = indexing_visitor.visit(&mut instrument);
        let model = SimpleModel::new(self.market_store.clone());
        let data = model.gen_market_data(&indexing_visitor.request())?;
        let par_visitor = ParValueConstVisitor::new(Rc::new(data));
        Ok(par_visitor.visit(&mut instrument)?)
    }

    pub fn generate_position(
        &self,
        config: &LoanConfiguration,
    ) -> Result<Instrument, LoanGeneratorError> {
        let structure = config.structure();
        let notional = self.amount * config.weight();
        let start_date = self.date;
        match config.rate_type() {
            RateType::Floating => {
                let builder = MakeFloatingRateLoan::new()
                    .with_start_date(start_date)
                    .with_tenor(config.tenor())
                    .with_payment_frequency(config.payment_frequency())
                    .with_structure(structure)
                    .with_side(config.side())
                    .with_forecast_curve_id(Some(config.forecast_curve_id()))
                    .with_discount_curve_id(Some(config.discount_curve_id()))
                    .with_currency(config.currency())
                    .with_rate_definition(config.rate_definition())
                    .with_notional(notional);
                let spread = self.calculate_par_spread(builder.clone())?;
                Ok(Instrument::FloatingRateInstrument(
                    builder.with_spread(spread).build()?,
                ))
            }
            RateType::Fixed => {
                let builder = MakeFixedRateLoan::new()
                    .with_notional(notional)
                    .with_start_date(start_date)
                    .with_tenor(config.tenor())
                    .with_payment_frequency(config.payment_frequency())
                    .with_rate_definition(config.rate_definition())
                    .with_side(config.side())
                    .with_currency(config.currency())
                    .with_discount_curve_id(Some(config.discount_curve_id()))
                    .with_structure(structure);

                let rate = self.calculate_par_rate(builder.clone())?;
                Ok(Instrument::FixedRateInstrument(
                    builder.with_rate_value(rate).build()?,
                ))
            }
        }
    }

    pub fn generate(&self) -> Vec<Instrument> {
        let positions = self
            .configs
            .iter()
            .map(|c| self.generate_position(c).unwrap())
            .collect();

        positions
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rates::{
            enums::Compounding,
            interestrate::{InterestRate, RateDefinition},
            interestrateindex::{enums::InterestRateIndex, iborindex::IborIndex},
            yieldtermstructure::{
                enums::YieldTermStructure, flatforwardtermstructure::FlatForwardTermStructure,
            },
        },
        time::{date::Date, daycounter::DayCounter, enums::TimeUnit},
    };

    use super::*;

    fn create_store() -> MarketStore {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let discount_rate = InterestRate::new(
            0.05,
            Compounding::Simple,
            Frequency::Annual,
            DayCounter::Actual360,
        );

        let discount_curve =
            YieldTermStructure::FlatForward(FlatForwardTermStructure::new(ref_date, discount_rate));

        let discount_index = IborIndex::new(ref_date).with_term_structure(discount_curve);
        market_store.mut_index_store().add_index(
            "DiscountCurve".to_string(),
            InterestRateIndex::IborIndex(discount_index),
        );
        return market_store;
    }

    #[test]
    fn generator_tests() {
        let market_store = Rc::new(create_store());
        let configs = Rc::new(vec![LoanConfiguration::new(
            1.0,
            Structure::Bullet,
            Frequency::Annual,
            Period::new(1, TimeUnit::Years),
            Currency::USD,
            Side::Receive,
            RateType::Fixed,
            RateDefinition::default(),
            0,
            None,
        )]);
        let date = Date::new(2023, 9, 1);
        let generator = LoanGenerator::new(100.0,date ,configs, market_store);
        let positions = generator.generate();
        assert_eq!(positions.len(), 1);
    }
}
