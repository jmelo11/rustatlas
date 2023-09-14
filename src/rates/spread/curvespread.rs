use crate::{
    rates::traits::Spread,
    time::{date::Date}, prelude::{DayCounter, Compounding}, 
    math::interpolation::traits::Interpolate,
};

pub struct CurveSpread<T> {
    year_fractions: Vec<f64>,  
    spread: Vec<f64>,
    interpolator: T,
    daycounter: DayCounter,
    compounding: Compounding,
}

impl<T> CurveSpread<T> where T: Interpolate {
    pub fn new(year_fractions: Vec<f64>, spread: Vec<f64>, interpolator: T, daycounter: DayCounter, compounding: Compounding) -> CurveSpread<T> {
        // check if year_fractions and rates have the same size
        if year_fractions.len() != spread.len() {
            panic!("year_fractions and spread should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if year_fractions[0] != 0.0 {
            panic!("year_fractions[0] needs to be 0.0");
        }

        CurveSpread {
            year_fractions,
            spread,
            interpolator,
            daycounter,
            compounding,
        }
    }

    pub fn year_fractions(&self) -> &Vec<f64> {
        return &self.year_fractions;
    }

    pub fn spread(&self) -> &Vec<f64> {
        return &self.spread;
    }

    pub fn daycounter(&self) -> &DayCounter {
        return &self.daycounter;
    }

    pub fn compounding(&self) -> &Compounding {
        return &self.compounding;
    }

    
}

impl<T> Spread<CurveSpread<T>> for CurveSpread<T> where T: Interpolate {
    fn return_spread_to_date(&self, year_fraction: f64) -> f64 {
        self.interpolator.interpolate(year_fraction)
    }
}


#[cfg(test)]
mod tests {
    use crate::math::interpolation::linear::LinearInterpolator;
    use super::*;

    #[test]
    fn test_curve_spread_new() {
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let spread = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(),spread.clone(),Some(true));   
        let daycounter = DayCounter::Actual360;
        let compounding = Compounding::Simple;

        let curve_spread = CurveSpread::new(year_fractions.clone(), spread.clone(), interpolator, daycounter.clone(), compounding.clone()); 

        assert_eq!(curve_spread.year_fractions(), &year_fractions);
        assert_eq!(curve_spread.spread(), &spread);
        assert_eq!(curve_spread.daycounter(), &daycounter);
        assert_eq!(curve_spread.compounding(), &compounding);
    }

    #[test]
    fn test_interpolation(){
        let year_fractions = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let spread = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let interpolator = LinearInterpolator::initialize(year_fractions.clone(),spread.clone(),Some(true));   
        let daycounter = DayCounter::Actual360;
        let compounding = Compounding::Simple;

        let curve_spread = CurveSpread::new(year_fractions.clone(), spread.clone(), interpolator, daycounter.clone(), compounding.clone()); 

        assert_eq!(curve_spread.return_spread_to_date(0.75), 0.03);
        //print!("spread: {}", curve_spread.return_spread_to_date(1.25));
        assert_eq!(curve_spread.return_spread_to_date(1.25), 0.05);   

    }


}