use serde::{Serialize, Serializer};
use super::block_stack::{BlockStack, BlockStackElement};
use wasm::ast::{FunctionType, Idx, Label, Val, ValType};
use wasm::ast::highlevel::{Function, Instr, Module};

/*
 * Structs for static information that is generated during instrumentation and output as JSON
 * so that the dynamic analysis author does not also have to develop static analyses.
 */

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize)]
/// short for Location
pub struct Loc(usize);

impl Loc {
    pub fn to_const(&self) -> Instr {
        Instr::Const(Val::I32(self.0 as i32))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    pub functions: Vec<FunctionInfo>,
    #[serde(serialize_with = "serialize_types")]
    pub globals: Vec<ValType>,
    pub start: Option<Idx<Function>>,
    pub table_export_name: Option<String>,
    //    pub first_function_export_name: Option<String>,
    pub br_tables: Vec<BrTableInfo>,
    instr_count_cumulative: Vec<usize>,
}

impl<'a> From<&'a Module> for ModuleInfo {
    fn from(module: &Module) -> Self {
        let mut instr_count_cumulative = Vec::new();
        for func in &module.functions {
            let last = instr_count_cumulative.last().cloned().unwrap_or(0);
            instr_count_cumulative.push(last + func.instr_count());
        }

        ModuleInfo {
            functions: module.functions.iter().map(Into::into).collect(),
            globals: module.globals.iter().map(|g| g.type_.0).collect(),
            start: module.start,
            // if the module has no table, there cannot be a call_indirect, so this null will never be read from JS runtime
            table_export_name: module.tables.get(0).and_then(|table| table.export.iter().cloned().next()),
            // FIXME is this a valid workaround for wrong Firefox exported function .name property?
//            first_function_export_name: module.functions.get(0).and_then(|func| func.export.iter().cloned().next()),
            br_tables: vec![],
            instr_count_cumulative,
        }
    }
}

impl ModuleInfo {
    pub fn loc(&self, func: Idx<Function>, instr: Idx<Instr>) -> Loc {
        Loc(self.instr_count_cumulative[func.0]
            // add one extra instruction per function for "virtual instructions" like implicit returns
            // with Idx<Instr> == -1
            + func.0
            + instr.0
            + 1)
    }

    /// for an instruction that does not actually exist (e.g., implicit returns), but is attached to a function
    pub fn loc_virtual(&self, func: Idx<Function>) -> Loc {
        Loc(self.instr_count_cumulative[func.0]
            // add one extra instruction per function for "virtual instructions" like implicit returns
            // with Idx<Instr> == -1
            + func.0)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionInfo {
    // optimizations to keep the generated static info small: types and locals as strings
    #[serde(serialize_with = "serialize_function_type")]
    pub type_: FunctionType,
    pub import: Option<(String, String)>,
    pub export: Vec<String>,
    #[serde(serialize_with = "serialize_types")]
    pub locals: Vec<ValType>,
    pub instr_count: usize,
}

impl<'a> From<&'a Function> for FunctionInfo {
    fn from(function: &Function) -> FunctionInfo {
        FunctionInfo {
            type_: function.type_.clone(),
            import: function.import.clone(),
            export: function.export.clone(),
            locals: function.code.iter().flat_map(|code| code.locals.clone()).collect(),
            instr_count: function.instr_count(),
        }
    }
}

fn serialize_function_type<S>(ty: &FunctionType, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let mut type_str = String::new();
    for ty in &ty.params {
        type_str.push(ty.to_char());
    }
    type_str.push('|');
    for ty in &ty.results {
        type_str.push(ty.to_char());
    }
    s.serialize_str(&type_str)
}

fn serialize_types<S>(tys: &[ValType], s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    let mut type_str = String::new();
    for ty in tys {
        type_str.push(ty.to_char());
    }
    s.serialize_str(&type_str)
}

#[derive(Serialize)]
/// for resolving br_table instruction targets at runtime
pub struct BrTableInfo {
    pub table: Vec<Target>,
    pub default: Target,
}

impl BrTableInfo {
    /// needs block_stack for resolving the labels to actual locations
    pub fn from_br_table(table: &[Idx<Label>],
                         default: Idx<Label>,
                         block_stack: &BlockStack,
                         module_info: &ModuleInfo,
                         func: Idx<Function>) -> Self {
        BrTableInfo {
            table: table.iter().cloned().map(|l| Target::from_label(l, block_stack, module_info, func)).collect(),
            default: Target::from_label(default, block_stack, module_info, func),
        }
    }
}

// TODO rename all location in analyses -> loc to make serialized info smaller
// TODO remove this and replace with just BranchTarget?
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
/// carries the relative label (as it appears in the instructions) and the actual location
/// to which this label resolves to (statically computed with block_stack)
// FIXME better name. the only difference to BranchTarget is that this contains A) a label and B) Loc instead of Instr<Idx>
pub struct Target {
    label: Idx<Label>,
    loc: Loc,
    // for calling end() hooks of all intermediate blocks at runtime
    end_args: Vec<EndHookArgs>,
}

impl Target {
    fn from_label(label: Idx<Label>, block_stack: &BlockStack, module_info: &ModuleInfo, func: Idx<Function>) -> Self {
        let target = block_stack.br_target(label);
        Target {
            label,
            loc: module_info.loc(func, target.absolute_instr),
            end_args: target.ended_blocks.iter().map(|b| EndHookArgs::from_block_stack_element(b, module_info, func)).collect(),
        }
    }
}

struct EndHookArgs {
    end_loc: Loc,
    block_type: &'static str,
    begin_loc: Loc,
    // only when block_type == "else"
    begin_if_loc: Option<Loc>,
}

impl EndHookArgs {
    fn from_block_stack_element(bse: &BlockStackElement, module_info: &ModuleInfo, func: Idx<Function>) -> Self {
        // TODO this code is basically duplicated with the code in add_hooks/mod.rs
        use super::block_stack::BlockStackElement::*;
        match bse {
            Function { end } => EndHookArgs {
                end_loc: module_info.loc(func, *end),
                block_type: "function",
                begin_loc: module_info.loc_virtual(func),
                begin_if_loc: None,
            },
            Block { begin, end } => EndHookArgs {
                end_loc: module_info.loc(func, *end),
                block_type: "block",
                begin_loc: module_info.loc(func, *begin),
                begin_if_loc: None,
            },
            Loop { begin, end } => EndHookArgs {
                end_loc: module_info.loc(func, *end),
                block_type: "loop",
                begin_loc: module_info.loc(func, *begin),
                begin_if_loc: None,
            },
            If { begin_if, end, .. } => EndHookArgs {
                end_loc: module_info.loc(func, *end),
                block_type: "if",
                begin_loc: module_info.loc(func, *begin_if),
                begin_if_loc: None,
            },
            Else { begin_else, begin_if, end } => EndHookArgs {
                end_loc: module_info.loc(func, *end),
                block_type: "if",
                begin_loc: module_info.loc(func, *begin_else),
                begin_if_loc: Some(module_info.loc(func, *begin_if)),
            },
        }
    }
}

// space optimization when serializing: save arguments as tuple, not object with properties
impl Serialize for EndHookArgs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        if let Some(begin_if_loc) = self.begin_if_loc {
            (self.end_loc, self.block_type, self.begin_loc, begin_if_loc).serialize(serializer)
        } else {
            (self.end_loc, self.block_type, self.begin_loc).serialize(serializer)
        }
    }
}