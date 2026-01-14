/// Double rate instrument module.
pub mod doublerateinstrument;
/// Fixed rate instrument module.
pub mod fixedrateinstrument;
/// Floating rate instrument module.
pub mod floatingrateinstrument;
/// Hybrid rate instrument module.
pub mod hybridrateinstrument;
/// Instrument base module.
pub mod instrument;
/// Leg module.
pub mod leg;
/// Loan deposit module.
pub mod loandepo;
/// Factory for double rate instruments.
pub mod makedoublerateinstrument;
/// Factory for fixed rate instruments.
pub mod makefixedrateinstrument;
/// Factory for fixed rate legs.
pub mod makefixedrateleg;
/// Factory for floating rate instruments.
pub mod makefloatingrateinstrument;
/// Factory for floating rate legs.
pub mod makefloatingrateleg;
/// Factory for swaps.
pub mod makeswap;
/// Swap module.
pub mod swap;
/// Common traits for instruments.
pub mod traits;
