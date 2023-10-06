use serde::{Deserialize, Serialize};

/// # Compounding
/// Enumerate the different compounding methods.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Compounding {
    Simple,
    Compounded,
    Continuous,
    SimpleThenCompounded,
    CompoundedThenSimple,
}
