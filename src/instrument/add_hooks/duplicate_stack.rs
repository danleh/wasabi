use wasm::highlevel::{Function, Instr, Local, LocalOp::*};
use wasm::Idx;

/*
 * Helper functions for duplicating stack values
 * by saving them into locals and later restoring them from the locals again
 */

/// save top locals.len() stack values into locals with the given index
/// types of locals must match stack, not enforced by this function!
pub fn save_stack_to_locals(locals: &[Idx<Local>]) -> Vec<Instr> {
    let mut instrs = Vec::new();
    // copy stack values into locals
    for &local in locals.iter().skip(1).rev() {
        instrs.push(Instr::Local(Set, local));
    }
    // optimization: for first local on the stack / last one saved use local.tee instead of local.set + local.get
    if let Some(&local) = locals.first() {
        instrs.push(Instr::Local(Tee, local));
    }
    // and restore (saving has removed them from the stack)
    for &local in locals.iter().skip(1) {
        instrs.push(Instr::Local(Get, local));
    }
    instrs
}

/// restores locals back onto stack and inserts code that converts i64 -> (i32, i32)
/// function is necessary to get the types of the locals
pub fn restore_locals_with_i64_handling(
    locals: &[Idx<Local>],
    function: &Function,
) -> Vec<Instr> {
    let mut instrs = Vec::new();
    for &local in locals {
        instrs.append(&mut super::convert_i64::convert_i64_instr(
            Instr::Local(Get, local),
            function.param_or_local_type(local),
        ));
    }
    instrs
}
