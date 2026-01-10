//! RustAtlas is a Rust library for financial calculations and analysis.
//!
//! This library provides tools for working with asset-liability management,
//! cash flows, financial instruments, interest rates, and related computations.

/// Asset-liability management module.
pub mod alm;
/// Cash flows module.
pub mod cashflows;
/// Core types and utilities.
pub mod core;
/// Currency-related types and utilities.
pub mod currencies;
/// Financial instruments module.
pub mod instruments;
/// Mathematical functions and utilities.
pub mod math;
/// Financial models module.
pub mod models;
/// Prelude module for convenient imports.
pub mod prelude;
/// Interest rates module.
pub mod rates;
/// Time and date utilities.
pub mod time;
/// General utilities.
pub mod utils;
/// Visitor pattern implementations.
pub mod visitors;
