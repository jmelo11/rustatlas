use std::{
    cell::RefCell,
    fmt::Display,
    ops::{Add, Mul},
    rc::Rc,
};

thread_local! {
    static TAPE: Rc<RefCell<Vec<ADNode>>> = Rc::new(RefCell::new(Vec::new()));
}

fn push_node(node: ADNode) {
    TAPE.with(|tape| {
        let mut tape = tape.borrow_mut();
        tape.push(node);
    });
}

fn tape_size() -> usize {
    TAPE.with(|tape| tape.borrow().len())
}

pub fn get_tape() -> Rc<RefCell<Vec<ADNode>>> {
    TAPE.with(|tape| tape.clone())
}

#[allow(dead_code)]
pub fn calculate_adjoints(num: &ADNum) -> Vec<f64> {
    let tmp = get_tape();
    let tape = tmp.borrow();

    let mut adjoints = vec![0.0; tape.len()];
    let id = num.id;
    adjoints[id] = 1.0;

    for i in (0..id + 1).rev() {
        let node = tape.get(i).unwrap();
        if node.n_args() > 0 {
            let adj = adjoints[i] * node.lhs_der;
            adjoints[node.lhs_id.unwrap()] += adj;

            if node.n_args() > 1 {
                adjoints[node.rhs_id.unwrap()] += adjoints[i] * node.rhs_der;
            }
        }
    }

    adjoints
}

#[allow(dead_code)]
pub fn derivative(num: &ADNum) -> f64 {
    let adjoints = calculate_adjoints(num);
    adjoints.get(num.id).unwrap().clone()
}

#[allow(dead_code)]
pub struct ADNode {
    lhs_der: f64,
    rhs_der: f64,
    n_args: usize,
    lhs_id: Option<usize>,
    rhs_id: Option<usize>,
}

impl ADNode {
    pub fn new(
        lhs_der: f64,
        rhs_der: f64,
        n_args: usize,
        lhs_id: Option<usize>,
        rhs_id: Option<usize>,
    ) -> ADNode {
        ADNode {
            lhs_der,
            rhs_der,
            n_args,
            lhs_id,
            rhs_id,
        }
    }

    #[allow(dead_code)]
    pub fn lhs_der(&self) -> f64 {
        self.lhs_der
    }

    #[allow(dead_code)]
    pub fn rhs_der(&self) -> f64 {
        self.rhs_der
    }

    #[allow(dead_code)]
    pub fn lhs_id(&self) -> Option<usize> {
        self.lhs_id
    }

    #[allow(dead_code)]
    pub fn rhs_id(&self) -> Option<usize> {
        self.rhs_id
    }

    #[allow(dead_code)]
    pub fn n_args(&self) -> usize {
        self.n_args
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ADNum {
    value: f64,
    id: usize,
}

impl Display for ADNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ADNum {
    pub fn new(value: f64) -> ADNum {
        let id = tape_size();

        // Create a new node for the new variable
        let node = ADNode::new(1.0, 0.0, 0, None, None);
        push_node(node);
        ADNum { value, id }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl Add for ADNum {
    type Output = ADNum;

    fn add(self, other: ADNum) -> ADNum {
        let result = self.value + other.value;
        let lhs_der = 1.0;
        let rhs_der = 1.0;
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new(lhs_der, rhs_der, 2, lhs_id, rhs_id);
        let id = tape_size();
        push_node(node);
        ADNum {
            value: result,
            id: id,
        }
    }
}

impl Mul for ADNum {
    type Output = ADNum;

    fn mul(self, other: ADNum) -> ADNum {
        let result = self.value * other.value;

        let lhs_der = other.value;
        let rhs_der = self.value;
        let lhs_id = Some(self.id);
        let rhs_id = Some(other.id);

        let node = ADNode::new(lhs_der, rhs_der, 2, lhs_id, rhs_id);
        let id = tape_size();
        push_node(node);

        ADNum {
            value: result,
            id: id,
        }
    }
}

// impl AddAssign for ADNum {
//     fn add_assign(&mut self, other: ADNum) {
//         self.value += other.value;
//     }
// }

// impl Sub for ADNum {
//     type Output = ADNum;

//     fn sub(self, other: ADNum) -> ADNum {
//         let id = tape_size();
//         ADNum {
//             value: self.value - other.value,
//             id: id,
//         }
//     }
// }

// impl SubAssign for ADNum {
//     fn sub_assign(&mut self, other: ADNum) {
//         self.value -= other.value;
//     }
// }

// impl MulAssign for ADNum {
//     fn mul_assign(&mut self, other: ADNum) {
//         self.value *= other.value;
//     }
// }

// impl Div for ADNum {
//     type Output = ADNum;

//     fn div(self, other: ADNum) -> ADNum {
//         ADNum {
//             value: self.value / other.value,
//         }
//     }
// }

// impl DivAssign for ADNum {
//     fn div_assign(&mut self, other: ADNum) {
//         self.value /= other.value;
//     }
// }

// impl PartialEq for ADNum {
//     fn eq(&self, other: &ADNum) -> bool {
//         self.value == other.value
//     }
// }

// impl PartialOrd for ADNum {
//     fn partial_cmp(&self, other: &ADNum) -> Option<std::cmp::Ordering> {
//         self.value.partial_cmp(&other.value)
//     }
// }

// impl Eq for ADNum {}

// impl Ord for ADNum {
//     fn cmp(&self, other: &ADNum) -> std::cmp::Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

