//! Python view of [`quantsupport::prelude::EvaluationResults`].
//!
//! Tabular results (sensitivities, cashflows) are returned as pandas
//! DataFrames.

use pyo3::prelude::*;
use quantsupport::prelude::EvaluationResults as QsEvaluationResults;

use crate::conv::dataframe;

/// A single cashflow row extracted from the Rust `CashflowsTable`.
#[derive(Clone)]
struct CashflowRow {
    payment_date: String,
    cashflow_type: String,
    amount: f64,
    fixing: Option<f64>,
    accrual_period: f64,
    currency: String,
    caplet_strike: Option<f64>,
    floorlet_strike: Option<f64>,
    leg_index: usize,
}

/// Results of evaluating a trade.
#[pyclass(name = "EvaluationResults")]
pub struct EvaluationResults {
    price: Option<f64>,
    fair_rate: Option<f64>,
    sensitivities: Option<Vec<(String, f64)>>,
    cashflows: Option<Vec<CashflowRow>>,
}

impl EvaluationResults {
    pub fn from_qs(res: &QsEvaluationResults) -> Self {
        let sensitivities = res.sensitivities().map(|s| {
            s.instrument_keys()
                .iter()
                .cloned()
                .zip(s.exposure().iter().copied())
                .collect()
        });
        let cashflows = res.cashflows().map(|t| {
            (0..t.payment_dates().len())
                .map(|i| CashflowRow {
                    payment_date: t.payment_dates()[i].to_string(),
                    cashflow_type: t.cashflow_types()[i].clone(),
                    amount: t.amounts()[i],
                    fixing: t.fixing()[i],
                    accrual_period: t.accrual_periods()[i],
                    currency: t.currencies()[i].to_string(),
                    caplet_strike: t.caplet_strikes()[i],
                    floorlet_strike: t.floorlet_strikes()[i],
                    leg_index: t.leg_indices()[i],
                })
                .collect()
        });
        Self {
            price: res.price(),
            fair_rate: res.fair_rate(),
            sensitivities,
            cashflows,
        }
    }
}

#[pymethods]
impl EvaluationResults {
    /// Net present value, if `"Value"` was requested.
    #[getter]
    fn price(&self) -> Option<f64> {
        self.price
    }

    /// Fair (par) rate, if `"FairRate"` was requested.
    #[getter]
    fn fair_rate(&self) -> Option<f64> {
        self.fair_rate
    }

    /// Sensitivities as a DataFrame with columns pillar/value (duplicated
    /// pillars summed, sorted by pillar), if `"Sensitivities"` was requested.
    #[getter]
    fn sensitivities<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let Some(pairs) = &self.sensitivities else {
            return Ok(None);
        };
        let mut map = std::collections::BTreeMap::new();
        for (k, v) in pairs {
            *map.entry(k.clone()).or_insert(0.0) += v;
        }
        let (pillars, values): (Vec<String>, Vec<f64>) = map.into_iter().unzip();
        Ok(Some(dataframe(
            py,
            &[
                ("pillar", pillars.into_pyobject(py)?.into_any()),
                ("value", values.into_pyobject(py)?.into_any()),
            ],
        )?))
    }

    /// Raw sensitivities as an ordered list of `(pillar_label, exposure)` tuples.
    #[getter]
    fn raw_sensitivities(&self) -> Option<Vec<(String, f64)>> {
        self.sensitivities.clone()
    }

    /// Cashflow table as a DataFrame, if `"Cashflows"` was requested.
    #[getter]
    fn cashflows<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let Some(rows) = &self.cashflows else {
            return Ok(None);
        };
        let n = rows.len();
        let mut payment_date = Vec::with_capacity(n);
        let mut cashflow_type = Vec::with_capacity(n);
        let mut amount = Vec::with_capacity(n);
        let mut fixing = Vec::with_capacity(n);
        let mut accrual_period = Vec::with_capacity(n);
        let mut currency = Vec::with_capacity(n);
        let mut caplet_strike = Vec::with_capacity(n);
        let mut floorlet_strike = Vec::with_capacity(n);
        let mut leg_index = Vec::with_capacity(n);
        for r in rows {
            payment_date.push(r.payment_date.clone());
            cashflow_type.push(r.cashflow_type.clone());
            amount.push(r.amount);
            fixing.push(r.fixing);
            accrual_period.push(r.accrual_period);
            currency.push(r.currency.clone());
            caplet_strike.push(r.caplet_strike);
            floorlet_strike.push(r.floorlet_strike);
            leg_index.push(r.leg_index);
        }
        Ok(Some(dataframe(
            py,
            &[
                ("payment_date", payment_date.into_pyobject(py)?.into_any()),
                ("type", cashflow_type.into_pyobject(py)?.into_any()),
                ("amount", amount.into_pyobject(py)?.into_any()),
                ("fixing", fixing.into_pyobject(py)?.into_any()),
                (
                    "accrual_period",
                    accrual_period.into_pyobject(py)?.into_any(),
                ),
                ("currency", currency.into_pyobject(py)?.into_any()),
                ("caplet_strike", caplet_strike.into_pyobject(py)?.into_any()),
                (
                    "floorlet_strike",
                    floorlet_strike.into_pyobject(py)?.into_any(),
                ),
                ("leg_index", leg_index.into_pyobject(py)?.into_any()),
            ],
        )?))
    }

    fn __repr__(&self) -> String {
        format!(
            "EvaluationResults(price={:?}, fair_rate={:?}, sensitivities={}, cashflows={})",
            self.price,
            self.fair_rate,
            self.sensitivities.is_some(),
            self.cashflows.is_some(),
        )
    }
}
