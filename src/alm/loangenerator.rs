use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    cashflows::cashflow::Side,
    core::marketstore::MarketStore,
    currencies::enums::Currency,
    instruments::{
        makefixedrateloan::MakeFixedRateLoan, makefloatingrateloan::MakeFloatingRateLoan,
        traits::Structure,
    },
    models::{simplemodel::SimpleModel, traits::Model},
    rates::{interestrate::RateDefinition, traits::HasReferenceDate},
    time::{enums::Frequency, period::Period},
    utils::errors::Result,
    visitors::{
        indexingvisitor::IndexingVisitor,
        parvaluevisitor::ParValueConstVisitor,
        traits::{ConstVisit, Visit},
    },
};

use super::enums::{Instrument, RateType};

/// # LoanConfiguration
/// Configuration for a loan. Represents the meta data required to generate a loan.
#[derive(Serialize, Deserialize, Clone)]
pub struct LoanConfiguration {
    weight: f64,
    structure: Structure,
    payment_frequency: Frequency,
    tenor: Period,
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

/// # LoanGenerator
/// Generates a loan based on a configuration and a market store.
pub struct LoanGenerator {
    amount: f64,
    currency: Currency,
    configs: Rc<Vec<LoanConfiguration>>,
    market_store: Rc<MarketStore>,
}

impl LoanGenerator {
    pub fn new(
        amount: f64,
        currency: Currency,
        configs: Rc<Vec<LoanConfiguration>>,
        market_store: Rc<MarketStore>,
    ) -> LoanGenerator {
        LoanGenerator {
            amount,
            currency,
            configs,
            market_store,
        }
    }

    fn calculate_par_spread(&self, builder: MakeFloatingRateLoan) -> Result<f64> {
        let mut instrument = builder.with_spread(0.01).build()?;
        let indexing_visitor = IndexingVisitor::new();
        let _ = indexing_visitor.visit(&mut instrument);
        let model = SimpleModel::new(self.market_store.clone());
        let data = model.gen_market_data(&indexing_visitor.request())?;
        let par_visitor = ParValueConstVisitor::new(Rc::new(data));
        Ok(par_visitor.visit(&mut instrument)?)
    }

    fn calculate_par_rate(&self, builder: MakeFixedRateLoan) -> Result<f64> {
        let mut instrument = builder.with_rate_value(0.03).build()?;
        let indexing_visitor = IndexingVisitor::new();
        let _ = indexing_visitor.visit(&mut instrument);
        let model = SimpleModel::new(self.market_store.clone());
        let data = model.gen_market_data(&indexing_visitor.request())?;
        let par_visitor = ParValueConstVisitor::new(Rc::new(data));
        Ok(par_visitor.visit(&mut instrument)?)
    }

    pub fn generate_position(&self, config: &LoanConfiguration) -> Result<Instrument> {
        let structure = config.structure();
        let notional = self.amount * config.weight();
        let start_date = self.market_store.reference_date();
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
                    .with_currency(self.currency)
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
                    .with_currency(self.currency)
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
            interestrate::RateDefinition, interestrateindex::iborindex::IborIndex,
            yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
        },
        time::{date::Date, enums::TimeUnit},
    };

    use super::*;

    fn create_store() -> Result<MarketStore> {
        let ref_date = Date::new(2021, 9, 1);
        let local_currency = Currency::USD;
        let mut market_store = MarketStore::new(ref_date, local_currency);

        let discount_curve = Box::new(FlatForwardTermStructure::new(
            ref_date,
            0.5,
            RateDefinition::default(),
        ));

        let discount_index = Box::new(IborIndex::new(ref_date).with_term_structure(discount_curve));
        market_store
            .mut_index_store()
            .add_index("DiscountCurve".to_string(), discount_index)?;
        return Ok(market_store);
    }

    #[test]
    fn generator_tests() -> Result<()> {
        let market_store = Rc::new(create_store()?);
        let configs = Rc::new(vec![LoanConfiguration::new(
            1.0,
            Structure::Bullet,
            Frequency::Annual,
            Period::new(1, TimeUnit::Years),
            Side::Receive,
            RateType::Fixed,
            RateDefinition::default(),
            0,
            None,
        )]);
        let generator = LoanGenerator::new(100.0, Currency::USD, configs, market_store);
        let positions = generator.generate();
        assert_eq!(positions.len(), 1);
        Ok(())
    }
}
