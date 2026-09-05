//! Market index definitions.
//!
//! Provides the [`MarketIndex`](crate::indices::marketindex::MarketIndex) enumeration,
//! the [`FxPair`](crate::indices::fxpair::FxPair) value type for FX currency pairs,
//! rate-index trait definitions, and concrete implementations for
//! major overnight and term indices (SOFR, ESTR, EURIBOR, SONIA, etc.).

pub mod fxpair;
pub mod marketindex;
pub mod quotetype;
pub mod rateindex;
pub mod rateindices;
