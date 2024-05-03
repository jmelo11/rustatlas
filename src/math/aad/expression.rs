use std::marker::PhantomData;

/// # EvalExpr
/// This trait is implemented by all expressions.
pub trait EvalExpr: Copy {
    fn evaluate(&self) -> f64;
}

/// # Expr
/// This type is a holder for all expressions.
#[derive(Debug, Clone, Copy)]
pub struct Expr<T: EvalExpr> {
    expr: T,
}

impl<T: EvalExpr> Expr<T> {
    pub fn new(expr: T) -> Expr<T> {
        Expr { expr }
    }

    pub fn expr(&self) -> T {
        self.expr
    }
}

impl<T: EvalExpr> EvalExpr for Expr<T> {
    fn evaluate(&self) -> f64 {
        self.expr.evaluate()
    }
}

/// # Value
/// This type is used to hold a value. It is used as a base case for all expressions.
#[derive(Debug, Clone, Copy)]
pub struct Value {
    value: f64,
}

impl Value {
    pub fn new(value: f64) -> Expr<Value> {
        Expr {
            expr: Value { value },
        }
    }

    pub fn from_expr<T: EvalExpr>(expr: Expr<T>) -> Expr<Value> {
        Expr {
            expr: Value {
                value: expr.evaluate(),
            },
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl EvalExpr for Value {
    fn evaluate(&self) -> f64 {
        self.value
    }
}

impl Derivative for Value {
    fn derivative(&self, _v: f64) -> f64 {
        0.0
    }
}

impl Derivatives for Value {
    fn lhs_derivative(_l: f64, _r: f64, _v: f64) -> f64 {
        0.0
    }

    fn rhs_derivative(_l: f64, _r: f64, _v: f64) -> f64 {
        0.0
    }
}

/// # Derivative
/// This trait is implemented by unary methods that can be differentiated.
pub trait Derivative {
    fn derivative(&self, v: f64) -> f64;
}

/// # Derivatives
/// This trait is implemented by binary methods that can be differentiated.
pub trait Derivatives {
    fn lhs_derivative(l: f64, r: f64, v: f64) -> f64;
    fn rhs_derivative(l: f64, r: f64, v: f64) -> f64;
}

/// # ApplyBinaryOp
/// This trait is implemented by binary methods that can be applied.
pub trait ApplyBinaryOp<LHS: EvalExpr, RHS: EvalExpr>: Copy {
    fn apply(lhs: &LHS, rhs: &RHS) -> f64;
}

/// # BinaryOp
/// This type is used by binary operations.
#[derive(Debug, Clone, Copy)]
pub struct BinaryOp<LHS, RHS, Op>
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
    Op: ApplyBinaryOp<LHS, RHS>,
{
    lhs: LHS,
    rhs: RHS,
    _marker: PhantomData<Op>,
}

impl<LHS, RHS, Op> BinaryOp<LHS, RHS, Op>
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
    Op: ApplyBinaryOp<LHS, RHS>,
{
    pub fn new(lhs: LHS, rhs: RHS) -> BinaryOp<LHS, RHS, Op> {
        BinaryOp {
            lhs,
            rhs,
            _marker: PhantomData,
        }
    }
}

impl<LHS, RHS, Op> EvalExpr for BinaryOp<LHS, RHS, Op>
where
    LHS: EvalExpr + Derivatives,
    RHS: EvalExpr + Derivatives,
    Op: ApplyBinaryOp<LHS, RHS>,
{
    fn evaluate(&self) -> f64 {
        Op::apply(&self.lhs, &self.rhs)
    }
}

/// # ApplyUnaryOp
/// This trait is implemented by unary methods that can be applied.
pub trait ApplyUnaryOp<T>: Copy
where
    T: EvalExpr + Derivative,
{
    fn apply(expr: &T) -> f64;
}

/// # UnaryOp
/// This type is used by unary operations.
#[derive(Debug, Clone, Copy)]
pub struct UnaryOp<T, Op>
where
    T: EvalExpr + Derivative,
    Op: ApplyUnaryOp<T>,
{
    expr: T,
    _marker: PhantomData<Op>,
}

impl<T, Op> UnaryOp<T, Op>
where
    T: EvalExpr + Derivative,
    Op: ApplyUnaryOp<T>,
{
    pub fn new(expr: T) -> UnaryOp<T, Op> {
        UnaryOp {
            expr,
            _marker: PhantomData,
        }
    }
}

impl<T, Op> EvalExpr for UnaryOp<T, Op>
where
    T: EvalExpr + Derivative,
    Op: ApplyUnaryOp<T>,
{
    fn evaluate(&self) -> f64 {
        Op::apply(&self.expr)
    }
}
