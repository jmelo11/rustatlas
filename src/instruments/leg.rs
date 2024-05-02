use crate::{
    cashflows::cashflow::{Cashflow, Side},
    currencies::enums::Currency,
    rates::interestrate::RateDefinition,
};

use super::{instrument::RateType, traits::Structure};

/// # Leg
/// A financial leg. Contains a stream of cashflows. Instruments have one or more legs.
#[derive(Debug, Clone)]
pub struct Leg {
    structure: Structure,
    rate_type: RateType,
    rate_value: f64,
    rate_definition: RateDefinition,
    currency: Currency,
    side: Side,
    discount_curve_id: Option<usize>,
    forecast_curve_id: Option<usize>,
    cashflows: Vec<Cashflow>,
}

impl Leg {
    pub fn new(
        structure: Structure,
        rate_type: RateType,
        rate_value: f64,
        rate_definition: RateDefinition,
        currency: Currency,
        side: Side,
        discount_curve_id: Option<usize>,
        forecast_curve_id: Option<usize>,
        cashflows: Vec<Cashflow>,
    ) -> Self {
        Leg {
            structure,
            rate_type,
            rate_value,
            rate_definition,
            currency,
            side,
            discount_curve_id,
            forecast_curve_id,
            cashflows,
        }
    }

    pub fn cashflows(&self) -> &[Cashflow] {
        &self.cashflows
    }

    pub fn structure(&self) -> Structure {
        self.structure
    }

    pub fn rate_type(&self) -> RateType {
        self.rate_type
    }

    pub fn rate_value(&self) -> f64 {
        self.rate_value
    }

    pub fn rate_definition(&self) -> RateDefinition {
        self.rate_definition
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn discount_curve_id(&self) -> Option<usize> {
        self.discount_curve_id
    }

    pub fn forecast_curve_id(&self) -> Option<usize> {
        self.forecast_curve_id
    }
}
