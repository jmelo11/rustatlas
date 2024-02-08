use serde::{Deserialize, Serialize};

use crate::{
    alm::enums::{AccountType, EvaluationMode, ProductFamily, Segment},
    cashflows::{
        cashflow::{Cashflow, CashflowType, Side},
        fixedratecoupon::FixedRateCoupon,
        floatingratecoupon::FloatingRateCoupon,
        simplecashflow::SimpleCashflow,
    },
    currencies::enums::Currency,
    prelude::{InterestAccrual, Payable},
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
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LoanDepoCashflow {
    cashflow_type: CashflowType,
    payment_date: Date,
    notional: Option<f64>,
    amount: Option<f64>,
    accrual_start_date: Option<Date>,
    accrual_end_date: Option<Date>,
}

impl LoanDepoCashflow {
    pub fn new(
        cashflow_type: CashflowType,
        payment_date: Date,
        notional: Option<f64>,
        amount: Option<f64>,
        accrual_start_date: Option<Date>,
        accrual_end_date: Option<Date>,
    ) -> Self {
        LoanDepoCashflow {
            cashflow_type,
            payment_date,
            notional,
            amount,
            accrual_start_date,
            accrual_end_date,
        }
    }

    pub fn cashflow_type(&self) -> CashflowType {
        self.cashflow_type
    }

    pub fn payment_date(&self) -> Date {
        self.payment_date
    }

    pub fn notional(&self) -> Option<f64> {
        self.notional
    }

    pub fn amount(&self) -> Option<f64> {
        self.amount
    }

    pub fn accrual_start_date(&self) -> Option<Date> {
        self.accrual_start_date
    }

    pub fn accrual_end_date(&self) -> Option<Date> {
        self.accrual_end_date
    }
}

impl From<Cashflow> for LoanDepoCashflow {
    /// Converts a Cashflow into a LoanDepoCashflow
    fn from(cashflow: Cashflow) -> Self {
        match cashflow {
            Cashflow::Disbursement(cf) => LoanDepoCashflow::new(
                CashflowType::Disbursement,
                cf.payment_date(),
                None,
                Some(cf.amount().unwrap()),
                None,
                None,
            ),
            Cashflow::Redemption(cf) => LoanDepoCashflow::new(
                CashflowType::Redemption,
                cf.payment_date(),
                None,
                Some(cf.amount().unwrap()),
                None,
                None,
            ),
            Cashflow::FixedRateCoupon(cf) => LoanDepoCashflow::new(
                CashflowType::FixedRateCoupon,
                cf.payment_date(),
                Some(cf.notional()),
                Some(cf.amount().unwrap()),
                Some(cf.accrual_start_date()),
                Some(cf.accrual_end_date()),
            ),
            Cashflow::FloatingRateCoupon(cf) => LoanDepoCashflow::new(
                CashflowType::FloatingRateCoupon,
                cf.payment_date(),
                Some(cf.notional()),
                None,
                Some(cf.accrual_start_date()),
                Some(cf.accrual_end_date()),
            ),
        }
    }
}

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

    pub months_to_first_coupon: Option<i8>,
    pub second_rate_frequency: Option<Frequency>,
    pub second_rate_day_counter: Option<DayCounter>,
    pub second_rate_compounding: Option<Compounding>,

    pub currency: Currency,
    pub discount_curve_id: usize,
    pub forecast_curve_id: Option<usize>,

    pub cashflows: Vec<LoanDepoCashflow>,
    pub evaluation_mode: Option<EvaluationMode>,
    pub rate_change_date: Option<Date>,
    pub cashflows_source: Option<String>,
}

/// Auxiliary function to get the current rate and its definition
fn get_current_rate(
    cashflow: &LoanDepoCashflow,
    value: &LoanDepo,
) -> Result<(f64, RateDefinition)> {
    let first_rate_definition = RateDefinition::new(
        value.first_rate_day_counter,
        value.first_rate_compounding,
        value.first_rate_frequency,
    );
    match cashflow.cashflow_type {
        CashflowType::FixedRateCoupon => match value.rate_type {
            RateType::Fixed => {
                let rate = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                    Some(EvaluationMode::ClientRate) => value.first_client_rate,
                    None => return Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                };
                Ok((rate, first_rate_definition))
            }
            RateType::Floating => Err(AtlasError::InvalidValueErr(
                "Fixed rate type can only reproduce fixed rate coupons".to_string(),
            )),
            RateType::FixedThenFloating => {
                if value.rate_change_date.is_some() {
                    if value.rate_change_date.unwrap()
                        >= cashflow
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?
                    {
                        let r_day_counter = value
                            .second_rate_day_counter
                            .ok_or(AtlasError::ValueNotSetErr("Second DayCounter".to_string()))?;
                        let r_compounding = value
                            .second_rate_compounding
                            .ok_or(AtlasError::ValueNotSetErr("Second Compounding".to_string()))?;
                        let r_frequency = value
                            .second_rate_frequency
                            .ok_or(AtlasError::ValueNotSetErr("Second Frequency".to_string()))?;
                        let rate_definition =
                            RateDefinition::new(r_day_counter, r_compounding, r_frequency);
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => Ok((
                                value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second FTP Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            Some(EvaluationMode::ClientRate) => Ok((
                                value.second_client_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second Client Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    } else {
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => {
                                Ok((value.first_ftp_rate, first_rate_definition))
                            }
                            Some(EvaluationMode::ClientRate) => {
                                Ok((value.first_client_rate, first_rate_definition))
                            }
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    }
                } else {
                    Err(AtlasError::ValueNotSetErr("Rate Change Date".to_string()))
                }
            }

            RateType::FloatingThenFixed => {
                if value.rate_change_date.is_some() {
                    if value.rate_change_date.unwrap()
                        >= cashflow
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?
                    {
                        let r_day_counter = value
                            .second_rate_day_counter
                            .ok_or(AtlasError::ValueNotSetErr("Second DayCounter".to_string()))?;
                        let r_compounding = value
                            .second_rate_compounding
                            .ok_or(AtlasError::ValueNotSetErr("Second Compounding".to_string()))?;
                        let r_frequency = value
                            .second_rate_frequency
                            .ok_or(AtlasError::ValueNotSetErr("Second Frequency".to_string()))?;
                        let rate_definition =
                            RateDefinition::new(r_day_counter, r_compounding, r_frequency);
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => Ok((
                                value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second FTP Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            Some(EvaluationMode::ClientRate) => Ok((
                                value.second_client_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second Client Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    } else {
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => {
                                Ok((value.first_ftp_rate, first_rate_definition))
                            }
                            Some(EvaluationMode::ClientRate) => {
                                Ok((value.first_client_rate, first_rate_definition))
                            }
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    }
                } else {
                    Err(AtlasError::ValueNotSetErr("Rate Change Date".to_string()))
                }
            }
        },
        CashflowType::FloatingRateCoupon => match value.rate_type {
            RateType::Floating => {
                let rate = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                    Some(EvaluationMode::ClientRate) => value.first_client_rate,
                    None => return Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                };
                Ok((rate, first_rate_definition))
            }
            RateType::Fixed => Err(AtlasError::InvalidValueErr(
                "Floating rate type can only reproduce floating rate coupons".to_string(),
            )),
            RateType::FixedThenFloating => {
                if value.rate_change_date.is_some() {
                    if value.rate_change_date.unwrap()
                        >= cashflow
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?
                    {
                        let r_day_counter = value
                            .second_rate_day_counter
                            .ok_or(AtlasError::ValueNotSetErr("Second DayCounter".to_string()))?;
                        let r_compounding = value
                            .second_rate_compounding
                            .ok_or(AtlasError::ValueNotSetErr("Second Compounding".to_string()))?;
                        let r_frequency = value
                            .second_rate_frequency
                            .ok_or(AtlasError::ValueNotSetErr("Second Frequency".to_string()))?;
                        let rate_definition =
                            RateDefinition::new(r_day_counter, r_compounding, r_frequency);
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => Ok((
                                value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second FTP Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            Some(EvaluationMode::ClientRate) => Ok((
                                value.second_client_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second Client Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    } else {
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => {
                                Ok((value.first_ftp_rate, first_rate_definition))
                            }
                            Some(EvaluationMode::ClientRate) => {
                                Ok((value.first_client_rate, first_rate_definition))
                            }
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    }
                } else {
                    Err(AtlasError::ValueNotSetErr("Rate Change Date".to_string()))
                }
            }
            RateType::FloatingThenFixed => {
                if value.rate_change_date.is_some() {
                    if value.rate_change_date.unwrap()
                        >= cashflow
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?
                    {
                        let r_day_counter = value
                            .second_rate_day_counter
                            .ok_or(AtlasError::ValueNotSetErr("Second DayCounter".to_string()))?;
                        let r_compounding = value
                            .second_rate_compounding
                            .ok_or(AtlasError::ValueNotSetErr("Second Compounding".to_string()))?;
                        let r_frequency = value
                            .second_rate_frequency
                            .ok_or(AtlasError::ValueNotSetErr("Second Frequency".to_string()))?;
                        let rate_definition =
                            RateDefinition::new(r_day_counter, r_compounding, r_frequency);
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => Ok((
                                value.second_ftp_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second FTP Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            Some(EvaluationMode::ClientRate) => Ok((
                                value.second_client_rate.ok_or(AtlasError::ValueNotSetErr(
                                    "Second Client Rate".to_string(),
                                ))?,
                                rate_definition,
                            )),
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    } else {
                        match value.evaluation_mode {
                            Some(EvaluationMode::FTPRate) => {
                                Ok((value.first_ftp_rate, first_rate_definition))
                            }
                            Some(EvaluationMode::ClientRate) => {
                                Ok((value.first_client_rate, first_rate_definition))
                            }
                            None => Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                        }
                    }
                } else {
                    Err(AtlasError::ValueNotSetErr("Rate Change Date".to_string()))
                }
            }
        },
        _ => Err(AtlasError::InvalidValueErr(
            "Only fixed and floating coupons are allowed".to_string(),
        )),
    }
}

impl TryFrom<LoanDepo> for Instrument {
    type Error = AtlasError;

    fn try_from(value: LoanDepo) -> Result<Instrument> {
        let first_rate_definition = RateDefinition::new(
            value.first_rate_day_counter,
            value.first_rate_compounding,
            value.first_rate_frequency,
        );
        let cashflows = value
            .cashflows
            .iter()
            .map(|partial_cf| match partial_cf.cashflow_type() {
                CashflowType::Disbursement => {
                    let cf =
                        SimpleCashflow::new(partial_cf.payment_date, value.currency, value.side)
                            .with_amount(
                                partial_cf
                                    .amount()
                                    .ok_or(AtlasError::ValueNotSetErr("Amount".to_string()))?,
                            );
                    Ok(Cashflow::Disbursement(cf))
                }
                CashflowType::Redemption => {
                    let cf =
                        SimpleCashflow::new(partial_cf.payment_date, value.currency, value.side)
                            .with_amount(
                                partial_cf
                                    .amount()
                                    .ok_or(AtlasError::ValueNotSetErr("Amount".to_string()))?,
                            );
                    Ok(Cashflow::Redemption(cf))
                }
                CashflowType::FixedRateCoupon => {
                    let (rate, rate_definition) = get_current_rate(&partial_cf, &value)?;
                    let cf = FixedRateCoupon::new(
                        partial_cf
                            .notional
                            .ok_or(AtlasError::ValueNotSetErr("Notional".to_string()))?,
                        InterestRate::from_rate_definition(rate, rate_definition),
                        partial_cf
                            .accrual_start_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual Start Date".to_string()))?,
                        partial_cf
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?,
                        partial_cf.payment_date,
                        value.currency,
                        value.side,
                    );
                    Ok(Cashflow::FixedRateCoupon(cf))
                }
                CashflowType::FloatingRateCoupon => {
                    let (rate, rate_definition) = get_current_rate(&partial_cf, &value)?;
                    let cf = FloatingRateCoupon::new(
                        partial_cf
                            .notional
                            .ok_or(AtlasError::ValueNotSetErr("Notional".to_string()))?,
                        rate,
                        partial_cf
                            .accrual_start_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual Start Date".to_string()))?,
                        partial_cf
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?,
                        partial_cf.payment_date,
                        rate_definition,
                        value.currency,
                        value.side,
                    );
                    Ok(Cashflow::FloatingRateCoupon(cf))
                }
            })
            .collect::<Result<Vec<Cashflow>>>()?;

        match value.rate_type {
            RateType::Fixed => {
                let rate = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                    Some(EvaluationMode::ClientRate) => value.first_client_rate,
                    None => return Err(AtlasError::ValueNotSetErr("Evaluation Mode".to_string())),
                };
                let instrument = FixedRateInstrument::new(
                    value.start_date,
                    value.end_date,
                    value.notional,
                    InterestRate::from_rate_definition(rate, first_rate_definition),
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
                let rate = match value.evaluation_mode {
                    Some(EvaluationMode::FTPRate) => value.first_ftp_rate,
                    Some(EvaluationMode::ClientRate) => value.first_client_rate,
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
                    first_rate_definition,
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
