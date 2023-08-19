use crate::rates::enums::Compounding;
use crate::time::date::Date;
use crate::time::daycounters::enums::DayCounter;
use crate::time::enums::Frequency;
use std::sync::Arc;

pub trait DiscountFactor {
    fn compound_factor(&self, start: Date, end: Date) -> f64;
    fn discount_factor(&self, start: Date, end: Date) -> f64 {
        return 1.0 / self.compound_factor(start, end);
    }
}

#[derive(Clone, Copy)]
pub struct RateDefinition {
    compounding: Compounding,
    frequency: Frequency,
    day_counter: DayCounter,
}

impl RateDefinition {
    pub fn new(
        compounding: Compounding,
        frequency: Frequency,
        day_counter: DayCounter,
    ) -> RateDefinition {
        RateDefinition {
            compounding,
            frequency,
            day_counter,
        }
    }

    pub fn compounding(&self) -> Compounding {
        return self.compounding;
    }

    pub fn frequency(&self) -> Frequency {
        return self.frequency;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.day_counter;
    }
}

#[derive(Clone, Copy)]
pub struct InsterestRate {
    rate: f64,
    rate_definition: RateDefinition,
}

impl InsterestRate {
    pub fn new(
        rate: f64,
        compounding: Compounding,
        frequency: Frequency,
        day_counter: DayCounter,
    ) -> InsterestRate {
        InsterestRate {
            rate,
            rate_definition: RateDefinition::new(compounding, frequency, day_counter),
        }
    }

    pub fn rate(&self) -> f64 {
        return self.rate;
    }

    pub fn rate_definition(&self) -> &RateDefinition {
        return &self.rate_definition;
    }

    pub fn compounding(&self) -> Compounding {
        return self.rate_definition.compounding();
    }

    pub fn frequency(&self) -> Frequency {
        return self.rate_definition.frequency();
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.rate_definition.day_counter();
    }
}

impl DiscountFactor for InsterestRate {
    fn compound_factor(&self, start: Date, end: Date) -> f64 {
        let rate = self.rate();
        let compounding = self.compounding();
        let day_counter = self.day_counter();
        let year_fraction = day_counter.year_fraction(start, end);
        match compounding {
            Compounding::Simple => 1.0 + rate * year_fraction,
            Compounding::Compounded => (1.0 + rate).powf(year_fraction),
            Compounding::Continuous => (1.0 + rate).exp() * year_fraction,
            Compounding::SimpleThenCompounded => {
                if year_fraction <= 1.0 {
                    1.0 + rate * year_fraction
                } else {
                    (1.0 + rate).powf(year_fraction)
                }
            }
            Compounding::CompoundedThenSimple => {
                if year_fraction <= 1.0 {
                    (1.0 + rate).powf(year_fraction)
                } else {
                    1.0 + rate * year_fraction
                }
            }
        }
    }
}
