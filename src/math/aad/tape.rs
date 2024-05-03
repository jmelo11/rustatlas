use std::cell::RefCell;

pub struct Node {
    pub value: f64,
    pub id: usize,
    pub lhs: f64,
    pub rhs: f64,
    pub n_args: usize,
}

pub struct Tape {
    nodes: RefCell<Vec<Node>>,
}

const DEFAULT_TAPE_SIZE: usize = 1024;

impl Tape {
    pub fn new() -> Tape {
        Tape {
            nodes: RefCell::new(Vec::with_capacity(DEFAULT_TAPE_SIZE)),
        }
    }

    pub fn add_node(&self, node: Node) -> usize {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(node);
        nodes.len() - 1
    }
}
