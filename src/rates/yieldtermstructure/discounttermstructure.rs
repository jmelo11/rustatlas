use crate::{
    rates::traits::HasReferenceDate,
    time::{date::Date, enums::Frequency},
    prelude::{YieldProvider, Compounding, DayCounter, InterestRate},
    math::interpolation::traits::Interpolate,
};

#[derive(Clone)]
pub struct DiscountTermStructure<T> {
    reference_date: Date,
    year_fractions: Vec<f64>,
    discount_factors: Vec<f64>,
    interpolator: T,
    daycounter: DayCounter,
}

impl<T> DiscountTermStructure<T> where T: Interpolate<T> {
    pub fn new(reference_date: Date, year_fractions: Vec<f64>,discount_factors: Vec<f64>, interpolator: T, daycounter: DayCounter) -> DiscountTermStructure<T> {
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
            daycounter,
        }
    }

    pub fn year_fractions(&self) -> &Vec<f64> {
        return &self.year_fractions;
    }

    pub fn discount_factors(&self) -> &Vec<f64> {
        return &self.discount_factors;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.daycounter;
    }

}

impl<T> HasReferenceDate for DiscountTermStructure<T> {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}
    
 impl<T> YieldProvider for DiscountTermStructure<T> where T: Interpolate<T> {
  
     fn discount_factor(&self, date: Date ) -> f64 {
         if date < self.reference_date() {
             panic!("date must be greater than reference date");
         }
         if date == self.reference_date() {
             return 1.0;
         }

         let delta_year_fraction = self.day_counter().year_fraction(self.reference_date(), date);

         let discount_factor = self.interpolator.interpolate(delta_year_fraction);
         return discount_factor;

     }
     fn forward_rate( &self, start_date: Date, end_date: Date, comp: Compounding, freq: Frequency) -> f64 {
        
        let delta_year_fraction_to_star = self.day_counter().year_fraction(self.reference_date(), start_date);
        let delta_year_fraction_to_end = self.day_counter().year_fraction(self.reference_date(), end_date);

        let discount_factor_to_star = self.interpolator.interpolate(delta_year_fraction_to_star);
        let discount_factor_to_end = self.interpolator.interpolate(delta_year_fraction_to_end);

        let comp_factor =  discount_factor_to_star / discount_factor_to_end;
        let t = self.day_counter().year_fraction(start_date, end_date);
        return InterestRate::implied_rate(comp_factor, self.day_counter(), comp, freq, t).rate();
        
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{math::interpolation::{linear::LinearInterpolator, traits::Interpolate}, time::daycounter::DayCounter};

    #[test]
    fn test_year_fractions() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        let daycounter = DayCounter::Actual360;
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator, daycounter);

        assert_eq!(discount_term_structure.year_fractions(), &vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    }


    #[test]
    fn test_discount_dactors() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        let daycounter = DayCounter::Actual360;

        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator, daycounter);

        assert_eq!(discount_term_structure.discount_factors(), &vec![1.0, 0.99, 0.98, 0.97, 0.96]);

    }
        

    #[test]
    fn test_reference_date(){
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        let daycounter = DayCounter::Actual360;
        
        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator, daycounter);

        assert_eq!(discount_term_structure.reference_date(), Date::new(2020, 1, 1));

    }

   #[test]
    fn test_interpolation() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        let daycounter = DayCounter::Actual365;

        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator, daycounter);

        assert!((discount_term_structure.discount_factor(Date::new(2020, 6, 1))-0.9833424657534247).abs() < 1e-8);
        //println!("discount_factor: {}", discount_term_structure.discount_factor(Date::new(2020, 6, 1)));

    }

    #[test]

    fn test_forward_rate() {
        let reference_date = Date::new(2020, 1, 1); 
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), discount_factors.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let comp = Compounding::Simple;
        let freq = Frequency::Annual;


        let discount_term_structure = DiscountTermStructure::new(reference_date, year_fractions, discount_factors, interpolator, daycounter);

        assert!((discount_term_structure.forward_rate(Date::new(2020, 1, 1), Date::new(2020, 12, 31), comp, freq)-(1.0/0.96-1.0)).abs() < 1e-8);

        //println!("forward_rate: {}", discount_term_structure.forward_rate(Date::new(2020, 1, 1), Date::new(2021, 12, 31), comp, freq));
        //println!("discount_factor: {}", discount_term_structure.discount_factor(Date::new(2020, 12, 31)));
        //let delta_year_fraction = daycounter.year_fraction(reference_date, Date::new(2020, 12, 31));      
        //println!("delta_year_fraction: {}", delta_year_fraction);
    }



}