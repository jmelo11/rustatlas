use crate::currencies::enums::Currency;

use super::{
    interestrateindex::enums::InterestRateIndex, yieldtermstructure::enums::YieldTermStructure,
};

/// # CurveContext
/// Struct that defines a curve context. A curve context is a combination of a yield term structure,
/// an interest rate index and a currency, all together with a name and an id.
///
/// # Example
/// ```
/// use rustatlas::{
///     rates::{
///         enums::Compounding,
///         interestrate::{InterestRate, RateDefinition},
///         interestrateindex::{
///             enums::InterestRateIndex,
///             iborindex::IborIndex,
///         },
///         yieldtermstructure::{
///             enums::YieldTermStructure,
///             flatforwardtermstructure::FlatForwardTermStructure
///         },
///         yieldtermstructuremanager::YieldTermStructureManager,
///     },
///     currencies::enums::Currency,
///     time::{
///         date::Date,
///         daycounters::enums::DayCounter,
///         enums::{Frequency, TimeUnit},
///         period::Period,
///     },
/// };
/// let mut yield_term_structure_manager = YieldTermStructureManager::new();
/// let today = Date::from_ymd(2020, 1, 1);
/// let rate = InterestRate::new(0.01, Compounding::Simple, Frequency::Annual, DayCounter::Actual360);
/// let term_structure = YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(today, rate));
///
/// let tenor = Period::new(1, TimeUnit::Months);
/// let index = InterestRateIndex::IborIndex(IborIndex::new(tenor));
/// let currency = Currency::EUR;
/// yield_term_structure_manager.add_curve_context("EUR1M".to_string(), term_structure, index, currency);
/// let curve_context = yield_term_structure_manager.get_curve_context_by_name("EUR1M".to_string()).unwrap();
///
/// assert_eq!(curve_context.id(), 0);
/// ```

#[derive(Clone)]
pub struct CurveContext {
    id: usize,
    term_structure: YieldTermStructure,
    interest_rate_index: InterestRateIndex,
    currency: Currency,
}

impl CurveContext {
    fn new(
        id: usize,
        term_structure: YieldTermStructure,
        interest_rate_index: InterestRateIndex,
        currency: Currency,
    ) -> CurveContext {
        CurveContext {
            id,
            term_structure,
            interest_rate_index,
            currency,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn term_structure(&self) -> &YieldTermStructure {
        &self.term_structure
    }

    pub fn interest_rate_index(&self) -> &InterestRateIndex {
        &self.interest_rate_index
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
}

/// # YieldTermStructureManager
/// Struct that manages all the curve contexts.
/// It is possible to add a curve context by name and retrieve it by name or id.
#[derive(Clone)]
pub struct YieldTermStructureManager {
    contexts: Vec<CurveContext>,
    names: Vec<String>,
}

impl YieldTermStructureManager {
    pub fn new() -> YieldTermStructureManager {
        YieldTermStructureManager {
            contexts: Vec::new(),
            names: Vec::new(),
        }
    }

    pub fn add_curve_context(
        &mut self,
        name: String,
        term_structure: YieldTermStructure,
        interest_rate_index: InterestRateIndex,
        currency: Currency,
    ) {
        let id = self.contexts.len();
        let context = CurveContext::new(id, term_structure, interest_rate_index, currency);
        self.contexts.push(context);
        self.names.insert(name, id);
    }

    pub fn get_curve_context_by_name(&self, name: String) -> Option<&CurveContext> {
        self.names
            .iter()
            .find(|(n, _)| n == &name)
            .map(|(_, id)| self.contexts.get(*id));
    }

    pub fn get_curve_context_by_id(&self, id: usize) -> Option<&CurveContext> {
        self.contexts.get(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        rates::{
            enums::Compounding, interestrate::InterestRate,
            interestrateindex::iborindex::IborIndex,
            yieldtermstructure::flatforwardtermstructure::FlatForwardTermStructure,
        },
        time::{
            date::Date,
            daycounters::enums::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
    };

    use super::*;
    // Add any additional imports needed for testing

    #[test]
    fn test_curve_context_new() {
        let term_structure =
            YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(
                Date::from_ymd(2020, 1, 1),
                InterestRate::new(
                    0.01,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            ));
        let index = InterestRateIndex::IborIndex(IborIndex::new(Period::new(1, TimeUnit::Months)));
        let currency = Currency::USD;

        let context = CurveContext::new(1, term_structure, index, currency);

        assert_eq!(context.id(), 1);
        assert_eq!(context.currency(), Currency::USD);
        // Add more assertions to validate `term_structure` and `index`
    }

    #[test]
    fn test_yield_term_structure_manager_add_and_get_by_name() {
        let mut manager = YieldTermStructureManager::new();

        let term_structure =
            YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(
                Date::from_ymd(2020, 1, 1),
                InterestRate::new(
                    0.01,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            ));
        let index = InterestRateIndex::IborIndex(IborIndex::new(Period::new(1, TimeUnit::Months)));
        let currency = Currency::USD;

        manager.add_curve_context("curve1".to_string(), term_structure, index, currency);

        let context = manager
            .get_curve_context_by_name("curve1".to_string())
            .unwrap();
        assert_eq!(context.id(), 0);
        assert_eq!(context.currency(), Currency::USD);
        // Add more assertions to validate `term_structure` and `index`
    }

    #[test]
    fn test_yield_term_structure_manager_get_by_id() {
        let mut manager = YieldTermStructureManager::new();

        let term_structure1 =
            YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(
                Date::from_ymd(2020, 1, 1),
                InterestRate::new(
                    0.01,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            ));
        let index1 = InterestRateIndex::IborIndex(IborIndex::new(Period::new(1, TimeUnit::Months)));
        let currency1 = Currency::USD;

        manager.add_curve_context("curve1".to_string(), term_structure1, index1, currency1);

        let term_structure2 =
            YieldTermStructure::FlatForwardTermStructure(FlatForwardTermStructure::new(
                Date::from_ymd(2020, 1, 1),
                InterestRate::new(
                    0.01,
                    Compounding::Simple,
                    Frequency::Annual,
                    DayCounter::Actual360,
                ),
            ));
        let index2 = InterestRateIndex::IborIndex(IborIndex::new(Period::new(1, TimeUnit::Months)));
        let currency2 = Currency::EUR;

        manager.add_curve_context("curve2".to_string(), term_structure2, index2, currency2);

        let context = manager.get_curve_context_by_id(1).unwrap();
        assert_eq!(context.id(), 1);
        assert_eq!(context.currency(), Currency::EUR);
        // Add more assertions to validate `term_structure` and `index`
    }
}
