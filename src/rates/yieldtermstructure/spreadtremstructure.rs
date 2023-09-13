use crate::{
    rates::traits::{HasReferenceDate, Spread},
    time::{date::Date, enums::Frequency},
    prelude::{YieldProvider, Compounding, DayCounter, InterestRate},
    math::interpolation::traits::Interpolate,
};


pub struct SpreadTermStructure<T,U> {
    reference_date: Date,
    year_fractions: Vec<f64>,  
    rates: Vec<f64>,
    interpolator: T,
    daycounter: DayCounter,
    compounding: Compounding,
    spread: U,
}

impl<T,U>   SpreadTermStructure<T,U> where T: Interpolate<T>, U: Spread<U> {
    pub fn new(reference_date: Date, year_fractions: Vec<f64>, rates: Vec<f64>, interpolator: T, daycounter: DayCounter, compounding: Compounding, spread: U) -> SpreadTermStructure<T,U> {
        // check if year_fractions and rates have the same size
        if year_fractions.len() != rates.len() {
            panic!("year_fractions and rates should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if year_fractions[0] != 0.0 {
            panic!("year_fractions[0] needs to be 0.0");
        }

        // rates[0] needs to be 0.0
        if rates[0] != 0.0 {
            panic!("rates[0] needs to be 0.0");
        }   

        SpreadTermStructure {
            reference_date,
            year_fractions,
            rates,
            interpolator,
            daycounter,
            compounding,
            spread,
        }
    }

    pub fn year_fractions(&self) -> &Vec<f64> {
        return &self.year_fractions;
    }

    pub fn rates(&self) -> &Vec<f64> {
        return &self.rates;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.daycounter;
    }

    pub fn compounding(&self) -> Compounding {
        return self.compounding;
    }

    pub fn calculate_compound(&self,rate: f64, year_fraction: f64) -> f64 {
        let compound : f64;

        match self.compounding() {
            Compounding::Simple => compound = 1.0 + rate * year_fraction,
            Compounding::Compounded =>  compound = (1.0 + rate).powf(year_fraction),
            Compounding::Continuous => compound = (1.0 + rate).exp() * year_fraction,
            Compounding::SimpleThenCompounded => {
                if year_fraction <= 1.0 {
                    compound = 1.0 + rate * year_fraction
                } else {
                    compound = (1.0 + rate).powf(year_fraction)
                }
            }
            Compounding::CompoundedThenSimple => {
                if year_fraction <= 1.0 {
                    compound =  (1.0 + rate).powf(year_fraction)
                } else {
                    compound =  1.0 + rate * year_fraction
                }
            }
        }
        
        return compound;

    }
    

}

impl<T,U> HasReferenceDate for SpreadTermStructure<T,U> {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl<T,U> YieldProvider for SpreadTermStructure<T,U> where T: Interpolate<T>, U: Spread<U> {
    fn discount_factor(&self, date: Date ) -> f64 {
        if date < self.reference_date() {
            panic!("date must be greater than reference date");
        }
        if date == self.reference_date() {
            return 1.0;
        }

        let spread = self.spread.return_spread_to_date(0.0);

        let year_fraction = self.day_counter().year_fraction(self.reference_date(), date);
        let rate = self.interpolator.interpolate(year_fraction) + spread;

        let discount_factor = 1.0/self.calculate_compound(rate, year_fraction);

        return discount_factor;
    }

    fn forward_rate( &self, start_date: Date, end_date: Date, comp: Compounding, freq: Frequency) -> f64 {

        let delta_year_fraction_to_star = self.day_counter().year_fraction(self.reference_date(), start_date);
        let delta_year_fraction_to_end = self.day_counter().year_fraction(self.reference_date(), end_date);

        let rate_to_star = self.interpolator.interpolate(delta_year_fraction_to_star);
        let rate_to_end = self.interpolator.interpolate(delta_year_fraction_to_end);

        let spread_to_star = self.spread.return_spread_to_date(delta_year_fraction_to_star);
        let spread_to_end = self.spread.return_spread_to_date(delta_year_fraction_to_end);

        let compound_to_star = self.calculate_compound(rate_to_star + spread_to_star, delta_year_fraction_to_star);
        let compound_to_end = self.calculate_compound(rate_to_end + spread_to_end, delta_year_fraction_to_end);

        let comp_factor = compound_to_end/compound_to_star;

        let t = self.day_counter().year_fraction(start_date, end_date);

        let forward_rate = InterestRate::implied_rate(comp_factor, self.day_counter(), comp, freq, t).rate();

        return forward_rate;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{math::interpolation::{linear::LinearInterpolator, traits::Interpolate}, 
                time::daycounter::DayCounter, 
                rates::spread::{constantspread::ConstantSpread, curvespread::CurveSpread}
            };

    #[test]
    fn test_new() {
        
        let reference_date = Date::new(2020, 1, 1);
        let year_fractions = vec![0.0, 1.0, 2.0];
        let rates = vec![0.0, 1.0, 4.0];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        let spread = ConstantSpread::new(0.1);

        let spreadtermstructure = SpreadTermStructure::new(reference_date, year_fractions, rates, interpolator, daycounter, compounding, spread);
       
       
        assert_eq!(spreadtermstructure.year_fractions, vec![0.0, 1.0, 2.0]);
        assert_eq!(spreadtermstructure.rates, vec![0.0, 1.0, 4.0]);
        assert_eq!(spreadtermstructure.daycounter, daycounter);
        assert_eq!(spreadtermstructure.spread.return_spread_to_date(0.0), 0.1);
    }

    #[test]
    fn test_reference_date() {
        let reference_date = Date::new(2020, 1, 1);
        let year_fractions = vec![0.0, 1.0, 2.0];
        let rates = vec![0.0, 1.0, 4.0];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        let spread = ConstantSpread::new(0.1);
        let spreadtermstructure = SpreadTermStructure::new(reference_date, year_fractions, rates, interpolator, daycounter,compounding ,spread);
       
        assert_eq!(spreadtermstructure.reference_date(), Date::new(2020, 1, 1));
    }

    #[test]
    fn test_discount_factors(){
        let reference_date = Date::new(2020, 1, 1);
        let year_fractions = vec![0.0, 1.0, 2.0];
        let rates = vec![0.0, 1.0, 4.0];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        let spread = ConstantSpread::new(0.1);
        let spreadtermstructure = SpreadTermStructure::new(reference_date, year_fractions, rates, interpolator, daycounter, compounding,spread);
       
        //println!("discount factor: {}", spreadtermstructure.discount_factor(Date::new(2020, 12, 31)));
        
        assert_eq!( spreadtermstructure.discount_factor(Date::new(2020, 12, 31)) , 1.0/2.1);
        //assert_eq!(spreadtermstructure.discount_factor(Date::new(2021, 1, 1)), 0.9090909090909091);
        //assert_eq!(spreadtermstructure.discount_factor(Date::new(2022, 1, 1)), 0.8264462809917356);
    }

    #[test]
    fn test_forward_rate() {
        let reference_date = Date::new(2020, 1, 1);
        let year_fractions = vec![0.0, 1.0, 2.0];
        let rates = vec![0.0, 1.0, 4.0];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        let spread = ConstantSpread::new(0.1);
        let spreadtermstructure = SpreadTermStructure::new(reference_date, year_fractions, rates, interpolator, daycounter, compounding, spread);
       

        println!("forward rate: {}", spreadtermstructure.forward_rate(Date::new(2020, 1, 1), Date::new(2020, 12, 31), Compounding::Simple, Frequency::Annual));
        
        
        //assert_eq!(spreadtermstructure.forward_rate(Date::new(2020, 1, 1), Date::new(2020, 12, 31), Compounding::Simple, Frequency::Annual), 0.1);

    }

    #[test]
    fn test_discount_factors_spread_curve(){
        
        // *********************************************************************************************************************
        // FALTA REVISAR DE MANERA EXAUSTIVA
        // *********************************************************************************************************************


        let reference_date = Date::new(2020, 1, 1);
        let year_fractions = vec![0.0, 1.0, 2.0];
        let rates = vec![0.0, 1.0, 4.0];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        // spread curve definition
        let year_fractions_sprear = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let spread = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let interpolator_spread = LinearInterpolator::initialize(year_fractions_sprear.clone(),spread.clone(),Some(true));   
        let curve_spread = CurveSpread::new(year_fractions_sprear.clone(), spread.clone(), interpolator_spread, daycounter.clone(), compounding.clone());
        

        let spreadtermstructure = SpreadTermStructure::new(reference_date, year_fractions, rates, interpolator, daycounter, compounding, curve_spread);
    
        println!("discount factor: {}", spreadtermstructure.discount_factor(Date::new(2020, 12, 31)));
        
    }


}