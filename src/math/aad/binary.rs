use crate::math::aad::expression::{ApplyBinaryOp, BinaryOp, Derivatives, EvalExpr, Expr};

/// # AddOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct AddOp;

impl<LHS, RHS> ApplyBinaryOp<LHS, RHS> for AddOp
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
{
    fn apply(lhs: &LHS, rhs: &RHS) -> f64 {
        lhs.evaluate() + rhs.evaluate()
    }
}

impl Derivatives for AddOp {
    fn lhs_derivative(l: f64, _: f64, _: f64) -> f64 {
        l
    }
    fn rhs_derivative(_: f64, r: f64, _: f64) -> f64 {
        r
    }
}

/// # SubOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct SubOp;

impl<LHS, RHS> ApplyBinaryOp<LHS, RHS> for SubOp
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
{
    fn apply(lhs: &LHS, rhs: &RHS) -> f64 {
        lhs.evaluate() - rhs.evaluate()
    }
}

impl Derivatives for SubOp {
    fn lhs_derivative(l: f64, _: f64, _: f64) -> f64 {
        l
    }
    fn rhs_derivative(_: f64, r: f64, _: f64) -> f64 {
        -r
    }
}

/// # MulOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct MulOp;

impl<LHS, RHS> ApplyBinaryOp<LHS, RHS> for MulOp
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
{
    fn apply(lhs: &LHS, rhs: &RHS) -> f64 {
        lhs.evaluate() * rhs.evaluate()
    }
}

impl Derivatives for MulOp {
    fn lhs_derivative(_: f64, r: f64, _: f64) -> f64 {
        r
    }
    fn rhs_derivative(l: f64, _: f64, _: f64) -> f64 {
        l
    }
}

/// # DivOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct DivOp;

impl<LHS, RHS> ApplyBinaryOp<LHS, RHS> for DivOp
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
{
    fn apply(lhs: &LHS, rhs: &RHS) -> f64 {
        lhs.evaluate() / rhs.evaluate()
    }
}

impl Derivatives for DivOp {
    fn lhs_derivative(_: f64, r: f64, _: f64) -> f64 {
        1.0 / r
    }
    fn rhs_derivative(l: f64, r: f64, _: f64) -> f64 {
        -l / r.powi(2)
    }
}

/// # PowOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct PowOp;

impl<LHS, RHS> ApplyBinaryOp<LHS, RHS> for PowOp
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
{
    fn apply(lhs: &LHS, rhs: &RHS) -> f64 {
        lhs.evaluate().powf(rhs.evaluate())
    }
}

impl Derivatives for PowOp {
    fn lhs_derivative(l: f64, r: f64, v: f64) -> f64 {
        r * v / l
    }
    fn rhs_derivative(l: f64, _: f64, v: f64) -> f64 {
        l.ln() * v
    }
}

pub fn powf<T: EvalExpr + Derivatives, U: EvalExpr + Derivatives>(
    expr: Expr<T>,
    power: Expr<U>,
) -> Expr<BinaryOp<T, U, PowOp>> {
    Expr::new(BinaryOp::new(expr.expr(), power.expr()))
}
