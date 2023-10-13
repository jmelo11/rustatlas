use crate::{
    rates::{
        enums::Compounding,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    },
    time::{date::Date, enums::Frequency},
};

use super::traits::YieldTermStructureTrait;

/// # SpreadedTermStructure
/// Struct that defines a spreaded term structure. The spreaded term structure is defined as:
/// $$
///    df_{spreaded}(t) = df_{spread}(t) * df_{base}(t)
/// $$
///
/// # Example
/// ```
/// use rustatlas::prelude::*;
/// let ref_date = Date::new(2021, 1, 1);
///
/// let spread_curve = FlatForwardTermStructure::new(
///   ref_date,
///   InterestRate::new(
///       0.01,
///     Compounding::Compounded,
///     Frequency::Annual,
///     DayCounter::Actual360,
///     ),
/// );
///
/// let base_curve = FlatForwardTermStructure::new(
///   ref_date,
///  InterestRate::new(
///     0.02,
///     Compounding::Compounded,
///     Frequency::Annual,
///     DayCounter::Actual360,
///     ),
/// );
///
/// let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);
/// assert_eq!(spreaded_curve.reference_date(), ref_date);
/// ```
#[derive(Clone)]
pub struct SpreadedTermStructure {
    date_reference: Date, // reference_date
    spread_curve: Box<dyn YieldTermStructureTrait>,
    base_curve: Box<dyn YieldTermStructureTrait>,
}

impl SpreadedTermStructure {
    pub fn new(
        spread_curve: Box<dyn YieldTermStructureTrait>,
        base_curve: Box<dyn YieldTermStructureTrait>,
    ) -> SpreadedTermStructure {
        SpreadedTermStructure {
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

impl HasReferenceDate for SpreadedTermStructure {
    fn reference_date(&self) -> Date {
        return self.date_reference;
    }
}

impl YieldProvider for SpreadedTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
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
    ) -> Result<f64, YieldProviderError> {
        let spread_forward_rate = self
            .spread_curve
            .forward_rate(start_date, end_date, comp, freq)?;
        let base_forward_rate = self
            .base_curve
            .forward_rate(start_date, end_date, comp, freq)?;
        return Ok(spread_forward_rate + base_forward_rate);
    }
}

impl YieldTermStructureTrait for SpreadedTermStructure {}

#[cfg(test)]
mod test {
    use crate::{
        rates::{
            enums::Compounding,
            interestrate::InterestRate,
            traits::{HasReferenceDate, YieldProvider},
            yieldtermstructure::{
                flatforwardtermstructure::FlatForwardTermStructure,
                spreadtermstructure::SpreadedTermStructure,
            },
        },
        time::{date::Date, daycounter::DayCounter, enums::Frequency},
    };

    #[test]
    fn test_reference_date() {
        let spread_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.01,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));

        let base_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.02,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));
        let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);
        assert!(spreaded_curve.reference_date() == Date::new(2020, 1, 1));
    }

    #[test]
    fn test_forward_rate() {
        let spread_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.01,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));

        let base_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.02,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));
        let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);

        let fr = spreaded_curve.forward_rate(
            Date::new(2020, 1, 1),
            Date::new(2022, 1, 1),
            Compounding::Compounded,
            Frequency::Annual,
        );
        println!("fr: {:?}", fr);
        assert!((fr.unwrap() - 0.03) < 0.0001);
    }

    #[test]
    fn test_discount_factor() {
        let spread_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.01,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));

        let base_curve = Box::new(FlatForwardTermStructure::new(
            Date::new(2020, 1, 1),
            InterestRate::new(
                0.02,
                Compounding::Compounded,
                Frequency::Annual,
                DayCounter::Actual360,
            ),
        ));

        let spreaded_curve = SpreadedTermStructure::new(spread_curve, base_curve);

        let df = spreaded_curve.discount_factor(Date::new(2021, 1, 1));
        println!("df: {:?}", df);

        assert!(df.unwrap() - 0.9702040771633191 < 0.00001);
    }
}
