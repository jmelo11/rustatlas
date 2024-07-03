use std::cell::RefCell;

use super::adnum::{ADNode, ADNum};

/// Defines the default tape size.
const DEFAULT_TAPE_SIZE: usize = 1024;

thread_local! {
    /// Thread-local GradientTape
    pub static TAPE: GradientTape = GradientTape::new();
}

/// # GradientTape
/// A tape that records the differential operations performed on the different variables inside the computation.
pub struct GradientTape {
    tape: RefCell<Vec<ADNode>>,
    is_active: RefCell<bool>,
}

impl GradientTape {
    pub fn new() -> Self {
        let vec = Vec::with_capacity(DEFAULT_TAPE_SIZE);
        GradientTape {
            tape: RefCell::new(vec),
            is_active: RefCell::new(false),
        }
    }

    pub fn push_node(&self, node: ADNode) {
        if self.is_active() {
            let mut tape = self.tape.borrow_mut();
            tape.push(node);
        }
    }

    pub fn is_active(&self) -> bool {
        *self.is_active.borrow()
    }

    pub fn len(&self) -> usize {
        let tape = self.tape.borrow();
        tape.len()
    }

    pub fn adjoints(&self, num: &ADNum) -> Vec<f64> {
        let tape = self.tape.borrow();
        let mut adjoints = vec![0.0; tape.len()];
        let id = num.id();
        adjoints[id] = 1.0;

        for i in (0..id + 1).rev() {
            let node = tape.get(i).unwrap();
            if node.n_args() > 0 {
                adjoints[node.lhs_id().unwrap()] += adjoints[i] * node.lhs_der();
                if node.n_args() > 1 {
                    adjoints[node.rhs_id().unwrap()] += adjoints[i] * node.rhs_der();
                }
            }
        }
        adjoints
    }

    pub fn derivative(&self, target: &ADNum, num: &ADNum) -> f64 {
        let adjoints = self.adjoints(target);
        adjoints.get(num.id()).unwrap().clone()
    }

    pub fn clear(&self) {
        let mut tape = self.tape.borrow_mut();
        tape.clear();
    }

    pub fn activate(&self) {
        *self.is_active.borrow_mut() = true;
    }

    pub fn deactivate(&self) {
        *self.is_active.borrow_mut() = false;
    }
}

#[cfg(test)]
mod tests {

    use crate::prelude::NewNumeric;

    use super::*;

    #[test]
    fn test_gradient_tape() {
        TAPE.with(|tape| tape.activate());
        let x = ADNum::new(2.0);
        let y = ADNum::new(3.0);
        let z = x + y;
        let w = z * z;
        let _ = TAPE.with(|tape| tape.deactivate());
        let dz_dx = TAPE.with(|tape| tape.derivative(&w, &x));
        let dz_dy = TAPE.with(|tape| tape.derivative(&w, &y));
        assert_eq!(dz_dx, 10.0);
        assert_eq!(dz_dy, 10.0);
    }

    #[test]
    fn test_correct_value_after_activation() {
        // should not record the operations, so outscope vars should be considered as constants
        let x = ADNum::new(2.0);
        TAPE.with(|tape| tape.activate());
        let y = ADNum::new(3.0);
        let z = x * y;
        let dz_dy = TAPE.with(|tape| tape.derivative(&z, &y));
        assert_eq!(dz_dy, 2.0);

        // flip side
        let z = y * x;
        let dz_dy = TAPE.with(|tape| tape.derivative(&z, &y));
        assert_eq!(dz_dy, 2.0);

        // constant ops
        let x = ADNum::new(2.0);
        let y = 3.0;
        let z = y * x;
        let dz_dy = TAPE.with(|tape| tape.derivative(&z, &x));
        assert_eq!(dz_dy, 3.0);

        // addition
        TAPE.with(|tape| tape.deactivate());
        let x = ADNum::new(2.0);
        TAPE.with(|tape| tape.activate());
        let y = ADNum::new(3.0);
        let z = x + y;
        let dz_dy = TAPE.with(|tape| tape.derivative(&z, &y));
        assert_eq!(dz_dy, 1.0);

        // flip side
        let z = y + x;
        let dz_dy = TAPE.with(|tape| tape.derivative(&z, &y));
        assert_eq!(dz_dy, 1.0);
    }
}
