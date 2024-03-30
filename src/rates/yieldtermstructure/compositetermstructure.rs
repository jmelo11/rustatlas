use std::sync::Arc;

use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider},
    },
    time::{date::Date, enums::Frequency, period::Period},
    utils::errors::Result,
};

use super::traits::{AdvanceTermStructureInTime, YieldTermStructureTrait};

/// # CompositeTermStructure
/// Struct that defines a term structure made with a combination of two curves. It's defined as:
/// $$
///    df_{spreaded}(t) = df_{spread}(t) * df_{base}(t)
/// $$
///
/// # Example
/// ```
/// use rustatlas::prelude::*;
/// use std::sync::Arc;
/// let ref_date = Date::new(2021, 1, 1);
///
/// let spread_curve = FlatForwardTermStructure::new(
///   ref_date,
///     0.01,
///     RateDefinition::default()
/// );
///
/// let base_curve = FlatForwardTermStructure::new(
///     ref_date,
///     0.02,
///     RateDefinition::default()
/// );
///
/// let spreaded_curve = CompositeTermStructure::new(Arc::new(spread_curve), Arc::new(base_curve));
/// assert_eq!(spreaded_curve.reference_date(), ref_date);
/// ```
#[derive(Clone)]
pub struct CompositeTermStructure {
    date_reference: Date, // reference_date
    spread_curve: Arc<dyn YieldTermStructureTrait>,
    base_curve: Arc<dyn YieldTermStructureTrait>,
}

impl CompositeTermStructure {
    pub fn new(
        spread_curve: Arc<dyn YieldTermStructureTrait>,
        base_curve: Arc<dyn YieldTermStructureTrait>,
    ) -> CompositeTermStructure {
        CompositeTermStructure {
            date_reference: base_curve.reference_date(),
            spread_curve,
            base_curve,
        }
    }

    pub fn spread_curve(&self) -> &dyn YieldTermStructureTrait {
        return self.spread_curve.as_ref();
    }

    pub fn base_curve(&self) -> &dyn YieldTermStructureTrait {
        return self.base_curve.as_ref();
    }
}

impl HasReferenceDate for CompositeTermStructure {
    fn reference_date(&self) -> Date {
        return self.date_reference;
    }
}

impl YieldProvider for CompositeTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64> {
        let spread_discount_factor = self.spread_curve.discount_factor(date)?;
        let base_discount_factor = self.base_curve.discount_factor(date)?;
        let add_df = spread_discount_factor * base_discount_factor;
        return Ok(add_df);
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64> {
        let spread_forward_rate = self
            .spread_curve
            .forward_rate(start_date, end_date, comp, freq)?;
        let base_forward_rate = self
            .base_curve
            .forward_rate(start_date, end_date, comp, freq)?;
        return Ok(spread_forward_rate + base_forward_rate);
    }
}

/// # AdvanceTermStructureInTime for CompositeTermStructure
impl AdvanceTermStructureInTime for CompositeTermStructure {
    fn advance_to_date(&self, date: Date) -> Result<Arc<dyn YieldTermStructureTrait>> {
        let base = self.base_curve().advance_to_date(date)?;
        let spread = self.spread_curve().advance_to_date(date)?;
        Ok(Arc::new(CompositeTermStructure::new(spread, base)))
    }

    fn advance_to_period(&self, period: Period) -> Result<Arc<dyn YieldTermStructureTrait>> {
        let base = self.base_curve().advance_to_period(period)?;
        let spread = self.spread_curve().advance_to_period(period)?;
        Ok(Arc::new(CompositeTermStructure::new(spread, base)))
    }
}

impl YieldTermStructureTrait for CompositeTermStructure {}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use crate::{
        rates::{
            enums::Compounding,
            interestrate::RateDefinition,
            traits::{HasReferenceDate, YieldProvider},
            yieldtermstructure::{
                compositetermstructure::CompositeTermStructure,
                flatforwardtermstructure::FlatForwardTermStructure,
            },
        },
        time::{date::Date, daycounter::DayCounter, enums::Frequency},
    };

    #[test]
    fn test_reference_date() {
        let spread_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.1,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));

        let base_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.2,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));
        let spreaded_curve = CompositeTermStructure::new(spread_curve, base_curve);
        assert!(spreaded_curve.reference_date() == Date::new(2020, 1, 1));
    }

    #[test]
    fn test_forward_rate() {
        let spread_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.01,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));

        let base_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.02,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));
        let spreaded_curve = CompositeTermStructure::new(spread_curve, base_curve);

        let fr = spreaded_curve.forward_rate(
            Date::new(2020, 1, 1),
            Date::new(2022, 1, 1),
            Compounding::Compounded,
            Frequency::Annual,
        );
        assert!((fr.unwrap() - 0.03) < 0.0001);
    }

    #[test]
    fn test_discount_factor() {
        let spread_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.1,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));

        let base_curve = Arc::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            0.2,
            RateDefinition::new(
                DayCounter::Actual360,
                Compounding::Compounded,
                Frequency::Annual,
            ),
        ));

        let spreaded_curve = CompositeTermStructure::new(spread_curve, base_curve);

        let df = spreaded_curve.discount_factor(Date::new(2021, 1, 1));
        println!("df: {:?}", df);

        assert!(df.unwrap() - 0.9702040771633191 < 0.00001);
    }
}
