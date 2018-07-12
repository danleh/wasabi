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

#[derive(Serialize)]
pub struct FunctionInfo {
    #[serde(rename = "type")]
    pub type_: FunctionType,
    pub import: Option<(String, String)>,
    pub export: Option<String>,
    pub locals: Vec<ValType>,
    #[serde(rename = "instrCount")]
    /// this is the only field that differentiates FunctionInfo from a plain Function:
    /// we do not want to save all instructions in the object, this seems a bit excessive...
    pub instr_count: Option<usize>,
}

impl<'a> From<&'a Function> for FunctionInfo {
    fn from(function: &Function) -> FunctionInfo {
        FunctionInfo {
            type_: function.type_.clone(),
            import: function.import.clone(),
            export: function.export.clone(),
            locals: function.code.iter().flat_map(|code| code.locals.clone()).collect(),
            instr_count: function.code.as_ref().map(|code| code.body.len()),
        }
    }
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
                location: Location { func, instr: target.absolute_instr },
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
    #[serde(rename = "endBlocks")]
    pub end_blocks: Vec<BlockStackElement>,
}

#[derive(Serialize)]
pub struct Location {
    pub func: Idx<Function>,
    pub instr: Idx<Instr>,
}