use chrono::ParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AtlasError {
    #[error("Not found error: {0}")]
    NotFoundErr(String),
    #[error("Date parsing error: {0}")]
    DateParsingErr(#[from] ParseError),
    #[error("Period parsing error: {0}")]
    PeriodParsingErr(String),
    #[error("Period operation error: {0}")]
    PeriodOperationErr(String),
    #[error("MakeSchedule error: {0}")]
    MakeScheduleErr(String),
    #[error("Evaluation error: {0}")]
    EvaluationErr(String),
    #[error("Serialization error: {0}")]
    SerializationErr(String),
    #[error("Deserialization error: {0}")]
    DeserializationErr(String),
    #[error("Value not set error: {0}")]
    ValueNotSetErr(String),
    #[error("Invalid value error: {0}")]
    InvalidValueErr(String),
    #[error("Solver error: {0}")]
    SolverErr(#[from] argmin::core::Error),
    #[error("{0}")]
    NotImplementedErr(String),
}

pub type Result<T> = std::result::Result<T, AtlasError>;
