use std::cell::RefCell;
use std::ops::{Add, Div, Mul, Neg, Sub};

thread_local! {
    static TAPE: RefCell<Vec<Node>> = RefCell::new(Vec::new());
}

#[derive(Clone, Copy)]
enum Op {
    Input,
    Add,
    Sub,
    Mul,
    Div,
    Neg,
}

#[derive(Clone, Copy)]
struct Node {
    value: f64,
    op: Op,
    lhs: Option<usize>,
    rhs: Option<usize>,
}

fn push(node: Node) -> usize {
    TAPE.with(|t| {
        let mut t = t.borrow_mut();
        t.push(node);
        t.len() - 1
    })
}

fn value_of(id: usize) -> f64 {
    TAPE.with(|t| t.borrow()[id].value)
}

pub fn reset_tape() {
    TAPE.with(|t| t.borrow_mut().clear())
}

#[derive(Clone, Copy, Debug)]
pub struct Var {
    id: usize,
}

impl Var {
    pub fn new(value: f64) -> Var {
        let id = push(Node {
            value,
            op: Op::Input,
            lhs: None,
            rhs: None,
        });
        Var { id }
    }

    pub fn value(&self) -> f64 {
        value_of(self.id)
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl From<f64> for Var {
    fn from(value: f64) -> Self {
        Var::new(value)
    }
}

impl From<Var> for f64 {
    fn from(v: Var) -> Self {
        v.value()
    }
}

impl Add for Var {
    type Output = Var;
    fn add(self, rhs: Var) -> Var {
        let v = self.value() + rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Add,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Sub for Var {
    type Output = Var;
    fn sub(self, rhs: Var) -> Var {
        let v = self.value() - rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Sub,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Mul for Var {
    type Output = Var;
    fn mul(self, rhs: Var) -> Var {
        let v = self.value() * rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Mul,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Div for Var {
    type Output = Var;
    fn div(self, rhs: Var) -> Var {
        let v = self.value() / rhs.value();
        let id = push(Node {
            value: v,
            op: Op::Div,
            lhs: Some(self.id),
            rhs: Some(rhs.id),
        });
        Var { id }
    }
}

impl Neg for Var {
    type Output = Var;
    fn neg(self) -> Var {
        let v = -self.value();
        let id = push(Node {
            value: v,
            op: Op::Neg,
            lhs: Some(self.id),
            rhs: None,
        });
        Var { id }
    }
}

pub fn backward(result: &Var) -> Vec<f64> {
    TAPE.with(|t| {
        let tape = t.borrow();
        let mut grad = vec![0.0; tape.len()];
        grad[result.id] = 1.0;
        for i in (0..=result.id).rev() {
            let node = &tape[i];
            match node.op {
                Op::Input => {}
                Op::Add => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] += grad[i];
                }
                Op::Sub => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] -= grad[i];
                }
                Op::Mul => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] * rv;
                    grad[r] += grad[i] * lv;
                }
                Op::Div => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] / rv;
                    grad[r] -= grad[i] * lv / (rv * rv);
                }
                Op::Neg => {
                    let l = node.lhs.unwrap();
                    grad[l] -= grad[i];
                }
            }
        }
        grad
    })
}

pub fn grad_hessian(result: &Var, inputs: &[Var]) -> (Vec<f64>, Vec<Vec<f64>>) {
    TAPE.with(|t| {
        let tape = t.borrow();
        let m = tape.len();
        let n = inputs.len();
        let input_ids: Vec<usize> = inputs.iter().map(|v| v.id).collect();

        // forward pass - derivative of each node wrt inputs
        let mut deriv = vec![vec![0.0; n]; m];
        for i in 0..m {
            let node = &tape[i];
            match node.op {
                Op::Input => {
                    if let Some(pos) = input_ids.iter().position(|&id| id == i) {
                        deriv[i][pos] = 1.0;
                    }
                }
                Op::Add => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    for j in 0..n {
                        deriv[i][j] = deriv[l][j] + deriv[r][j];
                    }
                }
                Op::Sub => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    for j in 0..n {
                        deriv[i][j] = deriv[l][j] - deriv[r][j];
                    }
                }
                Op::Mul => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    for j in 0..n {
                        deriv[i][j] = deriv[l][j] * rv + lv * deriv[r][j];
                    }
                }
                Op::Div => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    for j in 0..n {
                        deriv[i][j] = (deriv[l][j] * rv - lv * deriv[r][j]) / (rv * rv);
                    }
                }
                Op::Neg => {
                    let l = node.lhs.unwrap();
                    for j in 0..n {
                        deriv[i][j] = -deriv[l][j];
                    }
                }
            }
        }

        // reverse pass - gradients and Hessian
        let mut grad = vec![0.0; m];
        let mut hess = vec![vec![0.0; n]; m];
        grad[result.id] = 1.0;

        for i in (0..=result.id).rev() {
            let node = &tape[i];
            match node.op {
                Op::Input => {}
                Op::Add => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] += grad[i];
                    for j in 0..n {
                        hess[l][j] += hess[i][j];
                        hess[r][j] += hess[i][j];
                    }
                }
                Op::Sub => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    grad[l] += grad[i];
                    grad[r] -= grad[i];
                    for j in 0..n {
                        hess[l][j] += hess[i][j];
                        hess[r][j] -= hess[i][j];
                    }
                }
                Op::Mul => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] * rv;
                    grad[r] += grad[i] * lv;
                    for j in 0..n {
                        hess[l][j] += hess[i][j] * rv + grad[i] * deriv[r][j];
                        hess[r][j] += hess[i][j] * lv + grad[i] * deriv[l][j];
                    }
                }
                Op::Div => {
                    let l = node.lhs.unwrap();
                    let r = node.rhs.unwrap();
                    let lv = tape[l].value;
                    let rv = tape[r].value;
                    grad[l] += grad[i] / rv;
                    grad[r] -= grad[i] * lv / (rv * rv);
                    for j in 0..n {
                        let d2_da = -deriv[r][j] / (rv * rv);
                        let d2_db = (2.0 * lv * deriv[r][j] / (rv * rv * rv))
                            - deriv[l][j] / (rv * rv);
                        hess[l][j] += hess[i][j] / rv + grad[i] * d2_da;
                        hess[r][j] += hess[i][j] * (-lv / (rv * rv)) + grad[i] * d2_db;
                    }
                }
                Op::Neg => {
                    let l = node.lhs.unwrap();
                    grad[l] -= grad[i];
                    for j in 0..n {
                        hess[l][j] -= hess[i][j];
                    }
                }
            }
        }

        let gradient: Vec<f64> = input_ids.iter().map(|&id| grad[id]).collect();
        let mut hessian = vec![vec![0.0; n]; n];
        for (row_idx, &id_row) in input_ids.iter().enumerate() {
            for (col_idx, _) in input_ids.iter().enumerate() {
                hessian[row_idx][col_idx] = hess[id_row][col_idx];
            }
        }

        (gradient, hessian)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_plus_test() {
        reset_tape();
        let x = Var::new(2.0);
        let y = x * x + x;
        let grad = backward(&y);
        assert!((grad[x.id()] - 5.0).abs() < 1e-12);
    }

    #[test]
    fn multivar_test() {
        reset_tape();
        let x = Var::new(3.0);
        let y = Var::new(4.0);
        let z = x * y + y * y;
        let grad = backward(&z);
        assert!((grad[x.id()] - 4.0).abs() < 1e-12);
        assert!((grad[y.id()] - 11.0).abs() < 1e-12);
    }

    #[test]
    fn div_sub_test() {
        reset_tape();
        let x = Var::new(5.0);
        let y = Var::new(2.0);
        let z = (x / y) - x;
        let grad = backward(&z);
        assert!((grad[x.id()] + 0.5).abs() < 1e-12);
        assert!((grad[y.id()] + 1.25).abs() < 1e-12);
    }

    #[test]
    fn hessian_square_plus() {
        reset_tape();
        let x = Var::new(2.0);
        let y = x * x + x;
        let (grad, hess) = grad_hessian(&y, &[x]);
        assert!((grad[0] - 5.0).abs() < 1e-12);
        assert!((hess[0][0] - 2.0).abs() < 1e-12);
    }

    #[test]
    fn hessian_multivar() {
        reset_tape();
        let x = Var::new(3.0);
        let y = Var::new(4.0);
        let z = x * y + y * y;
        let (grad, hess) = grad_hessian(&z, &[x, y]);
        assert!((grad[0] - 4.0).abs() < 1e-12);
        assert!((grad[1] - 11.0).abs() < 1e-12);
        assert!((hess[0][0]).abs() < 1e-12);
        assert!((hess[0][1] - 1.0).abs() < 1e-12);
        assert!((hess[1][0] - 1.0).abs() < 1e-12);
        assert!((hess[1][1] - 2.0).abs() < 1e-12);
    }
}

