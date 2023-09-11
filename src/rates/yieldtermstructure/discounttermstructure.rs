use crate::{
    rates::{
        traits::{HasReferenceDate},
    },
    time::{date::Date, enums::Frequency},
    prelude::{YieldProvider, Compounding},
    math::interpolation,
};

#[derive(Clone)]
pub struct DiscountTermStructure<T> {
    reference_date: Date,
    year_fractions: Vec<f64>,
    discount_factors: Vec<f64>,
    interpolator: T,
}

impl<T> DiscountTermStructure<T> where T: interpolation::traits::Interpolate<T> {
    pub fn new(reference_date: Date, year_fractions: Vec<f64>,discount_factors: Vec<f64>, interpolator: T)-> DiscountTermStructure<T> {
        // check if year_fractions and discount_factors have the same size
        if year_fractions.len() != discount_factors.len() {
            panic!("year_fractions and discount_factors should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if year_fractions[0] != 0.0 {
            panic!("year_fractions[0] needs to be 0.0");
        }

        // discount_factors[0] needs to be 1.0
        if discount_factors[0] != 1.0 {
            panic!("discount_factors[0] needs to be 1.0");
        }

        DiscountTermStructure {
            reference_date,
            year_fractions,
            discount_factors,
            interpolator,
        }
    }

    pub fn year_fractions(&self) -> &Vec<f64> {
        return &self.year_fractions;
    }

    pub fn discount_factors(&self) -> &Vec<f64> {
        return &self.discount_factors;
    }
}

impl<T> HasReferenceDate for DiscountTermStructure<T> {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}
    
 impl<T> YieldProvider for DiscountTermStructure<T> where T: interpolation::traits::Interpolate<T> {
  
     fn discount_factor(&self, date: Date) -> f64 {
         if date < self.reference_date() {
             panic!("date must be greater than reference date");
         }
         if date == self.reference_date() {
             return 1.0;
         }

         let delta_year_fraction = (date - self.reference_date()) as f64 / 365.0;
         let discount_factor = self.interpolator.interpolate(delta_year_fraction);
         return discount_factor;

     }
     fn forward_rate( &self, start_date: Date, end_date: Date, comp: Compounding, freq: Frequency) -> f64 {
        
        
        if end_date < start_date {
            panic!("date must be greater than reference date");
        }
        if end_date == start_date {
            return 0.0;
        }

        let delta_year_fraction = (end_date - self.reference_date()) as f64 / 365.0;
        let discount_factor = self.interpolator.interpolate(delta_year_fraction);

        return discount_factor;

    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::interpolation::{linear::LinearInterpolator, traits::Interpolate};

    #[test]
    fn test_year_fractions() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator);

        assert_eq!(discount_term_structure.year_fractions(), &vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    }

    #[test]
    fn test_discount_dactors() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator);

        assert_eq!(discount_term_structure.discount_factors(), &vec![1.0, 0.99, 0.98, 0.97, 0.96]);

    }
        
    #[test]
    fn test_reference_date(){
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator);

        assert_eq!(discount_term_structure.reference_date(), Date::new(2020, 1, 1));

    }

    #[test]
    fn test_interpolation() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator);

        assert!((discount_term_structure.discount_factor(Date::new(2020, 6, 1))-0.9833424657534247).abs() < 1e-8);
        //println!("discount_factor: {}", discount_term_structure.discount_factor(Date::new(2020, 6, 1)));

    }


}