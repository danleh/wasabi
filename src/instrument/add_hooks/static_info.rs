use serde::{Serialize, Serializer};
use wasm::{FunctionType, Idx, Label, ValType};
use wasm::highlevel::{Function, Instr, Module};

use super::block_stack::{BlockStack, BlockStackElement};

/*
 * Structs for static information that is generated during instrumentation and output as JSON
 * so that the dynamic analysis author does not also have to develop static analyses.
 */

#[derive(Serialize)]
pub struct ModuleInfo {
    pub functions: Vec<FunctionInfo>,
    #[serde(serialize_with = "serialize_types")]
    pub globals: Vec<ValType>,
    pub start: Option<Idx<Function>>,
    #[serde(rename = "tableExportName")]
    pub table_export_name: Option<String>,
    //    #[serde(rename = "firstFunctionExportName")]
    //    pub first_function_export_name: Option<String>,
    #[serde(rename = "brTables")]
    pub br_tables: Vec<BrTableInfo>,
}

impl<'a> From<&'a Module> for ModuleInfo {
    fn from(module: &Module) -> Self {
        ModuleInfo {
            functions: module.functions.iter().map(Into::into).collect(),
            globals: module.globals.iter().map(|g| g.type_.0).collect(),
            start: module.start,
            // if the module has no table, there cannot be a call_indirect, so this null will never be read from JS runtime
            table_export_name: module
                .tables
                .get(0)
                .and_then(|table| table.export.iter().cloned().next()),
            // FIXME is this a valid workaround for wrong Firefox exported function .name property?
            //            first_function_export_name: module.functions.get(0).and_then(|func| func.export.iter().cloned().next()),
            br_tables: vec![],
        }
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
    // TODO we could add name (from debug info) here, but it would be added for all functions,
    // in the JSON as `"name": null`, which is a lot of overhead...
}

impl<'a> From<&'a Function> for FunctionInfo {
    fn from(function: &Function) -> FunctionInfo {
        FunctionInfo {
            type_: function.type_.clone(),
            import: function.import().map(|(module, name)| (module.to_string(), name.to_string())),
            export: function.export.clone(),
            locals: function
                .code()
                .iter()
                .flat_map(|code| &code.locals)
                .map(|local| local.type_)
                .collect(),
            instr_count: function.instr_count(),
        }
    }
}

fn serialize_function_type<S>(ty: &FunctionType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let mut type_str = String::new();
    for ty in ty.params.iter() {
        type_str.push(ty.to_char());
    }
    type_str.push('|');
    for ty in ty.results.iter() {
        type_str.push(ty.to_char());
    }
    s.serialize_str(&type_str)
}

fn serialize_types<S>(tys: &[ValType], s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let mut type_str = String::new();
    for ty in tys {
        type_str.push(ty.to_char());
    }
    s.serialize_str(&type_str)
}

#[derive(Serialize)]
/// for resolving br_table instruction targets at runtime
pub struct BrTableInfo {
    pub table: Vec<ResolvedLabel>,
    pub default: ResolvedLabel,
}

impl BrTableInfo {
    /// needs block_stack for resolving the labels to actual locations
    pub fn from_br_table(
        table: &[Label],
        default: Label,
        block_stack: &BlockStack,
        func: Idx<Function>,
    ) -> Self {
        let resolve = |label: Label| {
            let target = block_stack.br_target(label);
            ResolvedLabel {
                label,
                location: Location(func, target.absolute_instr),
                end_blocks: target.ended_blocks,
            }
        };
        BrTableInfo {
            table: table.iter().cloned().map(resolve).collect(),
            default: resolve(default),
        }
    }
}

#[derive(Serialize)]
/// carries the relative label (as it appears in the instructions) and the actual instruction index
/// to which this label resolves to (statically computed with block_stack)
pub struct ResolvedLabel {
    pub label: Label,
    pub location: Location,
    // for calling end() hooks of all intermediate blocks at runtime
    #[serde(rename = "ends")]
    pub end_blocks: Vec<BlockStackElement>,
}

#[derive(Serialize)]
pub struct Location(pub Idx<Function>, pub Idx<Instr>);

// space optimization when serializing: save block stack elements as tuples, not objects with properties
impl Serialize for BlockStackElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        use self::BlockStackElement::*;
        match self {
            Function { end } => ("function", -1, end).serialize(serializer),
            Block { begin, end } => ("block", begin, end).serialize(serializer),
            Loop { begin, end } => ("loop", begin, end).serialize(serializer),
            If { begin_if, end, .. } => ("if", begin_if, end).serialize(serializer),
            Else {
                begin_else,
                end,
                begin_if,
            } => ("else", begin_else, end, begin_if).serialize(serializer),
        }
    }
}
