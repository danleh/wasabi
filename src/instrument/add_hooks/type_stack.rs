// TODO move this into wasm crate, maybe utils submodule
// TODO block_stack as well?

use wasm::{BlockType, FunctionType, ValType};

use self::TypeStackElement::*;

/*
 * Data structure for representing the abstract "value stack", i.e., for every value at runtime this
 * holds the value's type.
 * Needed for:
 *  - type checking (although this is incomplete here: the stack at the end of blocks and functions is not checked to contain the correct result types)
 *  - monomorphization of polymorphic instructions Drop and Select:
 *      - they do not carry their argument types in the instruction itself (unlike, e.g., i32.add)
 *      - their argument types can also not be determined easily from the module information (e.g., function type for call instruction or local types for local.get etc.)
 */

#[derive(Debug)]
pub struct TypeStack(Vec<TypeStackElement>);

#[derive(Debug, PartialEq)]
enum TypeStackElement {
    Val(ValType),
    BlockBegin(BlockType),
    FunctionBegin,
    // TODO see add_hooks/mod.rs
    //    Unreachable,
}

impl TypeStack {
    pub fn new() -> Self {
        TypeStack(vec![FunctionBegin])
    }

    /// Returns the number of values on the type stack until the next "block stack"
    /// begins, i.e., either BlockBegin or FunctionBegin.
    pub fn block_depth(&self) -> usize {
        self.0.iter().rev().take_while(|el|
            match el {
                Val(_) => true,
                _ => false,
            }).count()
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
    pub fn instr(&mut self, ty: &FunctionType) {
        for &input_ty in ty.params.iter().rev() {
            assert_eq!(
                input_ty,
                self.pop_val(),
                "instruction expected input type, but stack top was"
            );
        }
        for &result_ty in ty.results.iter() {
            self.push_val(result_ty);
        }
    }

    pub fn begin(&mut self, block_ty: BlockType) {
        self.0.push(BlockBegin(block_ty))
    }

    /// implicitly pops all types from the stack until the last block begin
    /// pushes that blocks result type on the stack
    /// returns the BlockType of that last block, or None if the last block was the whole function
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
                Some(FunctionBegin) => return None,
            }
        }
    }

    pub fn else_(&mut self) {
        // reuse code from end...
        let block_ty = self.end().expect("else cannot end a function");
        // but undo pushing of block result (this will be done by the "real" end)
        if let BlockType(Some(ty)) = block_ty {
            assert_eq!(ty, self.pop_val());
        }
        self.begin(block_ty);
    }

    // TODO see add_hooks/mod.rs
    //    pub fn unreachable(&mut self) {
    //        self.0.push(TypeStackElement::Unreachable)
    //    }
}
