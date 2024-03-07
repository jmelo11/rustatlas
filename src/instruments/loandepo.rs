use serde::{Deserialize, Serialize};

use crate::{
    alm::enums::{AccountType, EvaluationMode, ProductFamily, Segment},
    cashflows::cashflow::{Cashflow, Side},
    currencies::enums::Currency,
    prelude::InterestAccrual,
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

/// # LoanDepoCashflow
/// Struct that represents a serialized cashflow. Used for serialization purposes.
// #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
// pub struct LoanDepoCashflow {
//     cashflow_type: CashflowType,
//     payment_date: Date,
//     notional: Option<f64>,
//     amount: Option<f64>,
//     accrual_start_date: Option<Date>,
//     accrual_end_date: Option<Date>,
// }

// impl LoanDepoCashflow {
//     pub fn new(
//         cashflow_type: CashflowType,
//         payment_date: Date,
//         notional: Option<f64>,
//         amount: Option<f64>,
//         accrual_start_date: Option<Date>,
//         accrual_end_date: Option<Date>,
//     ) -> Self {
//         LoanDepoCashflow {
//             cashflow_type,
//             payment_date,
//             notional,
//             amount,
//             accrual_start_date,
//             accrual_end_date,
//         }
//     }

//     pub fn cashflow_type(&self) -> CashflowType {
//         self.cashflow_type
//     }

//     pub fn payment_date(&self) -> Date {
//         self.payment_date
//     }

//     pub fn notional(&self) -> Option<f64> {
//         self.notional
//     }

//     pub fn amount(&self) -> Option<f64> {
//         self.amount
//     }

//     pub fn accrual_start_date(&self) -> Option<Date> {
//         self.accrual_start_date
//     }

//     pub fn accrual_end_date(&self) -> Option<Date> {
//         self.accrual_end_date
//     }
// }

// impl From<Cashflow> for LoanDepoCashflow {
//     /// Converts a Cashflow into a LoanDepoCashflow
//     fn from(cashflow: Cashflow) -> Self {
//         match cashflow {
//             Cashflow::Disbursement(cf) => LoanDepoCashflow::new(
//                 CashflowType::Disbursement,
//                 cf.payment_date(),
//                 None,
//                 Some(cf.amount().unwrap()),
//                 None,
//                 None,
//             ),
//             Cashflow::Redemption(cf) => LoanDepoCashflow::new(
//                 CashflowType::Redemption,
//                 cf.payment_date(),
//                 None,
//                 Some(cf.amount().unwrap()),
//                 None,
//                 None,
//             ),
//             Cashflow::FixedRateCoupon(cf) => LoanDepoCashflow::new(
//                 CashflowType::FixedRateCoupon,
//                 cf.payment_date(),
//                 Some(cf.notional()),
//                 Some(cf.amount().unwrap()),
//                 Some(cf.accrual_start_date()),
//                 Some(cf.accrual_end_date()),
//             ),
//             Cashflow::FloatingRateCoupon(cf) => LoanDepoCashflow::new(
//                 CashflowType::FloatingRateCoupon,
//                 cf.payment_date(),
//                 Some(cf.notional()),
//                 None,
//                 Some(cf.accrual_start_date()),
//                 Some(cf.accrual_end_date()),
//             ),
//         }
//     }
// }

/// # LoanDepo
/// Struct that represents a serialized loan or deposit. Used for serialization purposes.
#[derive(Serialize, Deserialize, Clone)]
pub struct LoanDepo {
    pub id: usize,
    pub mis_id: String,

    pub process_date: Date,
    pub loandepo_configuration_id: usize,
    pub notional: f64,
    pub issue_date: Date,
    pub start_date: Date,
    pub end_date: Date,
    pub credit_status: String,
    pub structure: Structure,
    pub side: Side,
    pub segment: Segment,
    pub account_type: AccountType,
    pub product_family: ProductFamily,
    pub payment_frequency: Frequency,

    pub first_ftp_rate: f64,
    pub first_client_rate: f64,
    pub second_ftp_rate: Option<f64>,
    pub second_client_rate: Option<f64>,

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
    pub cashflows_source: Option<String>,
}

impl TryFrom<LoanDepo> for Instrument {
    type Error = AtlasError;

    fn try_from(value: LoanDepo) -> Result<Instrument> {
        let mut cashflows = value.cashflows.clone();
        cashflows.iter_mut().try_for_each(|cf| -> Result<()> {
            match cf {
                Cashflow::FixedRateCoupon(coupon) => {
                    if let Some(date) = value.rate_change_date {
                        let rate = if coupon.accrual_start_date() < date {
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
                        let rate = if coupon.accrual_start_date() < date {
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
                    Some(value.id),
                    Some(value.issue_date),
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
                    Some(value.id),
                    Some(value.issue_date),
                );
                Ok(Instrument::FloatingRateInstrument(instrument))
            }
            _ => Err(AtlasError::NotImplementedErr("RateType".to_string())),
        }
    }
}
