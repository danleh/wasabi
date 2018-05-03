use ast::{FunctionType, Idx, ValType::{self, *}};
use ast::highlevel::{Function, Module, Instr::{self, *}};
use std::collections::HashMap;
use super::convert_i64::convert_i64_type;

// TODO merge/remove js_codegen
pub struct Hook {
    /// low-level hook name, i.e., exactly as it appears in the Wasm import section and as a
    /// property on the imports object in the host environment
    name: String,
    /// not including the (i32, i32) instruction location and not yet lowering i64 -> (i32, i32)
    args: Vec<ValType>,
    /// to-be function index in the module
    pub idx: Idx<Function>,
    pub js_code: String,
}

impl Hook {
    pub fn to_function(&self) -> Function {
        // prepend two I32 for (function idx, instr idx)
        let mut args = vec![I32, I32];
        args.extend(self.args.iter()
            // and expand i64 to a tuple of (i32, i32) since there is no JS interop for i64
            .flat_map(convert_i64_type));

        Function {
            // hooks do not return anything
            type_: FunctionType::new(args, vec![]),
            import: Some(("__wasabi_hooks".to_string(), self.name.clone())),
            code: None,
            export: None,
        }
    }
}

struct HookMap {
    map: HashMap<String, Hook>,
    function_count: usize,
}

impl HookMap {
    pub fn new(module: &Module) -> Self {
        HookMap {
            function_count: module.functions.len(),
            map: HashMap::new(),
        }
    }

    /// convenience
    pub fn instr_hook(&mut self, instr: &Instr, _additional_args: &[ValType]) {
        // TODO impl
        match *instr {
            Instr::Unreachable => {}
            Instr::Nop => {}
            Instr::Block(_) => {}
            Instr::Loop(_) => {}
            Instr::If(_) => {}
            Instr::Else => {}
            Instr::End => {}
            Instr::Br(_) => {}
            Instr::BrIf(_) => {}
            Instr::BrTable(_, _) => {}
            Instr::Return => {}
            Instr::Call(_) => {}
            Instr::CallIndirect(_, _) => {}
            Instr::Drop => {}
            Instr::Select => {}
            Instr::Local(_, _) => {}
            Instr::Global(_, _) => {}
            Instr::Load(_, _) => {}
            Instr::Store(_, _) => {}
            Instr::MemorySize(_) => {}
            Instr::MemoryGrow(_) => {}
            Instr::Const(_) => {}
            Instr::Numeric(_) => {}
        }
    }

    pub fn begin_hook() {}
    pub fn end_hook() {}
    pub fn call_post_hook() {}
    pub fn start_hook() {}
    // TODO any more non-instr hooks?

    /// base method, fallback
    fn manual_hook(&mut self, name: String, args: Vec<ValType>, js_code: String) -> Instr {
        let hook_count = self.map.len();
        let hook = self.map.entry(name.clone()).or_insert(Hook {
            name,
            args,
            idx: (self.function_count + hook_count).into(),
            js_code,
        });
        Call(hook.idx)
    }

    /// consumes the internally collected on-demand hooks
    /// returns the to-be-added functions in insertion order (i.e., you can use their idx to
    /// double-check whether no other functions were added to the module in the meantime).
    #[must_use]
    pub fn finish(self) -> Vec<Hook> {
        let mut result: Vec<_> = self.map.into_iter().map(|(_k, v)| v).collect();
        result.sort_by_key(|hook| hook.idx);
        result
    }
}
