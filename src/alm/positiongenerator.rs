use serde::{Deserialize, Serialize};

use crate::{
    cashflows::cashflow::Side,
    core::marketstore::MarketStore,
    currencies::enums::Currency,
    instruments::{
        instrument::{Instrument, RateType},
        makefixedrateinstrument::MakeFixedRateInstrument,
        makefloatingrateinstrument::MakeFloatingRateInstrument,
        traits::Structure,
    },
    models::{simplemodel::SimpleModel, traits::Model},
    rates::{interestrate::RateDefinition, traits::HasReferenceDate},
    time::{enums::Frequency, period::Period},
    utils::errors::{AtlasError, Result},
    visitors::{
        indexingvisitor::IndexingVisitor,
        parvaluevisitor::ParValueConstVisitor,
        traits::{ConstVisit, Visit},
    },
};

/// # `RolloverStrategy`
/// Configuration for a loan rollover strategy. It holds the data required to generate a loan.
///
/// ## Fields
/// * `weight` - Weight of the loan in the portfolio
/// * `structure` - Structure of the loan
/// * `payment_frequency` - Payment frequency of the loan
/// * `tenor` - Tenor of the loan
/// * `side` - Side of the loan
/// * `rate_type` - Type of the rate
/// * `rate_definition` - Rate definition
/// * `discount_curve_id` - Id of the discount curve
/// * `forecast_curve_id` - Id of the forecast curve, if any
#[derive(Serialize, Deserialize, Clone)]
pub struct RolloverStrategy {
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

impl RolloverStrategy {
    /// Creates a new `RolloverStrategy` with the specified parameters.
    #[must_use]
    // allowed: high-arity API; refactor deferred
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        weight: f64,
        structure: Structure,
        payment_frequency: Frequency,
        tenor: Period,
        side: Side,
        rate_type: RateType,
        rate_definition: RateDefinition,
        discount_curve_id: usize,
        forecast_curve_id: Option<usize>,
    ) -> Self {
        Self {
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

    /// Returns the weight of the strategy.
    #[must_use]
    pub const fn weight(&self) -> f64 {
        self.weight
    }

    /// Returns the structure of the strategy.
    #[must_use]
    pub const fn structure(&self) -> Structure {
        self.structure
    }

    /// Returns the payment frequency of the strategy.
    #[must_use]
    pub const fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    /// Returns the tenor of the strategy.
    #[must_use]
    pub const fn tenor(&self) -> Period {
        self.tenor
    }

    /// Returns the side of the strategy.
    #[must_use]
    pub const fn side(&self) -> Side {
        self.side
    }

    /// Returns the rate type of the strategy.
    #[must_use]
    pub const fn rate_type(&self) -> RateType {
        self.rate_type
    }

    /// Returns the rate definition of the strategy.
    #[must_use]
    pub const fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    /// Returns the discount curve ID of the strategy.
    #[must_use]
    pub const fn discount_curve_id(&self) -> usize {
        self.discount_curve_id
    }

    /// Returns the forecast curve ID of the strategy.
    #[must_use]
    pub const fn forecast_curve_id(&self) -> usize {
        match self.forecast_curve_id {
            Some(id) => id,
            None => panic!("No forecast curve id"),
        }
    }
}

/// # `PositionGenerator`
/// Generates a loan based on a configuration and a market store.
///
/// ## Fields
/// * `new_positions_currency` - Currency of the new positions
/// * `strategies` - Strategies to generate the new positions
#[derive(Clone)]
pub struct PositionGenerator<'a> {
    new_positions_currency: Currency,
    strategies: Vec<RolloverStrategy>,
    market_store: Option<&'a MarketStore>,
    amount: Option<f64>,
}

impl<'a> PositionGenerator<'a> {
    /// Creates a new `PositionGenerator` with the specified currency and strategies.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn new(new_positions_currency: Currency, strategies: Vec<RolloverStrategy>) -> Self {
        Self {
            new_positions_currency,
            strategies,
            market_store: None,
            amount: None,
        }
    }

    /// Sets the amount for position generation.
    #[must_use]
    pub const fn with_amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the market store for position generation.
    #[must_use]
    pub const fn with_market_store(mut self, market_store: &'a MarketStore) -> Self {
        self.market_store = Some(market_store);
        self
    }

    fn calculate_par_spread(&self, builder: MakeFloatingRateInstrument) -> Result<f64> {
        let mut instrument = builder.with_spread(0.01).build()?;
        let indexing_visitor = IndexingVisitor::new();
        let _ = indexing_visitor.visit(&mut instrument);
        let market_store = self.market_store.ok_or(AtlasError::ValueNotSetErr(
            "Market store not set for loan generator".into(),
        ))?;
        let model = SimpleModel::new(market_store);
        let data = model.gen_market_data(&indexing_visitor.request())?;
        let par_visitor = ParValueConstVisitor::new(&data);
        par_visitor.visit(&instrument)
    }

    fn calculate_par_rate(&self, builder: MakeFixedRateInstrument) -> Result<f64> {
        let mut instrument = builder.with_rate_value(0.03).build()?;
        let indexing_visitor = IndexingVisitor::new();
        let _ = indexing_visitor.visit(&mut instrument);
        let market_store = self.market_store.ok_or(AtlasError::ValueNotSetErr(
            "Market store not set for loan generator".into(),
        ))?;
        let model = SimpleModel::new(market_store);

        let data = model.gen_market_data(&indexing_visitor.request())?;

        let par_visitor = ParValueConstVisitor::new(&data);
        par_visitor.visit(&instrument)
    }

    /// Generates a single position based on the provided strategy.
    ///
    /// # Errors
    ///
    /// Returns an error if required configuration is missing or if pricing/indexing visitors fail.
    pub fn generate_position(&self, strategies: &RolloverStrategy) -> Result<Instrument> {
        let structure = strategies.structure();
        let amount = self
            .amount
            .ok_or(AtlasError::ValueNotSetErr("Amount".into()))?;
        let notional = amount * strategies.weight();

        let market_store = self.market_store.ok_or(AtlasError::ValueNotSetErr(
            "Market store not set for loan generator".into(),
        ))?;
        let start_date = market_store.reference_date();
        match strategies.rate_type() {
            RateType::Floating => {
                let builder = MakeFloatingRateInstrument::new()
                    .with_issue_date(start_date)
                    .with_start_date(start_date)
                    .with_tenor(strategies.tenor())
                    .with_payment_frequency(strategies.payment_frequency())
                    .with_structure(structure)
                    .with_side(strategies.side())
                    .with_forecast_curve_id(Some(strategies.forecast_curve_id()))
                    .with_discount_curve_id(Some(strategies.discount_curve_id()))
                    .with_currency(self.new_positions_currency)
                    .with_rate_definition(strategies.rate_definition())
                    .with_notional(notional);
                let spread = self.calculate_par_spread(builder.clone())?;
                Ok(Instrument::FloatingRateInstrument(
                    builder.with_spread(spread).build()?,
                ))
            }
            RateType::Fixed => {
                let builder = MakeFixedRateInstrument::new()
                    .with_issue_date(start_date)
                    .with_notional(notional)
                    .with_start_date(start_date)
                    .with_tenor(strategies.tenor())
                    .with_payment_frequency(strategies.payment_frequency())
                    .with_rate_definition(strategies.rate_definition())
                    .with_side(strategies.side())
                    .with_currency(self.new_positions_currency)
                    .with_discount_curve_id(Some(strategies.discount_curve_id()))
                    .with_structure(structure);

                let rate = self.calculate_par_rate(builder.clone())?;
                Ok(Instrument::FixedRateInstrument(
                    builder.with_rate_value(rate).build()?,
                ))
            }
            RateType::FixedThenFloating => {
                unimplemented!("Not implemented")
            }
            RateType::FloatingThenFixed => {
                unimplemented!("Not implemented")
            }
            RateType::FixedThenFixed => {
                unimplemented!("Not implemented")
            }
            RateType::Suffled => {
                unimplemented!("Not implemented")
            }
        }
    }

    /// Generates all positions based on the configured strategies.
    #[must_use]
    pub fn generate(&self) -> Vec<Instrument> {
        self.strategies
            .iter()
            .map(|c| {
                self.generate_position(c).unwrap_or_else(|e| {
                    panic!("generate_position should succeed in PositionGenerator::generate: {e}")
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

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

        let discount_curve = Arc::new(FlatForwardTermStructure::new(
            ref_date,
            0.5,
            RateDefinition::default(),
        ));

        let discount_index = Arc::new(RwLock::new(
            IborIndex::new(ref_date).with_term_structure(discount_curve),
        ));
        market_store
            .mut_index_store()
            .add_index(0, discount_index)?;
        Ok(market_store)
    }

    #[test]
    fn generator_tests_fixed() -> Result<()> {
        let market_store = create_store()?;
        let configs = vec![RolloverStrategy::new(
            1.0,
            Structure::Bullet,
            Frequency::Annual,
            Period::new(1, TimeUnit::Years),
            Side::Receive,
            RateType::Fixed,
            RateDefinition::default(),
            0,
            None,
        )];
        let generator = PositionGenerator::new(Currency::USD, configs)
            .with_amount(100.0)
            .with_market_store(&market_store);
        let positions = generator.generate();
        assert_eq!(positions.len(), 1);
        Ok(())
    }

    #[test]
    fn generator_tests_floating() -> Result<()> {
        let market_store = create_store()?;

        let configs = vec![RolloverStrategy::new(
            0.5,
            Structure::EqualRedemptions,
            Frequency::Annual,
            Period::new(1, TimeUnit::Years),
            Side::Receive,
            RateType::Floating,
            RateDefinition::default(),
            0,
            Some(0),
        )];

        let date = Date::new(2021, 9, 1) + Period::new(7, TimeUnit::Days);
        let tmp_store = market_store.advance_to_date(date)?;

        let generator = PositionGenerator::new(Currency::USD, configs)
            .with_amount(100.0)
            .with_market_store(&tmp_store);
        let positions = generator.generate();
        assert_eq!(positions.len(), 1);
        Ok(())
    }
}
