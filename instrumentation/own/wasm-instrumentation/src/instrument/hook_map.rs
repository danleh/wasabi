use ast::{FunctionType, Idx, ValType, ValType::*};
use ast::highlevel::{Function, Instr, Instr::*, Module};
use std::collections::HashMap;
use super::block_stack::BlockStackElement;
use super::convert_i64::convert_i64_type;

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

pub struct HookMap {
    map: HashMap<String, Hook>,
    /// needed to determine the function index of the created hooks (should start after the functions
    /// that are already present in the module)
    function_count: usize,
}

impl HookMap {
    pub fn new(module: &Module) -> Self {
        HookMap {
            function_count: module.functions.len(),
            map: HashMap::new(),
        }
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

    pub fn instr_hook(&mut self, instr: &Instr, mut polymorphic_tys: Vec<ValType>) -> Instr {
        let name = mangle_polymorphic_name(instr.to_name(), &polymorphic_tys);
        let args = match *instr {
            /*
                monomorphic instructions:
                - 1 instruction : 1 hook
                - types are determined just from instruction
            */

            Nop | Unreachable => vec![],

            // condition
            If(_) => vec![I32],
            // target label, target instruction index (function index is same, since there are no non-local branches)
            Br(_) => vec![I32, I32],
            // condition, target label, target instruction index
            BrIf(_) => vec![I32, I32, I32],
            // index into static info brTables array, table index as popped from stack
            BrTable(_, _) => vec![I32, I32],

            // memory size
            MemorySize(_) => vec![I32],
            // delta, previous memory size
            MemoryGrow(_) => vec![I32, I32],

            // address, offset, alignment, loaded/stored value
            Load(op, _) => vec![I32, I32, I32, op.to_type().results[0]],
            Store(op, _) => vec![I32, I32, I32, op.to_type().inputs[0]],

            // inputs and results
            Const(val) => vec![val.to_type()],
            Numeric(op) => {
                let ty = op.to_type();
                [ty.inputs, ty.results].concat().to_vec()
            }


            /*
                polymorphic instructions:
                1. types cannot be determined just from the instruction but must be determined
                   by other means (e.g., stack typing) and are given through polymorphic_tys
                2. no 1:1 relation between instructions and hooks but rather 1:N with mangled names,
                   e.g., 1 polymorphic call instruction -> many monomorphic hooks like call_i32_i64
            */

            Drop => {
                assert_eq!(polymorphic_tys.len(), 1, "drop has only one argument");
                polymorphic_tys
            }
            Select => {
                assert_eq!(polymorphic_tys.len(), 2, "select has two polymorphic arguments");
                assert_eq!(polymorphic_tys[0], polymorphic_tys[1], "select arguments must be equal");
                // condition, two arguments
                vec![I32, polymorphic_tys[0], polymorphic_tys[1]]
            }
            Local(_, _) | Global(_, _) => {
                assert_eq!(polymorphic_tys.len(), 1, "local and global instructions have only one argument");
                // local/global index, value
                vec![I32, polymorphic_tys[0]]
            }
            Return => polymorphic_tys,
            Call(_) | CallIndirect(_, _) => {
                // call: target function index, call_indirect: table index
                polymorphic_tys.insert(0, I32);
                polymorphic_tys
            }


            /* instructions that need additional information and thus have own method */

            Block(_) | Loop(_) | Else | End => panic!("cannot get hook for block-type instruction with this method, please use the other methods specialized to the block type"),
        };
        self.get_or_insert_hook(name, args, "") // FIXME
    }

    /* special hooks that do not directly correspond to an instruction or need additional information */

    pub fn start_hook(&mut self) -> Instr {
        self.get_or_insert_hook(
            "start",
            vec![],
            "start: function(func, instr) { start({func, instr}); }")
    }

    pub fn call_post_hook(&mut self, result_tys: Vec<ValType>) -> Instr {
        self.get_or_insert_hook(
            mangle_polymorphic_name("call_post", &result_tys),
            result_tys,
            "") // FIXME
    }

    pub fn begin_function_hook(&mut self) -> Instr {
        self.get_or_insert_hook(
            "begin_function",
            vec![],
            r#"begin_function: function (func, instr) { begin({func, instr}, "function"); }"#)
    }

    pub fn begin_block_hook(&mut self) -> Instr {
        self.get_or_insert_hook(
            "begin_block",
            vec![],
            r#"begin_block: function (func, instr) { begin({func, instr}, "block"); }"#)
    }

    pub fn begin_loop_hook(&mut self) -> Instr {
        self.get_or_insert_hook(
            "begin_loop",
            vec![],
            r#"begin_loop: function (func, instr) { begin({func, instr}, "loop"); }"#)
    }

    pub fn begin_if_hook(&mut self) -> Instr {
        self.get_or_insert_hook(
            "begin_if",
            vec![],
            r#"begin_if: function (func, instr) { begin({func, instr}, "if"); }"#)
    }

    pub fn begin_else_hook(&mut self) -> Instr {
        // instruction location of matching if
        self.get_or_insert_hook(
            "begin_else",
            vec![I32],
            r#"begin_else: function (func, instr, if_instr) { begin({func, instr}, "else", {func, instr: if_instr}); }"#)
    }

    pub fn end_hook(&mut self, block: &BlockStackElement) -> Instr {
        let (name, tys) = match *block {
            // function begin is implicit anyway, so no hook argument
            BlockStackElement::Function { .. } => ("end_function", vec![]),
            // matching begin instruction index
            BlockStackElement::Block { .. } => ("end_block", vec![I32]),
            BlockStackElement::Loop { .. } => ("end_loop", vec![I32]),
            BlockStackElement::If { .. } => ("end_if", vec![I32]),
            // instruction index of matching if, instruction index of matching else
            BlockStackElement::Else { .. } => ("end_else", vec![I32, I32]),
        };
        self.get_or_insert_hook(name, tys, "") // FIXME
    }


    /* Implementation */

    /// returns a Call instruction to the hook, which either
    /// A) was freshly generated, since it was not requested with these types before,
    /// B) came from the internal hook map.
    fn get_or_insert_hook(&mut self, name: impl Into<String>, args: Vec<ValType>, js_code: impl Into<String>) -> Instr {
        let name = name.into();
        let hook_count = self.map.len();
        let hook = self.map.entry(name.clone()).or_insert(Hook {
            name,
            args,
            idx: (self.function_count + hook_count).into(),
            js_code: js_code.into(),
        });
        Call(hook.idx)
    }
}


/* utility functions */

/// e.g. "call" + [I32, F32] -> "call_i32_f32"
fn mangle_polymorphic_name(name: &str, tys: &[ValType]) -> String {
    let mut mangled = name.to_string();
    for ty in tys {
        mangled.push('_');
        mangled.push_str(&ty.to_string());
    }
    mangled
}