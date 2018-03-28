use ast::FunctionType;
use ast::ValType;
use ast::highlevel::Function;

impl Function {
    pub fn to_info(&self) -> FunctionInfo {
        FunctionInfo {
            type_: self.type_.clone(),
            import: self.import.clone(),
            export: self.export.clone(),
            locals: self.code.iter().flat_map(|code| code.locals.clone()).collect()
        }
    }
}

#[derive(Serialize, Default)]
pub struct StaticInfo {
    pub functions: Vec<FunctionInfo>,
    pub table_export_name: String,
    pub br_tables: Vec<BrTableInfo>,
}

#[derive(Serialize)]
pub struct FunctionInfo {
    #[serde(rename = "type")]
    pub type_: FunctionType,
    pub import: Option<(String, String)>,
    pub export: Option<String>,
    pub locals: Vec<ValType>,
}

#[derive(Serialize)]
pub struct BrTableInfo {
    pub table: Vec<LabelAndLocation>,
    pub default: LabelAndLocation,
}

#[derive(Serialize)]
pub struct LabelAndLocation {
    pub label: usize,
    pub location: Location,
}

#[derive(Serialize)]
pub struct Location {
    pub func: usize,
    pub instr: usize,
}