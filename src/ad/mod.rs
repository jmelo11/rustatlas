//! Automatic differentiation (AD) support.
//!
//! Provides the generic nestable [`Fwd`](crate::ad::forward::Fwd) for
//! forward-mode AD of arbitrary order (`Fwd<f64>` = 1st, `Fwd<Fwd<f64>>` =
//! 2nd, …), a shared [`Tape`](crate::ad::tape::Tape) for recording
//! operations, and graph [`TapeNode`](crate::ad::node::TapeNode)s for
//! backward-mode adjoint propagation. Mixed mode is available as
//! `Dual<FwdN>` (e.g. [`DualFwd`](crate::ad::dual::DualFwd)).

/// Block-based slab allocator for tape nodes.
pub mod blocklist;
/// Constant wrapper (Const<T>).
pub mod constant;
/// Backward-mode AD wrapper (Dual<T>) and DualFwd alias.
pub mod dual;
/// Expression-template system (Expr, operators, BinExpr, UnExpr, FloatExt, free fns).
pub mod expr;
/// Forward-mode AD type (Fwd<T>, nestable to arbitrary order).
pub mod forward;
/// Node module.
pub mod node;
/// Scalar and InnerScalar traits.
pub mod scalar;
/// Tape node module.
pub mod tape;
