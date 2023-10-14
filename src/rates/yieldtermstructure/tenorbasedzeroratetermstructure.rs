use crate::{
    math::interpolation::enums::Interpolator,
    rates::{
        enums::Compounding,
        interestrate::InterestRate,
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
    },
    time::{date::Date, daycounter::DayCounter, enums::Frequency, period::Period},
};

pub struct TenorBasedZeroRateTermStructure {
    reference_date: Date,
    tenors: Vec<Period>,
    spreads: Vec<f64>,
    day_counter: DayCounter,
    compounding: Compounding,
    frequency: Frequency,
    year_fractions: Vec<f64>,
    interpolation: Interpolator,
    enable_extrapolation: bool,
}

impl TenorBasedZeroRateTermStructure {
    pub fn new(
        reference_date: Date,
        tenors: Vec<Period>,
        spreads: Vec<f64>,
        day_counter: DayCounter,
        compounding: Compounding,
        frequency: Frequency,
        interpolation: Interpolator,
        enable_extrapolation: bool,
    ) -> TenorBasedZeroRateTermStructure {
        let year_fractions = tenors
            .iter()
            .map(|x| {
                let date = reference_date + *x;
                day_counter.year_fraction(reference_date, date)
            })
            .collect();

        TenorBasedZeroRateTermStructure {
            reference_date,
            tenors,
            spreads,
            day_counter,
            compounding,
            frequency,
            year_fractions,
            interpolation,
            enable_extrapolation,
        }
    }

    pub fn tenors(&self) -> &Vec<Period> {
        return &self.tenors;
    }

    pub fn spreads(&self) -> &Vec<f64> {
        return &self.spreads;
    }
}

impl HasReferenceDate for TenorBasedZeroRateTermStructure {
    fn reference_date(&self) -> Date {
        return self.reference_date;
    }
}

impl YieldProvider for TenorBasedZeroRateTermStructure {
    fn discount_factor(&self, date: Date) -> Result<f64, YieldProviderError> {
        let year_fraction = self.day_counter.year_fraction(self.reference_date(), date);

        let spread = self.interpolation.interpolate(
            year_fraction,
            &self.year_fractions,
            &self.spreads,
            self.enable_extrapolation,
        );
        let rate = InterestRate::new(spread, self.compounding, self.frequency, self.day_counter);
        Ok(1.0 / rate.compound_factor(self.reference_date, date))
    }

    fn forward_rate(
        &self,
        start_date: Date,
        end_date: Date,
        comp: Compounding,
        freq: Frequency,
    ) -> Result<f64, YieldProviderError> {
        let start_df = self.discount_factor(start_date)?;
        let end_df = self.discount_factor(end_date)?;

        let compound = start_df / end_df;
        let t = self
            .day_counter
            .year_fraction(self.reference_date, end_date);
        let rate = InterestRate::implied_rate(compound, self.day_counter, comp, freq, t)?;
        Ok(rate.rate())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::interpolation::enums::Interpolator,
        rates::{
            enums::Compounding, traits::YieldProvider,
            yieldtermstructure::tenorbasedzeroratetermstructure::TenorBasedZeroRateTermStructure,
        },
        time::{
            date::Date,
            daycounter::DayCounter,
            enums::{Frequency, TimeUnit},
            period::Period,
        },
    };

    #[test]
    fn test_zero_rate() {
        let reference_date = Date::new(2021, 12, 1);
        let day_counter = DayCounter::Thirty360;
        let compounding = Compounding::Compounded;
        let frequency = Frequency::Semiannual;

        let interpolation = Interpolator::Linear;
        let enable_extrapolation = true;

        let years = vec![1, 2, 3, 4, 5];
        let spreads = vec![0.01, 0.02, 0.03, 0.04, 0.05];
        let tenors = years
            .iter()
            .map(|x| Period::new(*x, TimeUnit::Years))
            .collect();
        let zero_rate_term_structure = TenorBasedZeroRateTermStructure::new(
            reference_date,
            tenors,
            spreads,
            day_counter,
            compounding,
            frequency,
            interpolation,
            enable_extrapolation,
        );

        let df = zero_rate_term_structure
            .discount_factor(Date::new(2022, 6, 1))
            .unwrap();
        println!("df: {}", df);

        let forward_rate = zero_rate_term_structure
            .forward_rate(
                reference_date,
                reference_date + Period::new(1, TimeUnit::Years),
                Compounding::Compounded,
                Frequency::Annual,
            )
            .unwrap();
        assert!(forward_rate - 0.01 < 1e-10);
    }
}
