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
// #[deprecated(note = "LoanDepo is deprecated and will be removed in future versions. Use specific instrument implementations instead.")]
#[derive(Serialize, Deserialize, Clone)]
pub struct LoanDepo {
    /// Optional unique identifier for the loan/deposit
    pub id: Option<usize>,
    /// MIS (Management Information System) identifier
    pub mis_id: String,
    /// Reference date for the instrument
    pub reference_date: Date,
    /// Loan/deposit configuration identifier
    pub loandepo_configuration_id: usize,
    /// Principal amount
    pub notional: f64,
    /// Optional issue date of the instrument
    pub issue_date: Option<Date>,
    /// Start date of the loan/deposit
    pub start_date: Date,
    /// End/maturity date of the loan/deposit
    pub end_date: Date,
    /// Credit status of the instrument
    pub credit_status: String,
    /// Structure type of the instrument
    pub structure: Structure,
    /// Side (asset or liability) of the instrument
    pub side: Side,
    /// Account type classification
    pub account_type: AccountType,
    /// Business segment
    pub segment: String,
    /// Geographic area
    pub area: String,
    /// Product family classification
    pub product_family: String,
    /// Payment frequency
    pub payment_frequency: Frequency,

    /// First FTP (Funds Transfer Pricing) rate
    pub first_ftp_rate: f64,
    /// First client rate
    pub first_client_rate: f64,
    /// Optional second FTP rate (after rate change)
    pub second_ftp_rate: Option<f64>,
    /// Optional second client rate (after rate change)
    pub second_client_rate: Option<f64>,

    /// Notional amount in local currency
    pub notional_local_ccy: Option<f64>,
    /// Outstanding balance
    pub outstanding: Option<f64>,
    /// Outstanding balance in local currency
    pub outstanding_local_ccy: Option<f64>,
    /// Readjustment amount in local currency
    pub readjustment_local_ccy: Option<f64>,
    /// Average outstanding balance
    pub avg_outstanding: Option<f64>,
    /// Average outstanding balance in local currency
    pub avg_outstanding_local_ccy: Option<f64>,
    /// Average readjustment amount
    pub avg_readjustment: Option<f64>,
    /// Average interest accrued
    pub avg_interest: Option<f64>,
    /// Average interest accrued in local currency
    pub avg_interest_local_ccy: Option<f64>,
    /// FTP interest amount
    pub ftp_interest: Option<f64>,
    /// FTP interest amount in local currency
    pub ftp_interest_local_ccy: Option<f64>,
    /// Earned interest amount
    pub earned_interest: Option<f64>,
    /// Earned interest amount in local currency
    pub earned_interest_local_ccy: Option<f64>,
    /// Margin amount
    pub margin: Option<f64>,
    /// Margin amount in local currency
    pub margin_local_ccy: Option<f64>,

    /// Interest amount
    pub interest: Option<f64>,
    /// Interest amount in local currency
    pub interest_local_ccy: Option<f64>,

    /// Type of rate (fixed or floating)
    pub rate_type: RateType,
    /// Frequency of the first rate period
    pub first_rate_frequency: Frequency,
    /// Day counter convention for the first rate
    pub first_rate_day_counter: DayCounter,
    /// Compounding convention for the first rate
    pub first_rate_compounding: Compounding,

    /// Months until the first coupon payment
    pub months_to_first_coupon: Option<i16>,
    /// Optional frequency of the second rate period
    pub second_rate_frequency: Option<Frequency>,
    /// Optional day counter convention for the second rate
    pub second_rate_day_counter: Option<DayCounter>,
    /// Optional compounding convention for the second rate
    pub second_rate_compounding: Option<Compounding>,

    /// Currency of the instrument
    pub currency: Currency,
    /// Discount curve identifier
    pub discount_curve_id: usize,
    /// Optional forecast curve identifier
    pub forecast_curve_id: Option<usize>,

    /// List of cash flows
    pub cashflows: Vec<Cashflow>,
    /// Evaluation mode (FTP rate or client rate)
    pub evaluation_mode: Option<EvaluationMode>,
    /// Optional date when the rate changes
    pub rate_change_date: Option<Date>,
    /// Source of the cash flows data
    pub cashflows_source: String,

    /// Optional date of the last repricing
    pub last_reprice_date: Option<Date>,
    /// Optional date of the next repricing
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
