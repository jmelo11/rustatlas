use crate::{
    cashflows::cashflow::Side, currencies::enums::Currency, rates::interestrate::RateDefinition,
};

use super::instrument::RateType;

pub struct MakeSwap {
    first_leg_rate_type: Option<RateType>,
    first_leg_rate_value: Option<f64>,
    first_leg_rate_definition: Option<RateDefinition>,
    first_leg_currency: Option<Currency>,
    first_leg_side: Option<Side>,
    first_leg_structure: Option<String>,
    first_leg_discount_curve_id: Option<usize>,
    first_leg_forecast_curve_id: Option<usize>,

    second_leg_rate_type: Option<RateType>,
    second_leg_rate_value: Option<f64>,
    second_leg_rate_definition: Option<RateDefinition>,
    second_leg_currency: Option<Currency>,
    second_leg_side: Option<Side>,
    second_leg_structure: Option<String>,
    second_leg_discount_curve_id: Option<usize>,
    second_leg_forecast_curve_id: Option<usize>,

    id: Option<String>,
}

impl MakeSwap {
    pub fn new() -> Self {
        MakeSwap {
            first_leg_rate_type: None,
            first_leg_rate_value: None,
            first_leg_rate_definition: None,
            first_leg_currency: None,
            first_leg_side: None,
            first_leg_structure: None,
            first_leg_discount_curve_id: None,
            first_leg_forecast_curve_id: None,

            second_leg_rate_type: None,
            second_leg_rate_value: None,
            second_leg_rate_definition: None,
            second_leg_currency: None,
            second_leg_side: None,
            second_leg_structure: None,
            second_leg_discount_curve_id: None,
            second_leg_forecast_curve_id: None,

            id: None,
        }
    }

    pub fn with_first_leg_rate_type(mut self, rate_type: RateType) -> Self {
        self.first_leg_rate_type = Some(rate_type);
        self
    }

    pub fn with_first_leg_rate_value(mut self, rate_value: f64) -> Self {
        self.first_leg_rate_value = Some(rate_value);
        self
    }

    pub fn with_first_leg_rate_definition(mut self, rate_definition: RateDefinition) -> Self {
        self.first_leg_rate_definition = Some(rate_definition);
        self
    }

    pub fn with_first_leg_currency(mut self, currency: Currency) -> Self {
        self.first_leg_currency = Some(currency);
        self
    }

    pub fn with_first_leg_side(mut self, side: Side) -> Self {
        self.first_leg_side = Some(side);
        self
    }

    pub fn with_first_leg_discount_curve_id(mut self, curve_id: usize) -> Self {
        self.first_leg_discount_curve_id = Some(curve_id);
        self
    }

    pub fn with_first_leg_forecast_curve_id(mut self, curve_id: usize) -> Self {
        self.first_leg_forecast_curve_id = Some(curve_id);
        self
    }

    pub fn with_second_leg_rate_type(mut self, rate_type: RateType) -> Self {
        self.second_leg_rate_type = Some(rate_type);
        self
    }

    pub fn with_second_leg_rate_value(mut self, rate_value: f64) -> Self {
        self.second_leg_rate_value = Some(rate_value);
        self
    }

    pub fn with_second_leg_rate_definition(mut self, rate_definition: RateDefinition) -> Self {
        self.second_leg_rate_definition = Some(rate_definition);
        self
    }

    pub fn with_second_leg_currency(mut self, currency: Currency) -> Self {
        self.second_leg_currency = Some(currency);
        self
    }

    pub fn with_second_leg_side(mut self, side: Side) -> Self {
        self.second_leg_side = Some(side);
        self
    }

    pub fn with_second_leg_discount_curve_id(mut self, curve_id: usize) -> Self {
        self.second_leg_discount_curve_id = Some(curve_id);
        self
    }

    pub fn with_second_leg_forecast_curve_id(mut self, curve_id: usize) -> Self {
        self.second_leg_forecast_curve_id = Some(curve_id);
        self
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_first_leg_structure(mut self, structure: String) -> Self {
        self.first_leg_structure = Some(structure);
        self
    }

    pub fn with_second_leg_structure(mut self, structure: String) -> Self {
        self.second_leg_structure = Some(structure);
        self
    }
}
