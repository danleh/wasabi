use wasm::ast::{FunctionType, Idx, ValType, ValType::*};
use wasm::ast::highlevel::{Function, Instr, Instr::*, Module};
use std::collections::HashMap;
use super::block_stack::BlockStackElement;
use super::convert_i64::convert_i64_type;

/*
 * This does 3 things:
 *  - on-demand hook: only hooks for instructions that are actually present in the binary are generated and hooks that were already generated are re-used
 *  - monomorphization of polymorphic hooks: multiple monomorphized hook-variants are generated for one polymorphic instruction, such as call/return/drop/select etc.
 *  - JavaScript and Wasm hook codegen: generate imported functions with some type signature + matching low-level JavaScript functions that are glue-code to the high-level JavaScript hooks the user sees
 */

/// helper struct to encapsulate JavaScript arguments + their Wasm type
pub struct Arg {
    name: String,
    ty: ValType,
}

/// utility
impl Arg {
    /// for the parameter name in the low-level JavaScript function
    fn to_lowlevel_param_name(&self) -> String {
        match self.ty {
            I64 => self.name.clone() + "_low, " + &self.name + "_high",
            _ => self.name.clone()
        }
    }

    /// for the actual argument when forwarding to the high-level hook
    fn to_lowlevel_long_expr(&self) -> String {
        match self.ty {
            I64 => format!("new Long({})", self.to_lowlevel_param_name()),
            _ => self.name.clone()
        }
    }
}

/// to make creation of hooks easier and somewhat similar to rust function declarations (i.e. list of "name: type")
macro_rules! args {
    ($($name:ident: $ty:expr),*) => (vec![ $(Arg { name: stringify!($name).into(), ty: $ty }),* ]);
}

pub struct Hook {
    pub idx: Idx<Function>,
    pub wasm: Function,
    pub js: String,
}

impl Hook {
    /// args: do not include the (i32, i32) instruction location, also before i64 -> (i32, i32) lowering
    /// js_args: (quick and dirty, highly unsafe) JavaScript fragment, pasted into the high-level user hook call
    pub fn new(lowlevel_name: impl Into<String>, args: Vec<Arg>, highlevel_name: &str, js_args: &str) -> Self {
        let lowlevel_name = lowlevel_name.into();

        // generate JavaScript low-level hook that is called from Wasm and in turn calls the
        // high-level user analysis hook
        let js = format!("\"{}\": function (loc, {}) {{\n    Wasabi.analysis.{}(loc, {});\n}},",
                         &lowlevel_name,
                         args.iter().map(Arg::to_lowlevel_param_name).collect::<Vec<_>>().join(", "),
                         highlevel_name,
                         js_args);

        // generate low-level Wasm function to insert into the intrumented module
        let wasm = {
            // prepend I32 for location
            let mut lowlevel_args = vec![I32];
            lowlevel_args.extend(args.iter()
                // and expand i64 to a tuple of (i32, i32) since there is no JS interop for i64
                .flat_map(|Arg { name: _name, ref ty }| convert_i64_type(ty)));

            Function {
                // hooks do not return anything
                type_: FunctionType::new(lowlevel_args, vec![]),
                import: Some(("__wasabi_hooks".to_string(), lowlevel_name)),
                code: None,
                export: Vec::new(),
            }
        };

        Hook {
            wasm,
            js,
            // just a placeholder, replaced on insertion in the map
            idx: 0.into(),
        }
    }

    pub fn lowlevel_name(&self) -> String {
        self.wasm.import.as_ref().unwrap().1.clone()
    }
}


pub struct HookMap {
    /// remember requested (= already inserted) hooks by their low-level name
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

    pub fn instr(&mut self, instr: &Instr, polymorphic_tys: &[ValType]) -> Instr {
        let name = &mangle_polymorphic_name(instr.to_name(), polymorphic_tys)[..];
        let hook = match *instr {
            /*
                monomorphic instructions:
                - 1 instruction : 1 hook
                - types are determined just from instruction
            */

            Nop | Unreachable => Hook::new(name, args!(), name, ""),

            If(_) => Hook::new(name, args!(condition: I32), "if_", "condition === 1"),
            Br(_) => Hook::new(name, args!(label: I32, loc: I32), name, "{label, loc}"),
            BrIf(_) => Hook::new(name, args!(condition: I32, label: I32, loc: I32), name, "{label, loc}, condition === 1"),
            // NOTE js_args is very hacky! We rely on the Hook constructor to close the parenthesis and insert the call statement to endBrTableBlock() here
            BrTable(_, _) => Hook::new(name, args!(tableIdx: I32, brTablesInfoIdx: I32), name, "Wasabi.module.info.brTables[brTablesInfoIdx].table, Wasabi.module.info.brTables[brTablesInfoIdx].default, tableIdx); Wasabi.endBrTableBlocks(brTablesInfoIdx, tableIdx"),

            MemorySize(_) => Hook::new(name, args!(currentSizePages: I32), name, "currentSizePages"),
            MemoryGrow(_) => Hook::new(name, args!(deltaPages: I32, previousSizePages: I32), name, "deltaPages, previousSizePages"),

            Load(op, _) => {
                let ty = op.to_type().results[0];
                let args = args!(offset: I32, align: I32, addr: I32, value: ty);
                let instr_name = instr.to_name();
                let js_args = &format!("\"{}\", {{addr, offset, align}}, {}", instr_name, &args[3].to_lowlevel_long_expr());
                Hook::new(name, args, "load", js_args)
            }
            Store(op, _) => {
                let ty = op.to_type().inputs[1];
                let args = args!(offset: I32, align: I32, addr: I32, value: ty);
                let instr_name = instr.to_name();
                let js_args = &format!("\"{}\", {{addr, offset, align}}, {}", instr_name, &args[3].to_lowlevel_long_expr());
                Hook::new(name, args, "store", js_args)
            }

            Const(val) => {
                let args = args!(value: val.to_type());
                let js_args = &args[0].to_lowlevel_long_expr();
                Hook::new(name, args, "const_", js_args)
            }
            Numeric(op) => {
                let ty = op.to_type();
                let highlevel_name = match ty.inputs.len() {
                    1 => "unary",
                    2 => "binary",
                    _ => unreachable!()
                };
                let inputs = ty.inputs.iter().enumerate().map(|(i, &ty)| Arg { name: format!("input{}", i), ty });
                let results = ty.results.iter().enumerate().map(|(i, &ty)| Arg { name: format!("result{}", i), ty });
                let args = inputs.chain(results).collect::<Vec<_>>();
                let instr_name = instr.to_name();
                let js_args = &format!("\"{}\", {}", instr_name, args.iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, highlevel_name, js_args)
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
                let args = args!(value: polymorphic_tys[0]);
                let js_args = &args[0].to_lowlevel_long_expr();
                Hook::new(name, args, "drop", js_args)
            }
            Select => {
                assert_eq!(polymorphic_tys.len(), 2, "select has two polymorphic arguments");
                assert_eq!(polymorphic_tys[0], polymorphic_tys[1], "select arguments must be equal");
                let args = args!(condition: I32, input0: polymorphic_tys[0], input1: polymorphic_tys[1]);
                let js_args = &format!("condition === 1, {}", args[1..].iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "select", js_args)
            }
            Local(_, _) => {
                assert_eq!(polymorphic_tys.len(), 1, "local instructions have only one argument");
                let args = args!(index: I32, value: polymorphic_tys[0]);
                let instr_name = instr.to_name();
                let js_args = &format!("\"{}\", {}", instr_name, args.iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "local", js_args)
            }
            Global(_, _) => {
                assert_eq!(polymorphic_tys.len(), 1, "global instructions have only one argument");
                let args = args!(index: I32, value: polymorphic_tys[0]);
                let instr_name = instr.to_name();
                let js_args = &format!("\"{}\", {}", instr_name, args.iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "global", js_args)
            }
            Return => {
                let args = polymorphic_tys.iter().enumerate().map(|(i, &ty)| Arg { name: format!("result{}", i), ty }).collect::<Vec<_>>();
                let js_args = &format!("[{}]", args.iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "return_", js_args)
            }
            Call(_) => {
                let mut args = args!(targetFunc: I32);
                args.extend(polymorphic_tys.iter().enumerate().map(|(i, &ty)| Arg { name: format!("arg{}", i), ty }));
                // NOTE calls the high-level call_pre hook with one argument less than call_indirect, thus tableIdx === undefined since this is a direct call
                let js_args = &format!("targetFunc, [{}]", args[1..].iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "call_pre", js_args)
            }
            CallIndirect(_, _) => {
                let mut args = args!(tableIndex: I32);
                args.extend(polymorphic_tys.iter().enumerate().map(|(i, &ty)| Arg { name: format!("arg{}", i), ty }));
                let js_args = &format!("Wasabi.resolveTableIdx(tableIndex), [{}], tableIndex", args[1..].iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
                Hook::new(name, args, "call_pre", js_args)
            }


            /* instructions that need additional information and thus have own method */

            Block(_) | Loop(_) | Else | End => panic!("cannot get hook for block-type instruction with this method, please use the other methods specialized to the block type"),
        };
        self.get_or_insert(hook)
    }

    /* special hooks that do not directly correspond to an instruction or need additional information */

    pub fn start(&mut self) -> Instr {
        self.get_or_insert(Hook::new("start", vec![], "start", ""))
    }

    pub fn call_post(&mut self, result_tys: &[ValType]) -> Instr {
        let name = mangle_polymorphic_name("call_post", result_tys);
        let args = result_tys.iter().enumerate().map(|(i, &ty)| Arg { name: format!("result{}", i), ty }).collect::<Vec<_>>();
        let js_args = &format!("[{}]", args.iter().map(Arg::to_lowlevel_long_expr).collect::<Vec<_>>().join(", "));
        self.get_or_insert(Hook::new(name, args, "call_post", js_args))
    }

    pub fn begin_function(&mut self) -> Instr {
        self.get_or_insert(Hook::new("begin_function", vec![], "begin", "\"function\""))
    }

    pub fn begin_block(&mut self) -> Instr {
        self.get_or_insert(Hook::new("begin_block", vec![], "begin", "\"block\""))
    }

    pub fn begin_loop(&mut self) -> Instr {
        self.get_or_insert(Hook::new("begin_loop", vec![], "begin", "\"loop\""))
    }

    pub fn begin_if(&mut self) -> Instr {
        self.get_or_insert(Hook::new("begin_if", vec![], "begin", "\"if\""))
    }

    pub fn begin_else(&mut self) -> Instr {
        self.get_or_insert(Hook::new("begin_else", args!(ifLoc: I32), "begin", "\"else\", ifLoc"))
    }

    pub fn end(&mut self, block: &BlockStackElement) -> Instr {
        self.get_or_insert(match *block {
            BlockStackElement::Function { .. } => Hook::new("end_function", args!(beginLoc: I32), "end", "\"function\", beginLoc"),
            BlockStackElement::Block { .. } => Hook::new("end_block", args!(beginLoc: I32), "end", "\"block\", beginLoc"),
            BlockStackElement::Loop { .. } => Hook::new("end_loop", args!(beginLoc: I32), "end", "\"loop\", beginLoc"),
            BlockStackElement::If { .. } => Hook::new("end_if", args!(beginLoc: I32), "end", "\"if\", beginLoc"),
            BlockStackElement::Else { .. } => Hook::new("end_else", args!(elseLoc: I32, ifLoc: I32), "end", "\"else\", elseLoc, ifLoc"),
        })
    }

    /// returns a Call instruction to the requested hook, which either
    /// A) was freshly generated, since it was not requested with these types before,
    /// B) came from the internal hook map.
    fn get_or_insert(&mut self, hook: Hook) -> Instr {
        let hook_count = self.map.len();
        let hook = self.map.entry(hook.lowlevel_name().to_string()).or_insert(Hook {
            idx: (self.function_count + hook_count).into(),
            ..hook
        });
        Call(hook.idx)
    }
}


/* utility functions */

/// e.g. "call" + [I32, F64] -> "call_iF"
fn mangle_polymorphic_name(name: &str, tys: &[ValType]) -> String {
    let mut mangled = name.to_string();
    if !tys.is_empty() {
        mangled.push('_');
    }
    for ty in tys {
        mangled.push(ty.to_char());
    }
    mangled
}
