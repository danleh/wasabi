use std::collections::HashSet;
use std::iter::empty;
use std::slice::Iter;
use super::*;
use super::ValType::*;

/* High-level AST:
    - types are inlined instead of referenced by type idx (i.e., no manual handling of Type "pool")
    - Function + Code sections are merged into one list of functions,
      same for tables: Table + Element sections and memories: Memory + Data sections.
    - imports and exports are part of the respective item, not stored externally and referring to
      their item by index.
*/

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub tables: Vec<Table>,
    pub memories: Vec<Memory>,
    pub globals: Vec<Global>,

    pub start: Option<Idx<Function>>,

    pub custom_sections: Vec<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Function {
    // type is inlined here compared to low-level/binary/spec representation
    pub type_: FunctionType,
    // import and code are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub code: Option<Code>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Global {
    pub type_: GlobalType,
    // import and init are mutually exclusive, i.e., exactly one of both must be Some(...)
    pub import: Option<(String, String)>,
    pub init: Option<Expr>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub type_: TableType,
    pub import: Option<(String, String)>,
    pub elements: Vec<Element>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Memory {
    pub type_: MemoryType,
    pub import: Option<(String, String)>,
    pub data: Vec<Data>,
    pub export: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub locals: Vec<ValType>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub struct Element {
    pub offset: Expr,
    pub functions: Vec<Idx<Function>>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub offset: Expr,
    pub bytes: Vec<u8>,
}

pub type Expr = Vec<Instr>;

#[derive(Debug, Clone)]
pub enum Instr {
    Unreachable,
    Nop,

    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,

    Br(Idx<Label>),
    BrIf(Idx<Label>),
    BrTable(Vec<Idx<Label>>, Idx<Label>),

    Return,
    Call(Idx<Function>),
    CallIndirect(FunctionType, Idx<Table>),

    Drop,
    Select,

    GetLocal(Idx<Local>),
    SetLocal(Idx<Local>),
    TeeLocal(Idx<Local>),
    GetGlobal(Idx<Global>),
    SetGlobal(Idx<Global>),

    I32Load(Memarg),
    I64Load(Memarg),
    F32Load(Memarg),
    F64Load(Memarg),

    I32Load8S(Memarg),
    I32Load8U(Memarg),
    I32Load16S(Memarg),
    I32Load16U(Memarg),
    I64Load8S(Memarg),
    I64Load8U(Memarg),
    I64Load16S(Memarg),
    I64Load16U(Memarg),
    I64Load32S(Memarg),
    I64Load32U(Memarg),

    I32Store(Memarg),
    I64Store(Memarg),
    F32Store(Memarg),
    F64Store(Memarg),

    I32Store8(Memarg),
    I32Store16(Memarg),
    I64Store8(Memarg),
    I64Store16(Memarg),
    I64Store32(Memarg),

    CurrentMemory(Idx<Memory>),
    GrowMemory(Idx<Memory>),

    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    I32WrapI64,
    I32TruncSF32,
    I32TruncUF32,
    I32TruncSF64,
    I32TruncUF64,

    I64ExtendSI32,
    I64ExtendUI32,
    I64TruncSF32,
    I64TruncUF32,
    I64TruncSF64,
    I64TruncUF64,

    F32ConvertSI32,
    F32ConvertUI32,
    F32ConvertSI64,
    F32ConvertUI64,
    F32DemoteF64,

    F64ConvertSI32,
    F64ConvertUI32,
    F64ConvertSI64,
    F64ConvertUI64,
    F64PromoteF32,

    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
}


/* Impls/functions for typical use cases on WASM modules. */

impl Module {
    pub fn new() -> Self {
        Module {
            functions: Vec::new(),
            tables: Vec::new(),
            memories: Vec::new(),
            globals: Vec::new(),
            start: None,
            custom_sections: Vec::new(),
        }
    }

    pub fn add_function(&mut self, type_: FunctionType, locals: Vec<ValType>, body: Vec<Instr>) -> Idx<Function> {
        self.functions.push(Function {
            type_,
            import: None,
            code: Some(Code {
                locals,
                body,
            }),
            export: None,
        });
        (self.functions.len() - 1).into()
    }

    pub fn add_function_import(&mut self, type_: FunctionType, module: String, name: String) -> Idx<Function> {
        self.functions.push(Function {
            type_,
            import: Some((module, name)),
            code: None,
            export: None,
        });
        (self.functions.len() - 1).into()
    }

    pub fn add_global(&mut self, type_: ValType, mut_: Mutability, init: Vec<Instr>) -> Idx<Global> {
        self.globals.push(Global {
            type_: GlobalType(type_, mut_),
            import: None,
            init: Some(init),
            export: None,
        });
        (self.globals.len() - 1).into()
    }

    pub fn function(&mut self, idx: Idx<Function>) -> &mut Function { &mut self.functions[idx.0] }
    pub fn functions(&mut self) -> impl Iterator<Item=(Idx<Function>, &mut Function)> {
        self.functions.iter_mut().enumerate().map(|(i, f)| (i.into(), f))
    }

    pub fn types(&self) -> HashSet<&FunctionType> {
        let mut types = HashSet::new();
        for function in &self.functions {
            types.insert(&function.type_);
        }
        types
    }
}

impl Function {
    pub fn instructions(&mut self) -> impl Iterator<Item=(Idx<Instr>, &mut Instr)> {
        self.code.iter_mut().flat_map(|code| code.body.iter_mut().enumerate().map(|(i, f)| (i.into(), f)))
    }

    pub fn modify_instr(&mut self, f: impl Fn(Instr) -> Vec<Instr>) {
        if let Some(Code { ref mut body, .. }) = self.code {
            let new_body = Vec::with_capacity(body.len());
            let old_body = ::std::mem::replace(body, new_body);
            for instr in old_body.into_iter() {
                body.append(&mut f(instr));
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum InstrGroup {
    Const(ValType),
    Unary {
        input_ty: ValType,
        result_ty: ValType,
    },
    Binary {
        first_ty: ValType,
        second_ty: ValType,
        result_ty: ValType,
    },
    Other,
}

impl Instr {
    pub fn group(&self) -> InstrGroup {
        use self::{Instr::*, InstrGroup::*};
        match *self {
            /* Const */

            I32Const(_) => Const(I32),
            I64Const(_) => Const(I64),
            F32Const(_) => Const(F32),
            F64Const(_) => Const(F64),


            /* Unary */

            I32Eqz => Unary { input_ty: I32, result_ty: I32 },
            I64Eqz => Unary { input_ty: I64, result_ty: I32 },

            I32Clz | I32Ctz | I32Popcnt => Unary { input_ty: I32, result_ty: I32 },
            I64Clz | I64Ctz | I64Popcnt => Unary { input_ty: I64, result_ty: I64 },

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest => Unary { input_ty: F32, result_ty: F32 },
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest => Unary { input_ty: F64, result_ty: F64 },

            // conversions
            I32WrapI64 => Unary { input_ty: I64, result_ty: I32 },
            I32TruncSF32 | I32TruncUF32 => Unary { input_ty: I32, result_ty: F32 },
            I32TruncSF64 | I32TruncUF64 => Unary { input_ty: F64, result_ty: I32 },
            I64ExtendSI32 | I64ExtendUI32 => Unary { input_ty: I32, result_ty: I64 },
            I64TruncSF32 | I64TruncUF32 => Unary { input_ty: F32, result_ty: I64 },
            I64TruncSF64 | I64TruncUF64 => Unary { input_ty: F64, result_ty: I64 },
            F32ConvertSI32 | F32ConvertUI32 => Unary { input_ty: I32, result_ty: F32 },
            F32ConvertSI64 | F32ConvertUI64 => Unary { input_ty: I64, result_ty: F32 },
            F32DemoteF64 => Unary { input_ty: F64, result_ty: F32 },
            F64ConvertSI32 | F64ConvertUI32 => Unary { input_ty: I32, result_ty: F64 },
            F64ConvertSI64 | F64ConvertUI64 => Unary { input_ty: I64, result_ty: F64 },
            F64PromoteF32 => Unary { input_ty: F32, result_ty: F64 },
            I32ReinterpretF32 => Unary { input_ty: F32, result_ty: I32 },
            I64ReinterpretF64 => Unary { input_ty: F64, result_ty: I64 },
            F32ReinterpretI32 => Unary { input_ty: I32, result_ty: F32 },
            F64ReinterpretI64 => Unary { input_ty: I64, result_ty: F64 },


            /* Binary */

            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => Binary { first_ty: I32, second_ty: I32, result_ty: I32 },
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => Binary { first_ty: I64, second_ty: I64, result_ty: I32 },

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => Binary { first_ty: F32, second_ty: F32, result_ty: I32 },
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => Binary { first_ty: F64, second_ty: F64, result_ty: I32 },

            _ => Other,
        }
    }

    pub fn to_instr_name(&self) -> String {
        let mut s = String::new();
        write!(&mut s, "{:?}", self).unwrap();
        let mut result = String::new();
        let mut last_char = ' ';
        for c in s.split("(").next().unwrap_or("").chars() {
            if
                // do not prepend underscore at beginning of string
                !result.is_empty()
                    // prepend underscore in front of uppercase letters
                    && c.is_uppercase()
                    // or between lowercase and numerics
                    || (last_char.is_lowercase() && c.is_numeric()) {
                result.push('_');
            }
            last_char = c;
            result.push_str(&c.to_ascii_lowercase().to_string());
        }
        result
    }

    /// "generate" quick and dirty the low-level JavaScript hook function from an instruction
    pub fn to_js_hook(&self) -> String {
        fn arg(name: &str, ty: ValType) -> String {
            match ty {
                I64 => name.to_string() + "_low, " + name + "_high",
                _ => name.to_string()
            }
        }
        fn long(name: &str, ty: ValType) -> String {
            match ty {
                I64 => format!("new Long({})", arg(name, ty)),
                _ => name.to_string()
            }
        }
        match (self.group(), self, self.to_instr_name()) {
            (InstrGroup::Const(ty), instr, instr_str) => format!(
                "{}: function (func, instr, {}) {{
    const_({{func, instr}}, {});
}},",
                instr_str,
                arg("v", ty), long("v", ty)
            ),
            (InstrGroup::Unary { input_ty, result_ty }, instr, instr_str) => format!(
                "{}: function (func, instr, {}, {}) {{
    unary({{func, instr}}, \"{}\", {}, {});
}},",
                instr_str,
                arg("input", input_ty), arg("result", result_ty),
                instr_str,
                long("input", input_ty), long("result", result_ty)),
            (InstrGroup::Binary { first_ty, second_ty, result_ty }, instr, instr_str) => format!(
                "{}: function (func, instr, {}, {}, {}) {{
    binary({{func, instr}}, \"{}\", {}, {}, {});
}},",
                instr_str,
                arg("first", first_ty), arg("second", second_ty), arg("result", result_ty),
                instr_str,
                long("first", first_ty), long("second", second_ty), long("result", result_ty)),
            (_, _, instr) => unimplemented!("cannot generate JS hook code for instruction {}", instr)
        }
    }
}
