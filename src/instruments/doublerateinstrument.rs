use serde::{Deserialize, Serialize};

use super::instrument::RateType;
use crate::{
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::{InterestAccrual, Payable},
    },
    core::traits::HasCurrency,
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
    time::{date::Date, enums::Frequency},
    utils::errors::Result,
    visitors::traits::HasCashflows,
};
/// A financial instrument with two different interest rates applied in different periods.
///
/// This struct represents an instrument where the interest rate changes on a specified date.
/// Before the change date, the first rate applies; after the change date, the second rate applies.
// #[deprecated(note = "DoubleRateInstrument is deprecated and will be removed in future versions. A new implementation (SteppedCouponInstrument) should be created.")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DoubleRateInstrument {
    start_date: Date,
    end_date: Date,
    notional: f64,
    notional_at_change_rate: Option<f64>,
    payment_frequency: Frequency,
    rate_type: RateType,
    side: Side,
    currency: Currency,
    id: Option<String>,
    issue_date: Option<Date>,
    change_rate_date: Date,
    first_rate_definition: Option<RateDefinition>,
    first_rate: Option<f64>,
    second_rate_definition: Option<RateDefinition>,
    second_rate: Option<f64>,
    forecast_curve_id: Option<usize>,
    discount_curve_id: Option<usize>,
    cashflows: Vec<Cashflow>,
}

impl DoubleRateInstrument {
    /// Creates a new `DoubleRateInstrument` with the specified parameters.
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    // allowed: high-arity API; refactor deferred
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start_date: Date,
        end_date: Date,
        notional: f64,
        notional_at_change_rate: Option<f64>,
        payment_frequency: Frequency,
        side: Side,
        currency: Currency,
        id: Option<String>,
        issue_date: Option<Date>,
        change_rate_date: Date,
        rate_type: RateType,
        first_rate_definition: Option<RateDefinition>,
        first_rate: Option<f64>,
        second_rate_definition: Option<RateDefinition>,
        second_rate: Option<f64>,
        forecast_curve_id: Option<usize>,
        discount_curve_id: Option<usize>,
        cashflows: Vec<Cashflow>,
    ) -> Self {
        Self {
            start_date,
            end_date,
            notional,
            notional_at_change_rate,
            payment_frequency,
            rate_type,
            side,
            currency,
            id,
            issue_date,
            change_rate_date,
            first_rate_definition,
            first_rate,
            second_rate_definition,
            second_rate,
            forecast_curve_id,
            discount_curve_id,
            cashflows,
        }
    }

    /// Returns the notional amount of the instrument.
    #[must_use]
    pub const fn notional(&self) -> f64 {
        self.notional
    }

    /// Returns the notional amount at the rate change date, if specified.
    #[must_use]
    pub const fn notional_at_change_rate(&self) -> Option<f64> {
        self.notional_at_change_rate
    }

    /// Returns the payment frequency of the instrument.
    #[must_use]
    pub const fn payment_frequency(&self) -> Frequency {
        self.payment_frequency
    }

    /// Returns the side (payer or receiver) of the instrument.
    #[must_use]
    pub const fn side(&self) -> Side {
        self.side
    }

    /// Returns the identifier of the instrument, if specified.
    #[must_use]
    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    /// Returns the forecast curve ID, if specified.
    #[must_use]
    pub const fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }

    /// Returns the discount curve ID, if specified.
    #[must_use]
    pub const fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    /// Returns the start date of the instrument.
    #[must_use]
    pub const fn start_date(&self) -> Date {
        self.start_date
    }

    /// Returns the end date of the instrument.
    #[must_use]
    pub const fn end_date(&self) -> Date {
        self.end_date
    }

    /// Returns the issue date of the instrument, if specified.
    #[must_use]
    pub const fn issue_date(&self) -> Option<Date> {
        self.issue_date
    }

    /// Returns the date when the interest rate changes.
    #[must_use]
    pub const fn change_rate_date(&self) -> Date {
        self.change_rate_date
    }

    /// Returns the type of interest rate applied.
    #[must_use]
    pub const fn rate_type(&self) -> RateType {
        self.rate_type
    }

    /// Returns the rate definition for the first period.
    #[must_use]
    pub const fn first_rate_definition(&self) -> Option<RateDefinition> {
        self.first_rate_definition
    }

    /// Returns the rate value for the first period, if specified.
    #[must_use]
    pub const fn first_rate(&self) -> Option<f64> {
        self.first_rate
    }

    /// Returns the rate definition for the second period.
    #[must_use]
    pub const fn second_rate_definition(&self) -> Option<RateDefinition> {
        self.second_rate_definition
    }

    /// Returns the rate value for the second period, if specified.
    #[must_use]
    pub const fn second_rate(&self) -> Option<f64> {
        self.second_rate
    }

    /// Sets the discount curve ID and returns self for method chaining.
    #[must_use]
    pub const fn set_discount_curve_id(mut self, discount_curve_id: usize) -> Self {
        self.discount_curve_id = Some(discount_curve_id);
        self
    }

    /// Sets the forecast curve ID and returns self for method chaining.
    #[must_use]
    pub const fn set_forecast_curve_id(mut self, forecast_curve_id: usize) -> Self {
        self.forecast_curve_id = Some(forecast_curve_id);
        self
    }

    /// Sets the first rate for all cashflows before the rate change date.
    #[must_use]
    pub fn set_first_rate(mut self, rate: f64) -> Self {
        let change_rate_date = self.change_rate_date();
        self.mut_cashflows().iter_mut().for_each(|cf| {
            if cf.payment_date() <= change_rate_date {
                match cf {
                    Cashflow::FloatingRateCoupon(coupon) => {
                        coupon.set_spread(rate);
                    }
                    Cashflow::FixedRateCoupon(coupon) => {
                        coupon.set_rate_value(rate);
                    }
                    _ => {}
                }
            }
        });
        self
    }

    /// Sets the second rate for all cashflows after the rate change date.
    #[must_use]
    pub fn set_second_rate(mut self, rate: f64) -> Self {
        let change_rate_date = self.change_rate_date();
        self.mut_cashflows().iter_mut().for_each(|cf| {
            if cf.payment_date() > change_rate_date {
                match cf {
                    Cashflow::FloatingRateCoupon(coupon) => {
                        coupon.set_spread(rate);
                    }
                    Cashflow::FixedRateCoupon(coupon) => {
                        coupon.set_rate_value(rate);
                    }
                    _ => {}
                }
            }
        });
        self
    }

    /// Sets both the first and second rates if provided.
    #[must_use]
    pub fn set_rates(mut self, first_rate: Option<f64>, second_rate: Option<f64>) -> Self {
        if let Some(rate) = first_rate {
            self = self.set_first_rate(rate);
        }
        if let Some(rate) = second_rate {
            self = self.set_second_rate(rate);
        }
        self
    }
}

impl HasCurrency for DoubleRateInstrument {
    fn currency(&self) -> Result<Currency> {
        Ok(self.currency)
    }
}

impl InterestAccrual for DoubleRateInstrument {
    fn accrual_start_date(&self) -> Result<Date> {
        Ok(self.start_date)
    }

    fn accrual_end_date(&self) -> Result<Date> {
        Ok(self.end_date)
    }

    fn accrued_amount(&self, start_date: Date, end_date: Date) -> Result<f64> {
        let total_accrued_amount = self.cashflows.iter().fold(0.0, |acc, cf| {
            acc + cf.accrued_amount(start_date, end_date).unwrap_or(0.0)
        });
        Ok(total_accrued_amount)
    }
}

impl HasCashflows for DoubleRateInstrument {
    fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    fn mut_cashflows(&mut self) -> &mut [Cashflow] {
        &mut self.cashflows
    }
}
