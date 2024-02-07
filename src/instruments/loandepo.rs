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
    rates::interestrate::{InterestRate, RateDefinition},
    time::{date::Date, enums::Frequency},
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
    side: Side,
    ftp_rate: Option<f64>,
    client_rate: Option<f64>,
    rate_definition: Option<RateDefinition>,
    currency: Currency,
    amount: Option<f64>,
    accrual_start_date: Option<Date>,
    accrual_end_date: Option<Date>,
}

impl LoanDepoCashflow {
    pub fn new(
        cashflow_type: CashflowType,
        payment_date: Date,
        notional: Option<f64>,
        side: Side,
        ftp_rate: Option<f64>,
        client_rate: Option<f64>,
        rate_definition: Option<RateDefinition>,
        currency: Currency,
        amount: Option<f64>,
        accrual_start_date: Option<Date>,
        accrual_end_date: Option<Date>,
    ) -> Self {
        LoanDepoCashflow {
            cashflow_type,
            payment_date,
            notional,
            side,
            ftp_rate,
            client_rate,
            rate_definition,
            currency,
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

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn currency(&self) -> Currency {
        self.currency
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

    pub fn ftp_rate(&self) -> Option<f64> {
        self.ftp_rate
    }

    pub fn client_rate(&self) -> Option<f64> {
        self.client_rate
    }

    pub fn rate_definition(&self) -> Option<RateDefinition> {
        self.rate_definition
    }
}

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

    pub first_rate_definition: RateDefinition,
    pub second_rate_definition: Option<RateDefinition>,

    pub currency: Currency,
    pub discount_curve_id: usize,
    pub forecast_curve_id: Option<usize>,

    pub rate_type: RateType,
    pub months_to_first_coupon: Option<i8>,
    pub cashflows: Vec<LoanDepoCashflow>,
    pub evaluation_mode: Option<EvaluationMode>,
}

impl TryFrom<LoanDepo> for Instrument {
    type Error = AtlasError;

    fn try_from(value: LoanDepo) -> Result<Instrument> {
        let cashflows = value
            .cashflows
            .iter()
            .map(|partial_cf| match partial_cf.cashflow_type() {
                CashflowType::Disbursement => {
                    let cf = SimpleCashflow::new(
                        partial_cf.payment_date,
                        partial_cf.currency,
                        partial_cf.side,
                    )
                    .with_amount(
                        partial_cf
                            .amount()
                            .ok_or(AtlasError::ValueNotSetErr("Amount".to_string()))?,
                    );
                    Ok(Cashflow::Disbursement(cf))
                }
                CashflowType::Redemption => {
                    let cf = SimpleCashflow::new(
                        partial_cf.payment_date,
                        partial_cf.currency,
                        partial_cf.side,
                    )
                    .with_amount(
                        partial_cf
                            .amount()
                            .ok_or(AtlasError::ValueNotSetErr("Amount".to_string()))?,
                    );
                    Ok(Cashflow::Redemption(cf))
                }
                CashflowType::FixedRateCoupon => {
                    let rate = match value
                        .evaluation_mode
                        .ok_or(AtlasError::ValueNotSetErr("Evaluation Mode".to_string()))?
                    {
                        EvaluationMode::FTPRate => partial_cf
                            .ftp_rate()
                            .ok_or(AtlasError::ValueNotSetErr("FTP Rate".to_string()))?,
                        EvaluationMode::ClientRate => partial_cf
                            .client_rate()
                            .ok_or(AtlasError::ValueNotSetErr("Client Rate".to_string()))?,
                    };
                    let cf = FixedRateCoupon::new(
                        partial_cf
                            .notional
                            .ok_or(AtlasError::ValueNotSetErr("Notional".to_string()))?,
                        InterestRate::from_rate_definition(
                            rate,
                            partial_cf
                                .rate_definition
                                .ok_or(AtlasError::ValueNotSetErr("Rate Definition".to_string()))?,
                        ),
                        partial_cf
                            .accrual_start_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual Start Date".to_string()))?,
                        partial_cf
                            .accrual_end_date
                            .ok_or(AtlasError::ValueNotSetErr("Accrual End Date".to_string()))?,
                        partial_cf.payment_date,
                        partial_cf.currency,
                        partial_cf.side,
                    );
                    Ok(Cashflow::FixedRateCoupon(cf))
                }
                CashflowType::FloatingRateCoupon => {
                    let rate = match value
                        .evaluation_mode
                        .ok_or(AtlasError::ValueNotSetErr("Evaluation Mode".to_string()))?
                    {
                        EvaluationMode::FTPRate => partial_cf
                            .ftp_rate()
                            .ok_or(AtlasError::ValueNotSetErr("FTP Rate".to_string()))?,
                        EvaluationMode::ClientRate => partial_cf
                            .client_rate()
                            .ok_or(AtlasError::ValueNotSetErr("Client Rate".to_string()))?,
                    };
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
                        partial_cf
                            .rate_definition
                            .ok_or(AtlasError::ValueNotSetErr("Rate Definition".to_string()))?,
                        partial_cf.currency,
                        partial_cf.side,
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
                    InterestRate::from_rate_definition(rate, value.first_rate_definition),
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
                    value.first_rate_definition,
                    value.structure,
                    value.currency,
                    Some(value.discount_curve_id),
                    value.forecast_curve_id,
                    Some(value.id),
                    Some(value.issue_date),
                );

                Ok(Instrument::FloatingRateInstrument(instrument))
            }
            _ => unimplemented!(),
        }
    }
}
