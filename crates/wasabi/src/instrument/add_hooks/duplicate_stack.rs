use wasabi_wasm::Function;
use wasabi_wasm::Idx;
use wasabi_wasm::Instr;
use wasabi_wasm::Local;
use wasabi_wasm::LocalOp::*;

/*
 * Helper functions for duplicating stack values
 * by saving them into locals and later restoring them from the locals again
 */

/// save top locals.len() stack values into locals with the given index
/// types of locals must match stack, not enforced by this function!
pub fn save_stack_to_locals(
    append_to: &mut Vec<Instr>,
    locals: &[Idx<Local>]
) {
    // copy stack values into locals
    for &local in locals.iter().skip(1).rev() {
        append_to.push(Instr::Local(Set, local));
    }
    // optimization: for first local on the stack / last one saved use local.tee instead of local.set + local.get
    if let Some(&local) = locals.first() {
        append_to.push(Instr::Local(Tee, local));
    }
    // and restore (saving has removed them from the stack)
    for &local in locals.iter().skip(1) {
        append_to.push(Instr::Local(Get, local));
    }
}

/// restores locals back onto stack and inserts code that converts i64 -> (i32, i32)
/// function is necessary to get the types of the locals
pub fn restore_locals_with_i64_handling(
    append_to: &mut Vec<Instr>,
    locals: impl IntoIterator<Item=Idx<Local>>,
    function: &Function,
) {
    for local in locals {
        super::convert_i64::convert_i64_instr(
            append_to,
            Instr::Local(Get, local),
            function.param_or_local_type(local),
        );
    }
}
