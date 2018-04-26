use ast::{highlevel::Instr, Idx, Label};
use std::collections::HashMap;
use self::BlockStackElement::*;

#[derive(Debug)]
pub struct BlockStack {
    block_stack: Vec<BlockStackElement>,
    // pre-computed on new()
    begin_end_map: HashMap<Idx<Instr>, Idx<Instr>>,
}

#[derive(Debug, Clone)]
pub enum BlockStackElement {
    Function {
        end: Idx<Instr>,
    },
    Block {
        begin: Idx<Instr>,
        end: Idx<Instr>,
    },
    Loop {
        begin: Idx<Instr>,
        end: Idx<Instr>,
    },
    If {
        begin_if: Idx<Instr>,
        begin_else: Option<Idx<Instr>>,
        end: Idx<Instr>,
    },
    Else {
        begin_if: Idx<Instr>,
        begin_else: Idx<Instr>,
        end: Idx<Instr>,
    }
}

impl BlockStack {
    pub fn new(instrs: &[Instr]) -> Self {
        // build this already at construction, so that we know later in O(1) where the end's are
        let mut begin_end_map: HashMap<Idx<Instr>, Idx<Instr>> = HashMap::new();

        let mut begin_stack: Vec<Idx<Instr>> = vec![];
        for (iidx, instr) in instrs[..instrs.len() - 1].iter().enumerate() {
            let iidx = iidx.into();
            match *instr {
                Instr::Block(_) | Instr::Loop(_) | Instr::If(_) => begin_stack.push(iidx),
                Instr::Else | Instr::End => {
                    let begin_iidx = begin_stack.pop().expect("invalid block nesting: could not end block, stack was empty");
                    begin_end_map.insert(begin_iidx, iidx);
                    // special case: Else also starts its own block
                    if let Instr::Else = instr {
                        begin_stack.push(iidx);
                    }
                }
                _ => {}
            }
        }
        assert!(begin_stack.is_empty(), "invalid block nesting: some blocks were not closed, stack at end is {:?}", begin_stack);

        BlockStack {
            block_stack: vec![Function { end: (instrs.len() - 1).into() }],
            begin_end_map,
        }
    }

    pub fn begin_block(&mut self, begin: Idx<Instr>) {
        self.block_stack.push(Block {
            begin,
            end: *self.begin_end_map.get(&begin)
                .expect(&format!("invalid block nesting: could not find end for block begin at {:?}", begin)),
        });
    }

    pub fn begin_loop(&mut self, begin: Idx<Instr>) {
        self.block_stack.push(Loop {
            begin,
            end: *self.begin_end_map.get(&begin)
                .expect(&format!("invalid block nesting: could not find end for loop begin at {:?}", begin)),
        });
    }

    pub fn begin_if(&mut self, begin_if: Idx<Instr>) {
        let end_or_else = *self.begin_end_map.get(&begin_if)
            .expect(&format!("invalid block nesting: could not find end/else for if begin at {:?}", begin_if));

        let if_ = if let Some(&end) = self.begin_end_map.get(&end_or_else) {
            If {
                begin_if,
                begin_else: Some(end_or_else),
                end
            }
        } else {
            If {
                begin_if,
                begin_else: None,
                end: end_or_else,
            }
        };
        self.block_stack.push(if_);
    }

    /// returns instruction index of the matching if begin
    pub fn else_(&mut self) -> Idx<Instr> {
        match self.block_stack.pop() {
            Some(If { begin_if, begin_else: Some(begin_else), end }) => {
                self.block_stack.push(Else { begin_if, begin_else, end });
                begin_if
            }
            Some(block) => panic!("invalid block nesting: expected if with else on block stack, but got {:?}", block),
            None => panic!("invalid block nesting: expected if, but stack was empty"),
        }
    }

    pub fn end(&mut self) -> BlockStackElement {
        self.block_stack.pop().expect("invalid block nesting: could not end block, stack was empty")
    }

    pub fn br_target(&self, label: Idx<Label>) -> Idx<Instr> {
        let target_block = self.block_stack.iter()
            .rev().nth(label.0)
            .expect(&format!("invalid label: cannot find target block for {:?}", label));
        // backward branch for loops, forward for all other blocks
        match *target_block {
            Loop { begin, .. } => begin,
            Function { end } | Block { end, .. } | If { end, .. } | Else { end, .. } => end,
        }
    }
}
