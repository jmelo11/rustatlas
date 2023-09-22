use crate::{
    math::interpolation::traits::Interpolate,
    rates::traits::{HasReferenceDate, YieldProviderError},
    rates::{
        enums::Compounding, interestrate::InterestRate, traits::YieldProvider,
        yieldtermstructure::errortermstructure::TermStructureConstructorError,
    },
    time::{date::Date, daycounter::DayCounter, enums::Frequency},
};

#[derive(Clone)]
pub struct DiscountTermStructure<T: Interpolate> {
    reference_date: Date,
    dates: Vec<Date>,
    discount_factors: Vec<f64>,
    interpolator: T,
    day_counter: DayCounter,
}

impl<T: Interpolate> DiscountTermStructure<T> {
    pub fn new(
        reference_date: Date,
        dates: Vec<Date>,
        discount_factors: Vec<f64>,
        day_counter: DayCounter,
    ) -> Result<DiscountTermStructure<T>, TermStructureConstructorError> {
        // check if year_fractions and discount_factors have the same size
        if dates.len() != discount_factors.len() {
            return Err(TermStructureConstructorError::DatesAndDiscountFactorsSize);
        }

        // dates[0] needs to be equal to reference date
        if dates[0] != reference_date {
            return Err(TermStructureConstructorError::FirstDateNeedsToBeReferenceDate);
        }

        // discount_factors[0] needs to be 1.0
        if discount_factors[0] != 1.0 {
            return Err(TermStructureConstructorError::FirstDiscountFactorsNeedsToBeOne);
        }

        let year_fractions: Vec<f64> = dates
            .iter()
            .map(|x| day_counter.year_fraction(reference_date, *x))
            .collect();

        let interpolator: T = T::new(year_fractions.clone(), discount_factors.clone(), Some(true));

        Ok(DiscountTermStructure {
            reference_date,
            dates,
            discount_factors,
            interpolator,
            day_counter,
        })
    }

    pub fn dates(&self) -> &Vec<Date> {
        return &self.dates;
    }

    pub fn discount_factors(&self) -> &Vec<f64> {
        return &self.discount_factors;
    }

    pub fn day_counter(&self) -> DayCounter {
        return self.day_counter;
    }
}

impl<T: Interpolate> HasReferenceDate for DiscountTermStructure<T> {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl<T: Interpolate> YieldProvider for DiscountTermStructure<T> {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        if date < self.reference_date() {
            return Err(YieldProviderError::DateMustBeGreaterThanReferenceDate);
        }
        if date == self.reference_date() {
            return Ok(1.0);
        }

        let delta_year_fraction = self
            .day_counter()
            .year_fraction(self.reference_date(), date);

        let discount_factor = self.interpolator.interpolate(delta_year_fraction);
        return Ok(discount_factor);
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
        let discount_factor_to_star = self.discount_factor(start_date)?;
        let discount_factor_to_end = self.discount_factor(end_date)?;

        let comp_factor = discount_factor_to_star / discount_factor_to_end;
        let t = self.day_counter().year_fraction(start_date, end_date);

        return Ok(
            InterestRate::implied_rate(comp_factor, self.day_counter(), comp, freq, t)?.rate(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::interpolation::{
        linear::LinearInterpolator, loglinear::LogLinearInterpolator,
    };

    #[test]
    fn test_year_fractions() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 4, 1),
            Date::new(2020, 7, 1),
            Date::new(2020, 10, 1),
            Date::new(2021, 1, 1),
        ];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let day_counter = DayCounter::Actual360;

        let discount_term_structure: DiscountTermStructure<LinearInterpolator> =
            DiscountTermStructure::new(reference_date, dates, discount_factors, day_counter)
                .unwrap();

        assert_eq!(
            discount_term_structure.dates(),
            &vec![
                Date::new(2020, 1, 1),
                Date::new(2020, 4, 1),
                Date::new(2020, 7, 1),
                Date::new(2020, 10, 1),
                Date::new(2021, 1, 1)
            ]
        );
    }

    #[test]
    fn test_discount_dactors() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 4, 1),
            Date::new(2020, 7, 1),
            Date::new(2020, 10, 1),
            Date::new(2021, 1, 1),
        ];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let day_counter = DayCounter::Actual360;

        let discount_term_structure: DiscountTermStructure<LinearInterpolator> =
            DiscountTermStructure::new(reference_date, dates, discount_factors, day_counter)
                .unwrap();

        assert_eq!(
            discount_term_structure.discount_factors(),
            &vec![1.0, 0.99, 0.98, 0.97, 0.96]
        );
    }

    #[test]
    fn test_reference_date() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 4, 1),
            Date::new(2020, 7, 1),
            Date::new(2020, 10, 1),
            Date::new(2021, 1, 1),
        ];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let day_counter = DayCounter::Actual360;

        let discount_term_structure: DiscountTermStructure<LogLinearInterpolator> =
            DiscountTermStructure::new(reference_date, dates, discount_factors, day_counter)
                .unwrap();

        assert_eq!(
            discount_term_structure.reference_date(),
            Date::new(2020, 1, 1)
        );
    }

    #[test]
    fn test_interpolation() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 4, 1),
            Date::new(2020, 7, 1),
            Date::new(2020, 10, 1),
            Date::new(2021, 1, 1),
        ];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let day_counter = DayCounter::Actual360;

        let discount_term_structure: DiscountTermStructure<LinearInterpolator> =
            DiscountTermStructure::new(reference_date, dates, discount_factors, day_counter)
                .unwrap();

        assert!(
            (discount_term_structure
                .discount_factor(Date::new(2020, 6, 1))
                .unwrap()
                - 0.9832967032967033)
                .abs()
                < 1e-8
        );
        //println!("discount_factor: {}", discount_term_structure.discount_factor(Date::new(2020, 6, 1)).unwrap());
    }

    #[test]

    fn test_forward_rate() {
        let reference_date = Date::new(2020, 1, 1);
        let dates = vec![
            Date::new(2020, 1, 1),
            Date::new(2020, 4, 1),
            Date::new(2020, 7, 1),
            Date::new(2020, 10, 1),
            Date::new(2021, 1, 1),
        ];
        let discount_factors = vec![1.0, 0.99, 0.98, 0.97, 0.96];
        let day_counter = DayCounter::Actual360;

        let discount_term_structure: DiscountTermStructure<LinearInterpolator> =
            DiscountTermStructure::new(reference_date, dates, discount_factors, day_counter)
                .unwrap();

        let comp = Compounding::Simple;
        let freq = Frequency::Annual;

        assert!(
            (discount_term_structure
                .forward_rate(Date::new(2020, 1, 1), Date::new(2020, 12, 31), comp, freq)
                .unwrap()
                - 0.04097957689796514)
                .abs()
                < 1e-8
        );
        println!(
            "forward_rate: {}",
            discount_term_structure
                .forward_rate(Date::new(2020, 1, 1), Date::new(2020, 12, 31), comp, freq)
                .unwrap()
        );
    }
}
