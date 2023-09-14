use crate::{
    rates::traits::{HasReferenceDate, YieldProviderError},
    time::{date::Date, enums::Frequency},
    prelude::{YieldProvider, Compounding, DayCounter, InterestRate},
    math::interpolation::{traits::Interpolate, linear::LinearInterpolator, loglinear::LogLinearInterpolator},
};


pub struct ZeroRateCurve<T> {
    reference_date: Date,
    dates: Vec<Date>,
    rates: Vec<f64>,
    interpolator: T,
    daycounter: DayCounter,
    compounding: Compounding,
}

pub trait MakeNew<T> {
    fn new(reference_date: Date, dates: Vec<Date>, rates: Vec<f64>, daycounter: DayCounter, compounding: Compounding) -> ZeroRateCurve<T>;
}

impl MakeNew<LinearInterpolator> for ZeroRateCurve<LinearInterpolator> {
    fn new(reference_date: Date, dates: Vec<Date> , rates: Vec<f64>, daycounter: DayCounter, compounding: Compounding) -> ZeroRateCurve<LinearInterpolator> {
        // check if dates and rates have the same size
        if dates.len() != rates.len() {
            panic!("Dates and rates should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if dates[0] != reference_date {
            panic!("Dates[0] needs to be reference_date");
        }

        let year_fractions: Vec<f64>  = dates.iter().map(|x| daycounter.year_fraction(reference_date, *x)).collect();
        let interpolator: LinearInterpolator = LinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));

        ZeroRateCurve {
            reference_date,
            dates,
            rates,
            interpolator,
            daycounter,
            compounding,
        }
    }


}

impl MakeNew<LogLinearInterpolator> for ZeroRateCurve<LogLinearInterpolator> {
    fn new(reference_date: Date, dates: Vec<Date> , rates: Vec<f64>, daycounter: DayCounter, compounding: Compounding) -> ZeroRateCurve<LogLinearInterpolator> {
        // check if dates and rates have the same size
        if dates.len() != rates.len() {
            panic!("Dates and rates should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if dates[0] != reference_date {
            panic!("Dates[0] needs to be reference_date");
        }

        let year_fractions: Vec<f64>  = dates.iter().map(|x| daycounter.year_fraction(reference_date, *x)).collect();
        let interpolator: LogLinearInterpolator = LogLinearInterpolator::initialize(year_fractions.clone(), rates.clone(), Some(true));

        ZeroRateCurve {
            reference_date,
            dates,
            rates,
            interpolator,
            daycounter,
            compounding,
        }
    }
}

impl<T> ZeroRateCurve<T> {
    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
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
        let compound: f64;

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

impl<T> HasReferenceDate for ZeroRateCurve<T> where T: Interpolate {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}
    
impl<T> YieldProvider for ZeroRateCurve<T> where T: Interpolate {
 
    fn discount_factor(&self, date: Date ) -> Result<f64, YieldProviderError> {
        let year_fraction = self.day_counter().year_fraction(self.reference_date(), date);
        let rate = self.interpolator.interpolate(year_fraction);
         
        let compound = self.calculate_compound(rate, year_fraction);

        return Ok(1.0 / compound);
    }

    fn forward_rate( &self, start_date: Date, end_date: Date, comp: Compounding, freq: Frequency) -> Result<f64, YieldProviderError> {
        let delta_year_fraction_to_star = self.day_counter().year_fraction(self.reference_date(), start_date);
        let delta_year_fraction_to_end = self.day_counter().year_fraction(self.reference_date(), end_date);

        let rate_to_star = self.interpolator.interpolate(delta_year_fraction_to_star);
        let rate_to_end = self.interpolator.interpolate(delta_year_fraction_to_end);

        let compound_to_star = self.calculate_compound(rate_to_star, delta_year_fraction_to_star);
        let compound_to_end = self.calculate_compound(rate_to_end, delta_year_fraction_to_end);

        let comp_factor = compound_to_end/compound_to_star;

        let t = self.day_counter().year_fraction(start_date, end_date);

        let forward_rate = (InterestRate::implied_rate(comp_factor, self.day_counter(), comp, freq, t)?).rate();

        return Ok(forward_rate);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{time::daycounter::DayCounter};

    #[test]
    fn test_zero_rate_curve() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![Date::new(2020, 1, 1), Date::new(2020, 4, 1), Date::new(2020, 7, 1), Date::new(2020, 10, 1), Date::new(2021, 1, 1)];
        let rates = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let daycounter = DayCounter::Actual365;
        let compounding = Compounding::Simple;

        let zero_rate_curve: ZeroRateCurve<LinearInterpolator> = ZeroRateCurve::new(
            reference_date, 
            dates,
            rates,
            daycounter, 
            compounding
        );

        assert_eq!(zero_rate_curve.reference_date(), reference_date);
        assert_eq!(zero_rate_curve.dates(), &vec![Date::new(2020, 1, 1), Date::new(2020, 4, 1), Date::new(2020, 7, 1), Date::new(2020, 10, 1), Date::new(2021, 1, 1)]);
        assert_eq!(zero_rate_curve.rates(), &vec![0.0, 0.01, 0.02, 0.03, 0.04]);
        assert_eq!(zero_rate_curve.day_counter(), DayCounter::Actual365);
        
    }
}








