use ast::{BlockType, FunctionType, ValType};
use self::TypeStackElement::*;

#[derive(Debug)]
pub struct TypeStack(Vec<TypeStackElement>);

#[derive(Debug, PartialEq)]
enum TypeStackElement {
    Val(ValType),
    BlockBegin(BlockType),
}

impl TypeStack {
    pub fn new() -> Self {
        TypeStack(vec![])
    }

    pub fn push_val(&mut self, ty: ValType) {
        self.0.push(Val(ty))
    }

    /// panics if stack is empty or if there was a block begin (and not a ValType)
    pub fn pop_val(&mut self) -> ValType {
        match self.0.pop() {
            None => panic!("tried to pop from empty type stack"),
            Some(BlockBegin(_)) => panic!("expected ValType on type stack, but got block begin marker indicating empty block stack; full type stack was {:?}", self.0),
            Some(Val(ty)) => ty
        }
    }

    /// convenience, pops and validates input_tys, then pushes the result_tys
    pub fn instr(&mut self, input_tys: &[ValType], result_tys: &[ValType]) {
        for &input_ty in input_tys.iter().rev() {
            assert_eq!(input_ty, self.pop_val());
        }
        for &result_ty in result_tys {
            self.push_val(result_ty);
        }
    }

    pub fn begin(&mut self, block_ty: BlockType) {
        self.0.push(BlockBegin(block_ty))
    }

    /// implicitly pops all types from the stack until the last block begin
    /// pushes that blocks result type on the stack
    /// returns the BlockType of that last block
    pub fn end(&mut self) -> BlockType {
        loop {
            match self.0.pop() {
                None => panic!("could not end block, no block begin was found on type stack"),
                Some(Val(_ty)) => {},
                Some(BlockBegin(block_ty)) => {
                    // NOTE there is no validation that the stack is correct at the end of a block
                    // it is unclear to me how it exactly works with, e.g., br/return + drops
                    if let BlockType(Some(ty)) = block_ty {
                        self.push_val(ty);
                    }
                    return block_ty
                },
            }
        }
    }

    pub fn else_(&mut self) {
        let block_ty = self.end();
        self.begin(block_ty);
    }
}
