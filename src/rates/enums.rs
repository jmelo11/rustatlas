use serde::{Deserialize, Serialize};

use crate::utils::errors::{AtlasError, Result};

/// # Compounding
/// Enumerate the different compounding methods.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
    CompoundedThenSimple,
}

impl Compounding {
    pub fn from_str(s: &str) -> Result<Compounding> {
        match s {
            "Simple" => Ok(Compounding::Simple),
            "Compounded" => Ok(Compounding::Compounded),
            "Continuous" => Ok(Compounding::Continuous),
            "SimpleThenCompounded" => Ok(Compounding::SimpleThenCompounded),
            "CompoundedThenSimple" => Ok(Compounding::CompoundedThenSimple),
            _ => Err(AtlasError::InvalidValueErr(format!(
                "Invalid compounding: {}",
                s
            ))),
        }
    }
}
