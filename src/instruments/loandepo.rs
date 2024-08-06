use serde::{Deserialize, Serialize};

use crate::{
    alm::enums::{AccountType, EvaluationMode},
    cashflows::{
        cashflow::{Cashflow, Side},
        traits::InterestAccrual,
    },
    currencies::enums::Currency,
    rates::{
        enums::Compounding,
        interestrate::{InterestRate, RateDefinition},
    },
    time::{date::Date, daycounter::DayCounter, enums::Frequency},
    utils::errors::{AtlasError, Result},
};

use super::{
    fixedrateinstrument::FixedRateInstrument,
    floatingrateinstrument::FloatingRateInstrument,
    instrument::{Instrument, RateType},
    traits::Structure,
};

/// # LoanDepo
/// Struct that represents a serialized loan or deposit. Used for serialization purposes.
#[derive(Serialize, Deserialize, Clone)]
pub struct LoanDepo {
    pub id: Option<usize>,
    pub mis_id: String,
    pub reference_date: Date,
    pub loandepo_configuration_id: usize,
    pub notional: f64,
    pub issue_date: Option<Date>,
    pub start_date: Date,
    pub end_date: Date,
    pub credit_status: String,
    pub structure: Structure,
    pub side: Side,
    pub account_type: AccountType,
    pub segment: String,
    pub area: String,
    pub product_family: String,
    pub payment_frequency: Frequency,

    pub first_ftp_rate: f64,
    pub first_client_rate: f64,
    pub second_ftp_rate: Option<f64>,
    pub second_client_rate: Option<f64>,

    // pre-calculated fields
    pub notional_local_ccy: Option<f64>,
    pub outstanding: Option<f64>,
    pub outstanding_local_ccy: Option<f64>,
    pub readjustment_local_ccy: Option<f64>,
    pub avg_outstanding: Option<f64>,
    pub avg_outstanding_local_ccy: Option<f64>,
    pub avg_readjustment: Option<f64>,
    pub avg_interest: Option<f64>,
    pub avg_interest_local_ccy: Option<f64>,
    pub ftp_interest: Option<f64>,
    pub ftp_interest_local_ccy: Option<f64>,
    pub earned_interest: Option<f64>,
    pub earned_interest_local_ccy: Option<f64>,
    pub margin: Option<f64>,
    pub margin_local_ccy: Option<f64>,

    pub interest: Option<f64>, 
    pub interest_local_ccy: Option<f64>, 

    pub rate_type: RateType,
    pub first_rate_frequency: Frequency,
    pub first_rate_day_counter: DayCounter,
    pub first_rate_compounding: Compounding,

    pub months_to_first_coupon: Option<i16>,
    pub second_rate_frequency: Option<Frequency>,
    pub second_rate_day_counter: Option<DayCounter>,
    pub second_rate_compounding: Option<Compounding>,

    pub currency: Currency,
    pub discount_curve_id: usize,
    pub forecast_curve_id: Option<usize>,

    pub cashflows: Vec<Cashflow>,
    pub evaluation_mode: Option<EvaluationMode>,
    pub rate_change_date: Option<Date>,
    pub cashflows_source: String,

    pub last_reprice_date: Option<Date>,
    pub next_reprice_date: Option<Date>,
}

impl TryFrom<LoanDepo> for Instrument {
    type Error = AtlasError;

    fn try_from(value: LoanDepo) -> Result<Instrument> {
        let mut cashflows = value.cashflows.clone();
        cashflows.iter_mut().try_for_each(|cf| -> Result<()> {
            match cf {
                Cashflow::FixedRateCoupon(coupon) => {
                    if let Some(date) = value.rate_change_date {
                        let rate = if coupon.accrual_start_date()? < date {
                            match value.evaluation_mode {
                                Some(EvaluationMode::FTPRate) => InterestRate::new(
                                    value.first_ftp_rate,
                                    value.first_rate_compounding,
                                    value.first_rate_frequency,
                                    value.first_rate_day_counter,
                                ),
                                Some(EvaluationMode::ClientRate) => InterestRate::new(
                                    value.first_client_rate,
                                    value.first_rate_compounding,
                                    value.first_rate_frequency,
                                    value.first_rate_day_counter,
                                ),
                                None => {
                                    return Err(AtlasError::ValueNotSetErr(
                                        "Evaluation Mode".to_string(),
                                    ))
                                }
                            }
                        } else {
                            match value.evaluation_mode {
                                Some(EvaluationMode::FTPRate) => InterestRate::new(
                                    value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                        "Second FTP rate".to_string(),
                                    ))?,
                                    value.second_rate_compounding.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate compounding".to_string(),
                                        ),
                                    )?,
                                    value.second_rate_frequency.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate frequency".to_string(),
                                        ),
                                    )?,
                                    value.second_rate_day_counter.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate day counter".to_string(),
                                        ),
                                    )?,
                                ),
                                Some(EvaluationMode::ClientRate) => InterestRate::new(
                                    value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                        "Second client rate".to_string(),
                                    ))?,
                                    value.second_rate_compounding.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate compounding".to_string(),
                                        ),
                                    )?,
                                    value.second_rate_frequency.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate frequency".to_string(),
                                        ),
                                    )?,
                                    value.second_rate_day_counter.ok_or(
                                        AtlasError::ValueNotSetErr(
                                            "Second rate day counter".to_string(),
                                        ),
                                    )?,
                                ),
                                None => {
                                    return Err(AtlasError::ValueNotSetErr(
                                        "Evaluation Mode".to_string(),
                                    ))
                                }
                            }
                        };
                        coupon.set_rate(rate);
                    } else {
                        let rate = match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => InterestRate::new(
                                value.first_ftp_rate,
                                value.first_rate_compounding,
                                value.first_rate_frequency,
                                value.first_rate_day_counter,
                            ),
                            Some(EvaluationMode::ClientRate) => InterestRate::new(
                                value.first_client_rate,
                                value.first_rate_compounding,
                                value.first_rate_frequency,
                                value.first_rate_day_counter,
                            ),
                            None => {
                                return Err(AtlasError::ValueNotSetErr(
                                    "Evaluation Mode".to_string(),
                                ))
                            }
                        };
                        coupon.set_rate(rate);
                    }
                    Ok(())
                }
                Cashflow::FloatingRateCoupon(coupon) => {
                    if let Some(date) = value.rate_change_date {
                        let rate = if coupon.accrual_start_date()? < date {
                            match value.evaluation_mode {
                                Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                                Some(EvaluationMode::ClientRate) => value.first_client_rate,
                                None => {
                                    return Err(AtlasError::ValueNotSetErr(
                                        "Evaluation Mode".to_string(),
                                    ))
                                }
                            }
                        } else {
                            match value.evaluation_mode {
                                Some(EvaluationMode::FTPRate) => value.second_ftp_rate.ok_or(
                                    AtlasError::ValueNotSetErr("Second FTP rate".to_string()),
                                )?,
                                Some(EvaluationMode::ClientRate) => {
                                    value.second_client_rate.ok_or(AtlasError::ValueNotSetErr(
                                        "Second client rate".to_string(),
                                    ))?
                                }
                                None => {
                                    return Err(AtlasError::ValueNotSetErr(
                                        "Evaluation Mode".to_string(),
                                    ))
                                }
                            }
                        };
                        coupon.set_spread(rate);
                    } else {
                        let rate = match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                            Some(EvaluationMode::ClientRate) => value.first_client_rate,
                            None => {
                                return Err(AtlasError::ValueNotSetErr(
                                    "Evaluation Mode".to_string(),
                                ))
                            }
                        };
                        coupon.set_spread(rate);
                    }
                    Ok(())
                }
                _ => Ok(()),
            }
        })?;

        match value.rate_type {
            RateType::Fixed => {
                let rate = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => InterestRate::new(
                        value.first_ftp_rate,
                        value.first_rate_compounding,
                        value.first_rate_frequency,
                        value.first_rate_day_counter,
                    ),
                    Some(EvaluationMode::ClientRate) => InterestRate::new(
                        value.first_client_rate,
                        value.first_rate_compounding,
                        value.first_rate_frequency,
                        value.first_rate_day_counter,
                    ),
                    None => return Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                };
                let instrument = FixedRateInstrument::new(
                    value.start_date,
                    value.end_date,
                    value.notional,
                    rate,
                    value.payment_frequency,
                    cashflows,
                    value.structure,
                    value.side,
                    value.currency,
                    Some(value.discount_curve_id),
                    Some(value.mis_id),
                    value.issue_date,
                    None,
                );

                Ok(Instrument::FixedRateInstrument(instrument))
            }
            RateType::Floating => {
                let (rate, rate_definition) = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => (
                        value.first_ftp_rate,
                        RateDefinition::new(
                            value.first_rate_day_counter,
                            value.first_rate_compounding,
                            value.first_rate_frequency,
                        ),
                    ),
                    Some(EvaluationMode::ClientRate) => (
                        value.first_client_rate,
                        RateDefinition::new(
                            value.first_rate_day_counter,
                            value.first_rate_compounding,
                            value.first_rate_frequency,
                        ),
                    ),
                    None => return Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                };
                let instrument = FloatingRateInstrument::new(
                    value.start_date,
                    value.end_date,
                    value.notional,
                    rate,
                    value.side,
                    cashflows,
                    value.payment_frequency,
                    rate_definition,
                    value.structure,
                    value.currency,
                    Some(value.discount_curve_id),
                    value.forecast_curve_id,
                    Some(value.mis_id),
                    value.issue_date,
                );
                Ok(Instrument::FloatingRateInstrument(instrument))
            }
            _ => Err(AtlasError::NotImplementedErr("RateType".to_string())),
        }
    }
}
