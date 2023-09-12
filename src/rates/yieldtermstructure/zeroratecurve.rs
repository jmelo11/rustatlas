
pub struct ZeroRateCurve<T> {
    reference_date: Date,
    year_fractions: Vec<f64>,
    rates: Vec<f64>,
    interpolator: T,
    daycounter: DayCounter,
}

impl<T> ZeroRateCurve<T> where T: Interpolate<T> {
    pub fn new(reference_date: Date, dates: Vec<Date>, rates: Vec<f64>, interpolator: T, daycounter: DayCounter) -> ZeroRateCurve<T> {
        // check if dates and rates have the same size
        if year_fraction.len() != rates.len() {
            panic!("dates and rates should have the same size.");
        }

        // year_fractions[0] needs to be 0.0
        if dates[0] != 0.0 {
            panic!("dates[0] needs to be 0.0");
        }

        ZeroRateCurve {
            reference_date,
            dates,
            rates,
            interpolator,
            daycounter,
        }
    }

    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
    }

    pub fn rates(&self) -> &Vec<f64> {
        return &self.rates;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.daycounter;
    }

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_zero_rate_curve() {
        let reference_date = Date::from_ymd(2019, 1, 1);
        let year_fraction = vec![0.0, 0.25, 0.5, 0.75, 1.0]; 
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let daycounter = DayCounter::Actual365;

        let interpolator = LinearInterpolator::initialize(
            year_fractions.clone(),
            discount_factors.clone(),
            Some(true)
        );

        let zero_rate_curve = ZeroRateCurve::new(reference_date, dates, rates, interpolator, daycounter);

        assert_eq!(zero_rate_curve.reference_date(), reference_date);
        assert_eq!(zero_rate_curve.dates(), &dates);
        assert_eq!(zero_rate_curve.rates(), &rates);
        assert_eq!(zero_rate_curve.day_counter(), daycounter);
    }
}








