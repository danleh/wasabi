use ast::{BlockType, FunctionType, ValType};
use self::TypeStackElement::*;

#[derive(Debug, new)]
pub struct TypeStack(#[new(default)] Vec<TypeStackElement>);

#[derive(Debug, PartialEq)]
pub enum TypeStackElement {
    Val(ValType),
    BlockBegin(BlockType),
}

impl TypeStack {
    pub fn push(&mut self, ty: ValType) {
        self.0.push(Val(ty))
    }

    /// panics if stack is empty or if there was a block begin (and not a ValType)
    pub fn pop(&mut self) -> ValType {
        match self.0.pop() {
            None => panic!("tried to pop from empty type stack"),
            Some(BlockBegin(_)) => panic!("expected ValType on type stack, but got block begin marker indicating empty block stack; full type stack was {:?}", self.0),
            Some(Val(ty)) => ty
        }
    }

    /// convenience, pops and validates input_tys, then pushes the result_tys
    pub fn op(&mut self, input_tys: &[ValType], result_tys: &[ValType]) {
        for &input_ty in input_tys.iter().rev() {
            assert_eq!(input_ty, self.pop());
        }
        for &result_ty in result_tys {
            self.push(result_ty);
        }
    }

    pub fn begin_block(&mut self, block_ty: BlockType) {
        self.0.push(BlockBegin(block_ty))
    }

    /// implicitly pops all types from the stack until the last block begin
    /// pushes that blocks result type on the stack
    /// returns the BlockType of that last block
    pub fn end_block(&mut self) -> BlockType {
        loop {
            match self.0.pop() {
                None => panic!("tried to end block by popping from type stack until block begin, but no block begin was found"),
                Some(Val(_ty)) => {},
                Some(BlockBegin(block_ty)) => {
                    // NOTE there is no validation that the stack is correct at the end of a block
                    // it is unclear to me how it exactly works with, e.g., br/return + drops
                    if let BlockType(Some(ty)) = block_ty {
                        self.push(ty);
                    }
                    return block_ty
                },
            }
        }
    }
}
