use ast::{BlockType, FunctionType, ValType};
use self::TypeStackElement::*;

/// Abstract Wasm stack
#[derive(Debug)]
pub struct TypeStack(Vec<TypeStackElement>);

#[derive(Debug, PartialEq)]
pub enum TypeStackElement {
    Val(ValType),
    BlockBegin(BlockType),
}

impl TypeStack {
    pub fn new() -> Self {
        TypeStack(Vec::new())
    }

    pub fn topn(&self, n: usize) -> Vec<ValType> {
        self.0.iter()
            .rev()
            .filter_map(|el| match el {
                Val(ty) => Some(*ty),
                _ => None
            })
            .take(n)
            .collect()
    }

    pub fn op(&mut self, input_tys: &[ValType], result_tys: &[ValType]) {
        for &input_ty in input_tys.iter().rev() {
            // TODO better error message
            assert_eq!(Val(input_ty), self.0.pop().expect("tried to pop empty abstract stack"), "invalid abstract stack")
        }
        for &result_ty in result_tys {
            self.0.push(Val(result_ty));
        }
    }

    pub fn begin_block(&mut self, block_ty: BlockType) {
        self.0.push(BlockBegin(block_ty))
    }

    pub fn end_block(&mut self) -> BlockType {
        // find last block begin
        let mut block_begin = None;
        for (idx, el) in self.0.iter().enumerate().rev() {
            match el {
                &BlockBegin(ty) => {
                    block_begin = Some((idx, ty));
                    break;
                }
                _ => {}
            }
        }

        if let Some((idx, block_ty)) = block_begin {
            self.0.truncate(idx);
            if let BlockType(Some(ty)) = block_ty {
                self.0.push(Val(ty));
            }
            block_ty
        } else {
            unreachable!("trying to end a block, but none was begun, abstract stack is {:?}", self);
        }
    }
}