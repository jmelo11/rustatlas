use thiserror::Error;

use crate::{
    math::interpolation::traits::Interpolate,
    rates::{
        traits::{HasReferenceDate, YieldProvider, YieldProviderError},
        yieldtermstructure::{
            discounttermstructure::DiscountTermStructure,
            errortermstructure::TermStructureConstructorError,
            flatforwardtermstructure::FlatForwardTermStructure,
            zeroratetermstructure::ZeroRateTermStructure,
        },
    },
    time::{date::Date, enums::TimeUnit, period::Period},
};

/// # AdvanceInTime
/// Trait for advancing in time a given object. Returns a represation of the object
/// as it would be after the given period.
pub trait AdvanceInTime {
    type Output;

    fn advance_to_period(&self, period: Period) -> Result<Self::Output, AdvanceInTimeError>;
    fn advance_to_date(&self, date: Date) -> Result<Self::Output, AdvanceInTimeError>;
}

#[derive(Error, Debug)]
pub enum AdvanceInTimeError {
    #[error("Invalid date")]
    InvalidDate,
    #[error("YieldProviderError: {0}")]
    YieldProviderError(#[from] YieldProviderError),
    #[error("TermStructureConstructorError: {0}")]
    TermStructureConstructorError(#[from] TermStructureConstructorError),
}

/// # AdvanceInTime for FlatForwardTermStructure
impl AdvanceInTime for FlatForwardTermStructure {
    type Output = FlatForwardTermStructure;
    fn advance_to_period(&self, period: Period) -> Result<Self::Output, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());
        return Ok(FlatForwardTermStructure::new(
            new_reference_date,
            self.rate(),
        ));
    }

    fn advance_to_date(&self, date: Date) -> Result<Self::Output, AdvanceInTimeError> {
        return Ok(FlatForwardTermStructure::new(date, self.rate()));
    }
}

/// # AdvanceInTime for DiscountTermStructure
impl<T: Interpolate> AdvanceInTime for DiscountTermStructure<T> {
    type Output = DiscountTermStructure<T>;
    fn advance_to_period(&self, period: Period) -> Result<Self::Output, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());

        let new_dates: Vec<Date> = self
            .dates()
            .iter()
            .map(|x| x.advance(period.length(), period.units()))
            .collect();

        let start_df = self.discount_factor(new_dates[0])?;
        let shifted_dfs: Result<Vec<f64>, AdvanceInTimeError> = new_dates
            .iter()
            .map(|x| {
                let df = self.discount_factor(*x)?;
                Ok(df / start_df)
            })
            .collect();

        Ok(DiscountTermStructure::new(
            new_reference_date,
            new_dates,
            shifted_dfs?,
            self.day_counter(),
        )?)
    }

    fn advance_to_date(&self, date: Date) -> Result<Self::Output, AdvanceInTimeError> {
        let days = (self.reference_date() - date) as i32;
        if days < 0 {
            return Err(AdvanceInTimeError::InvalidDate);
        }
        let period = Period::new(days, TimeUnit::Days);
        return self.advance_to_period(period);
    }
}

/// # AdvanceInTime for ZeroRateTermStructure
impl<T: Interpolate> AdvanceInTime for ZeroRateTermStructure<T> {
    type Output = ZeroRateTermStructure<T>;
    fn advance_to_period(&self, period: Period) -> Result<Self::Output, AdvanceInTimeError> {
        let new_reference_date = self
            .reference_date()
            .advance(period.length(), period.units());

        let new_dates: Vec<Date> = self
            .dates()
            .iter()
            .map(|x| x.advance(period.length(), period.units()))
            .collect();

        let start_df = self.discount_factor(new_dates[0])?;
        let shifted_dfs: Result<Vec<f64>, AdvanceInTimeError> = new_dates
            .iter()
            .map(|x| {
                let df = self.discount_factor(*x)?;
                Ok(df / start_df)
            })
            .collect();

        Ok(ZeroRateTermStructure::new(
            new_reference_date,
            new_dates,
            shifted_dfs?,
            self.rate_definition(),
        )?)
    }

    fn advance_to_date(&self, date: Date) -> Result<Self::Output, AdvanceInTimeError> {
        let days = (self.reference_date() - date) as i32;
        if days < 0 {
            return Err(AdvanceInTimeError::InvalidDate);
        }
        let period = Period::new(days, TimeUnit::Days);
        return self.advance_to_period(period);
    }
}


