use wasm::ast::highlevel::{Function, Instr, Instr::Local, LocalOp::*};
use wasm::ast::{self, Idx};

/*
 * Helper functions for duplicating stack values
 * by saving them into locals and later restoring them from the locals again
 */

/// save top locals.len() stack values into locals with the given index
/// types of locals must match stack, not enforced by this function!
pub fn save_stack_to_locals(locals: &[Idx<ast::Local>]) -> Vec<Instr> {
    let mut instrs = Vec::new();
    // copy stack values into locals
    for &local in locals.iter().skip(1).rev() {
        instrs.push(Local(SetLocal, local));
    }
    // optimization: for first local on the stack / last one saved use tee_local instead of set_local + get_local
    for &local in locals.iter().next() {
        instrs.push(Local(TeeLocal, local));
    }
    // and restore (saving has removed them from the stack)
    for &local in locals.iter().skip(1) {
        instrs.push(Local(GetLocal, local));
    }
    return instrs;
}

/// restores locals back onto stack and inserts code that converts i64 -> (i32, i32)
/// function is necessary to get the types of the locals
pub fn restore_locals_with_i64_handling(
    locals: &[Idx<ast::Local>],
    function: &Function,
) -> Vec<Instr> {
    let mut instrs = Vec::new();
    for &local in locals {
        instrs.append(&mut super::convert_i64::convert_i64_instr(
            Local(GetLocal, local),
            function.local_type(local),
        ));
    }
    return instrs;
}
