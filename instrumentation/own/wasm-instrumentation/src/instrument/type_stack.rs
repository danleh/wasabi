use ast::{BlockType, ValType};
use self::TypeStackElement::*;

#[derive(Debug)]
pub struct TypeStack(Vec<TypeStackElement>);

#[derive(Debug, PartialEq)]
enum TypeStackElement {
    Val(ValType),
    BlockBegin(BlockType),
    FunctionBegin,
}

impl TypeStack {
    pub fn new() -> Self {
        TypeStack(vec![FunctionBegin])
    }

    pub fn push_val(&mut self, ty: ValType) {
        self.0.push(Val(ty))
    }

    /// panics if stack is empty or if there was a block begin (and not a ValType)
    pub fn pop_val(&mut self) -> ValType {
        match self.0.pop() {
            None => panic!("tried to pop from empty type stack"),
            Some(Val(ty)) => ty,
            Some(BlockBegin(_)) => panic!("expected ValType on type stack, but got block begin marker indicating empty block stack; full type stack was {:?}", self.0),
            Some(FunctionBegin) => panic!("expected ValType on type stack, but got function begin marker indicating empty block stack; full type stack was {:?}", self.0),
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
    /// returns the BlockType of that last block, or None if the last block was the function
    pub fn end(&mut self) -> Option<BlockType> {
        loop {
            match self.0.pop() {
                None => panic!("could not end block, no block begin was found on type stack"),
                Some(Val(_ty)) => {}
                Some(BlockBegin(block_ty)) => {
                    // NOTE there is no validation that the stack is correct at the end of a block
                    // it is unclear to me how it exactly works with, e.g., br/return + drops
                    if let BlockType(Some(ty)) = block_ty {
                        self.push_val(ty);
                    }
                    return Some(block_ty);
                }
                Some(FunctionBegin) => return None
            }
        }
    }

    pub fn else_(&mut self) {
        let block_ty = self.end().expect("else cannot end a function");
        self.begin(block_ty);
    }
}
