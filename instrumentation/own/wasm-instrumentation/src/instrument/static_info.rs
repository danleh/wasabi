use ast::{FunctionType, Idx, Label, ValType};
use ast::highlevel::{Function, Instr, Module};
use super::block_stack::BlockStack;

#[derive(Serialize)]
pub struct ModuleInfo {
    pub functions: Vec<FunctionInfo>,
    pub globals: Vec<ValType>,
    #[serde(rename = "tableExportName")]
    pub table_export_name: String,
    #[serde(rename = "brTables")]
    pub br_tables: Vec<BrTableInfo>,
}

impl<'a> From<&'a Module> for ModuleInfo {
    fn from(module: &Module) -> Self {
        ModuleInfo {
            functions: module.functions.iter().map(Into::into).collect(),
            globals: module.globals.iter().map(|g| g.type_.0).collect(),
// FIXME table evaluation/constexpr eval
//            table: module.tables.iter()
//                .flat_map(|table| table.elements.clone())
//                .map(|element| {
//                    eprintln!("{:?}", element.offset);
//                    eprintln!("{:?}", module.eval_const_expr(&element.offset));
//                    0usize
//                }).collect(),
            // FIXME what happens if the module has no table?
            table_export_name: module.tables.get(0).and_then(|table| table.export.clone()).unwrap_or("".into()),
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
pub struct BrTableInfo {
    pub table: Vec<ResolvedLabel>,
    pub default: ResolvedLabel,
}

impl BrTableInfo {
    /// needs block_stack for resolving the labels to actual locations
    pub fn from_br_table(table: &[Idx<Label>], default: Idx<Label>, block_stack: &BlockStack, func: Idx<Function>) -> Self {
        let resolve = |label: Idx<Label>| {
            ResolvedLabel { label, location: Location { func, instr: block_stack.br_target(label) } }
        };
        BrTableInfo {
            table: table.iter().cloned().map(resolve).collect(),
            default: resolve(default),
        }
    }
}

#[derive(Serialize)]
pub struct ResolvedLabel {
    pub label: Idx<Label>,
    pub location: Location,
}

#[derive(Serialize)]
pub struct Location {
    pub func: Idx<Function>,
    pub instr: Idx<Instr>,
}