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

impl TryFrom<String> for Compounding {
    type Error = AtlasError;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
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

impl From<Compounding> for String {
    fn from(compounding: Compounding) -> Self {
        match compounding {
            Compounding::Simple => "Simple".to_string(),
            Compounding::Compounded => "Compounded".to_string(),
            Compounding::Continuous => "Continuous".to_string(),
            Compounding::SimpleThenCompounded => "SimpleThenCompounded".to_string(),
            Compounding::CompoundedThenSimple => "CompoundedThenSimple".to_string(),
        }
    }
}
