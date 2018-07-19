use serde::{Serialize, Serializer};
use super::block_stack::{BlockStack, BlockStackElement};
use wasm::ast::{FunctionType, Idx, Label, ValType};
use wasm::ast::highlevel::{Function, Instr, Module};

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
            table_export_name: module.tables.get(0).and_then(|table| table.export.clone()),
            br_tables: vec![],
        }
    }
}

#[derive(Clone)]
//#[derive(Serialize)]
pub struct FunctionInfo {
//    #[serde(serialize_with = "serialize_function_type")]
    pub type_: FunctionType,
    pub import: Option<(String, String)>,
    pub export: Option<String>,
//    #[serde(serialize_with = "serialize_types")]
    pub locals: Vec<ValType>,
}

impl<'a> From<&'a Function> for FunctionInfo {
    fn from(function: &Function) -> FunctionInfo {
        FunctionInfo {
            type_: function.type_.clone(),
            import: function.import.clone(),
            export: function.export.clone(),
            locals: function.code.iter().flat_map(|code| code.locals.clone()).collect(),
        }
    }
}

// optimizations to keep the generated static info small:
// - functions as tuples (= JS arrays), not objects
// - types and locals as strings
#[derive(Serialize)]
struct FunctionInfoSerialized(
    Option<(String, String)>,
    Option<String>,
    #[serde(serialize_with = "serialize_function_type")]
    FunctionType,
    #[serde(serialize_with = "serialize_types")]
    Vec<ValType>,
);

impl Serialize for FunctionInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let this = self.clone();
        FunctionInfoSerialized(this.import, this.export, this.type_, this.locals).serialize(serializer)
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
    pub table: Vec<ResolvedLabel>,
    pub default: ResolvedLabel,
}

impl BrTableInfo {
    /// needs block_stack for resolving the labels to actual locations
    pub fn from_br_table(table: &[Idx<Label>], default: Idx<Label>, block_stack: &BlockStack, func: Idx<Function>) -> Self {
        let resolve = |label: Idx<Label>| {
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
    pub label: Idx<Label>,
    pub location: Location,
    // for calling end() hooks of all intermediate blocks at runtime
    #[serde(rename = "ends")]
    pub end_blocks: Vec<BlockStackElement>,
}

#[derive(Serialize)]
pub struct Location(pub Idx<Function>, pub Idx<Instr>);

// space optimization when serializing block ends again
impl Serialize for BlockStackElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use self::BlockStackElement::*;
        match self {
            Function { end } => ("function", -1, end).serialize(serializer),
            Block { begin, end } => ("block", begin, end).serialize(serializer),
            Loop { begin, end } => ("loop", begin, end).serialize(serializer),
            If { begin_if, end, begin_else } => ("if", begin_if, end, begin_else).serialize(serializer),
            Else { begin_else, end, begin_if } => ("else", begin_else, end, begin_if).serialize(serializer),
        }
    }
}