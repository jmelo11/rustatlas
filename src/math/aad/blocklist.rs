const BLOCK_SIZE: usize = 16384; // Adjust this value as needed

pub struct BlockList<T> {
    data: Vec<Vec<T>>,
    next_space: usize,
    marked_block: usize,
    marked_space: usize,
}

impl<T: Default> BlockList<T> {
    pub fn new() -> Self {
        let mut data = Vec::new();
        data.push(Vec::with_capacity(BLOCK_SIZE));
        BlockList {
            data,
            next_space: 0,
            marked_block: 0,
            marked_space: 0,
        }
    }

    fn new_block(&mut self) {
        self.data.push(Vec::with_capacity(BLOCK_SIZE));
        self.next_space = 0;
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.new_block();
    }

    pub fn rewind(&mut self) {
        self.next_space = 0;
    }

    pub fn memset(&mut self, value: T)
    where
        T: Clone,
    {
        for block in &mut self.data {
            for item in block.iter_mut() {
                *item = value.clone();
            }
        }
    }

    pub fn emplace_back(&mut self) -> &mut T {
        if self.next_space == BLOCK_SIZE {
            self.new_block();
        }
        self.data.last_mut().unwrap().push(T::default());
        self.next_space += 1;
        self.data.last_mut().unwrap().last_mut().unwrap()
    }

    pub fn emplace_back_multi(&mut self, n: usize) -> &mut [T] {
        if self.next_space + n > BLOCK_SIZE {
            self.new_block();
        }
        let last_block = self.data.last_mut().unwrap();
        last_block.extend((0..n).map(|_| T::default()));
        self.next_space += n;
        &mut last_block[(self.next_space - n)..self.next_space]
    }

    pub fn set_mark(&mut self) {
        if self.next_space == BLOCK_SIZE {
            self.new_block();
        }
        self.marked_block = self.data.len() - 1;
        self.marked_space = self.next_space;
    }

    pub fn rewind_to_mark(&mut self) {
        self.next_space = self.marked_space;
        self.data.truncate(self.marked_block + 1);
    }

   
    pub fn mark(&self) -> usize {
        self.marked_space
    }

    pub fn find(&self, element: &T) -> Option<usize>
    where
        T: PartialEq,
    {
        for (block_idx, block) in self.data.iter().enumerate().rev() {
            for (idx, item) in block.iter().enumerate().rev() {
                if item == element {
                    return Some(block_idx * BLOCK_SIZE + idx);
                }
            }
        }
        None
    }
}
