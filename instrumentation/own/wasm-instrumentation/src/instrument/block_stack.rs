use ast::{Idx, Label};

#[derive(Debug)]
pub struct BlockStack(Vec<Begin>);

// TODO refactor (use InstructionLocation, not raw usize)
/// also keeps instruction index, needed later for End hooks
#[derive(Debug, PartialEq)]
pub enum Begin {
    // function begins correspond to no actual instruction, so no instruction index
    Function,
    Block(usize),
    Loop(usize),
    If(usize),
    Else(usize),
}

impl BlockStack {
    pub fn new() -> Self {
        BlockStack(vec![Begin::Function])
    }

    // TODO more meaningfull functions for beginning of a block, e.g. begin_loop vs begin_block or so
    pub fn push(&mut self, begin: Begin) {
        self.0.push(begin)
    }

    pub fn pop(&mut self) -> Begin {
        self.0.pop().expect("tried to pop from empty block stack")
    }

    // TODO document
    pub fn label_to_instr_idx(&self, label: Idx<Label>) -> usize {
        let target_block = self.0.iter()
            .rev().nth(label.0)
            .expect(&format!("cannot resolve target for {:?}", label));
        match *target_block {
            Begin::Function => 0,
            Begin::Loop(begin_iidx) => begin_iidx,
            // FIXME if/else/block (forward jump, needs forward scanning for End)
            Begin::If(i) | Begin::Else(i) | Begin::Block(i) => i
        }
    }
}

/// just a validity check
impl Drop for BlockStack {
    fn drop(&mut self) {
        assert!(self.0.is_empty(), "expected empty block stack at end of function, but got {:?}", self.0);
    }
}