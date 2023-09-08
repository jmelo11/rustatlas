use std::collections::HashMap;

use crate::{
    rates::{
        traits::{HasReferenceDate},
    },
    time::{date::Date, enums::Frequency, period::Period},
    prelude::{YieldProvider, Compounding},
};


#[derive(Clone)]
pub struct DiscountFactorTermStructure {
    reference_date: Date,
    discount_factors : HashMap<Date, f64>
}


impl DiscountFactorTermStructure {
    pub fn new(reference_date: Date, discount_factors: HashMap<Date, f64>) -> DiscountFactorTermStructure {
        DiscountFactorTermStructure {
            reference_date,
            discount_factors,
        }
    }

    pub fn discount_factors(&self) -> &HashMap<Date, f64> {
        return &self.discount_factors;
    }
}


impl HasReferenceDate for DiscountFactorTermStructure {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl YieldProvider for DiscountFactorTermStructure {
    
    fn discount_factor(&self, date: Date) -> f64 {
        if date < self.reference_date() {
            panic!("date must be greater than reference date");
        }
        
        
    }
    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> f64 {
        let x = 10.0;
        x
    }
}


#[cfg(test)]
mod tests {
    use super::DiscountFactorTermStructure;
    use super::*;

    #[test]
    fn test_reference_date() {
        let reference_date = Date::new(2023, 8, 19);
        let mut discount_factors = HashMap::new();

        discount_factors.insert(Date::new(2023, 9 , 1), 1.0);
        discount_factors.insert(Date::new(2023, 10, 1), 0.99);
        discount_factors.insert(Date::new(2023, 11, 1), 0.98);
        discount_factors.insert(Date::new(2023, 12, 1), 0.97);
        discount_factors.insert(Date::new(2024,  1, 1), 0.96);

        let term_structure = DiscountFactorTermStructure::new(reference_date, discount_factors);

        assert_eq!(term_structure.reference_date(), reference_date);
    }

    #[test]
    fn test_discount_factors() {
        let reference_date = Date::new(2023, 8, 19);
        let mut discount_factors = HashMap::new();

        discount_factors.insert(Date::new(2023, 9 , 1), 1.0);
        discount_factors.insert(Date::new(2023, 10, 1), 0.99);
        discount_factors.insert(Date::new(2023, 11, 1), 0.98);
        discount_factors.insert(Date::new(2023, 12, 1), 0.97);
        discount_factors.insert(Date::new(2024,  1, 1), 0.96);

        let term_structure = DiscountFactorTermStructure::new(reference_date, discount_factors);

        assert_eq!(term_structure.discount_factors().len(), 5);
        assert_eq!(term_structure.discount_factors().get(&Date::new(2023, 9 , 1)), Some(&1.0));
        assert_eq!(term_structure.discount_factors().get(&Date::new(2023, 10, 1)), Some(&0.99));
        assert_eq!(term_structure.discount_factors().get(&Date::new(2023, 11, 1)), Some(&0.98));
        assert_eq!(term_structure.discount_factors().get(&Date::new(2023, 12, 1)), Some(&0.97));
        assert_eq!(term_structure.discount_factors().get(&Date::new(2024,  1, 1)), Some(&0.96));
    }
}