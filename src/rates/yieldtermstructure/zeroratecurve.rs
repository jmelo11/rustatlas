use interestrate::RateDefinition;

use crate::{
    rates::{traits::{HasReferenceDate, YieldProviderError}, interestrate},
    time::{date::Date, enums::Frequency},
    prelude::{YieldProvider, Compounding, InterestRate},
    math::interpolation::{traits::Interpolate},
    rates::yieldtermstructure::errortermstructure::TermStructureConstructorError,
};

#[derive(Clone)]
pub struct ZeroRateCurve<T: Interpolate> {
    reference_date: Date,
    dates: Vec<Date>,
    rates: Vec<f64>,
    interpolator: T,
    ratedefinition: RateDefinition,
}

impl<T: Interpolate> ZeroRateCurve<T> {
    pub fn new(reference_date: Date, dates: Vec<Date> , rates: Vec<f64>, ratedefinition: RateDefinition) -> Result<ZeroRateCurve<T>, TermStructureConstructorError> {
        // check if dates and rates have the same size
        if dates.len() != rates.len() {
            return Err(TermStructureConstructorError::DatesAndRatesSize);
        }

        // year_fractions[0] needs to be 0.0
        if dates[0] != reference_date {
            return Err(TermStructureConstructorError::Dates0NeedsToBeReferenceDate);
        }

        let year_fractions: Vec<f64>  = dates.iter().map(|x| ratedefinition.day_counter().year_fraction(reference_date, *x)).collect();
        let interpolator: T = T::initialize(year_fractions.clone(), rates.clone(), Some(true));

        Ok(
            ZeroRateCurve {
            reference_date,
            dates,
            rates,
            interpolator,
            ratedefinition,
        }
        )
    }
    
    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
    }

    pub fn rates(&self) -> &Vec<f64> {
        return &self.rates;
    }

    pub fn rate_definition(&self) -> RateDefinition{
        return self.ratedefinition;
    }

}

impl<T: Interpolate> HasReferenceDate for ZeroRateCurve<T> {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}
    
impl<T: Interpolate> YieldProvider for ZeroRateCurve<T> {
 
    fn discount_factor(&self, date: Date ) -> Result<f64, YieldProviderError> {
        let year_fraction = self.rate_definition().day_counter().year_fraction(self.reference_date(), date);
        let rate = self.interpolator.interpolate(year_fraction);
         
        let rt = InterestRate::from_rate_definition(
            rate,
            self.rate_definition()
        );

        let compound = rt.compound_factor_from_yf(year_fraction);

        return Ok(1.0 / compound);
    }

    fn forward_rate( &self, start_date: Date, end_date: Date, comp: Compounding, freq: Frequency) -> Result<f64, YieldProviderError> {
        let df_to_star = self.discount_factor(start_date)?;
        let df_to_end = self.discount_factor(end_date)?;

        let comp_factor = df_to_star/df_to_end;

        let t = self.rate_definition().day_counter().year_fraction(start_date, end_date);

        let forward_rate = (InterestRate::implied_rate(comp_factor, self.rate_definition().day_counter(), comp, freq, t)?).rate();

        return Ok(forward_rate);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{time::daycounter::DayCounter, math::interpolation::linear::LinearInterpolator};

    #[test]
    fn test_zero_rate_curve() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![Date::new(2020, 1, 1), Date::new(2020, 4, 1), Date::new(2020, 7, 1), Date::new(2020, 10, 1), Date::new(2021, 1, 1)];
        let rates = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let rate_definition = RateDefinition::default();

        let zero_rate_curve: ZeroRateCurve<LinearInterpolator> = ZeroRateCurve::new(
            reference_date, 
            dates,
            rates,
            rate_definition,
        ).unwrap();

        assert_eq!(zero_rate_curve.reference_date(), reference_date);
        assert_eq!(zero_rate_curve.dates(), &vec![Date::new(2020, 1, 1), Date::new(2020, 4, 1), Date::new(2020, 7, 1), Date::new(2020, 10, 1), Date::new(2021, 1, 1)]);
        assert_eq!(zero_rate_curve.rates(), &vec![0.0, 0.01, 0.02, 0.03, 0.04]);
        assert_eq!(zero_rate_curve.rate_definition().day_counter(), DayCounter::Actual360);
        
    }
    
    #[test]
    fn test_forward_rate(){
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![Date::new(2020, 1, 1), Date::new(2021, 1, 1), Date::new(2022, 1, 1), Date::new(2023, 1, 1), Date::new(2024, 1, 1)];
        let rates = vec![0.0, 0.01, 0.02, 0.03, 0.04];
        let rate_definition = RateDefinition::default();

        let zero_rate_curve: ZeroRateCurve<LinearInterpolator> = ZeroRateCurve::new(
            reference_date, 
            dates,
            rates,
            rate_definition,
        ).unwrap();

        let fr = zero_rate_curve.forward_rate(Date::new(2021, 1, 1), Date::new(2022, 1, 1), rate_definition.compounding(), rate_definition.frequency());

        println!("fr: {:?}", fr);
        assert!(fr.unwrap() - 0.02972519115024655 <0.000000001);
    }

}








