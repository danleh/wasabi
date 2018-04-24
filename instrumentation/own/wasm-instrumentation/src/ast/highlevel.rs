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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Nop,
    Unreachable,

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

    MemorySize(Idx<Memory>),
    MemoryGrow(Idx<Memory>),

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

    // FIXME untested, possibly broken
    pub fn eval_const_expr(&self, expr: &Expr) -> Instr {
        use self::Instr::*;
        if let &[ref instr, End] = expr.as_slice() {
            match *instr {
                I32Const(_) | I64Const(_) | F32Const(_) | F64Const(_) => return instr.clone(),
                GetGlobal(idx) => return self.eval_const_expr(self.globals[idx.0].init.as_ref().unwrap()),
                _ => {}
            }
        }
        unimplemented!("currently only constant expressions with a single T.const or get_global instruction are supported, but got: {:?}", expr);
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

    /// add a new local with type ty and return its index
    pub fn add_fresh_local(&mut self, ty: ValType) -> Idx<Local> {
        let locals = &mut self.code.as_mut()
            .expect("cannot add local to imported function")
            .locals;
        let idx = locals.len() + self.type_.params.len();
        locals.push(ty);
        idx.into()
    }

    pub fn add_fresh_locals(&mut self, tys: &[ValType]) -> Vec<Idx<Local>> {
        tys.iter()
            .map(|ty| self.add_fresh_local(*ty))
            .collect()
    }

    /// get type of the local with index idx
    pub fn local_type(&self, idx: Idx<Local>) -> ValType {
        let param_count = self.type_.params.len();
        if (idx.0) < param_count {
            self.type_.params[idx.0]
        } else {
            let locals = &self.code.as_ref()
                .expect("cannot get type of a local in an imported function")
                .locals;
            *locals.get(idx.0 - param_count)
                .expect(&format!("invalid local index {}, function has {} parameters and {} locals", idx.0, param_count, locals.len()))
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum InstrGroup {
    Const(ValType),
    Numeric {
        input_tys: Vec<ValType>,
        result_tys: Vec<ValType>,
    },
    Memory(MemoryOp, ValType, Memarg),
    Other,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum MemoryOp {
    Load,
    Store,
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

            I32Eqz => Numeric { input_tys: vec![I32], result_tys: vec![I32] },
            I64Eqz => Numeric { input_tys: vec![I64], result_tys: vec![I32] },

            I32Clz | I32Ctz | I32Popcnt => Numeric { input_tys: vec![I32], result_tys: vec![I32] },
            I64Clz | I64Ctz | I64Popcnt => Numeric { input_tys: vec![I64], result_tys: vec![I64] },

            F32Abs | F32Neg | F32Ceil | F32Floor | F32Trunc | F32Nearest | F32Sqrt => Numeric { input_tys: vec![F32], result_tys: vec![F32] },
            F64Abs | F64Neg | F64Ceil | F64Floor | F64Trunc | F64Nearest | F64Sqrt => Numeric { input_tys: vec![F64], result_tys: vec![F64] },

            // conversions
            I32WrapI64 => Numeric { input_tys: vec![I64], result_tys: vec![I32] },
            I32TruncSF32 | I32TruncUF32 => Numeric { input_tys: vec![I32], result_tys: vec![F32] },
            I32TruncSF64 | I32TruncUF64 => Numeric { input_tys: vec![F64], result_tys: vec![I32] },
            I64ExtendSI32 | I64ExtendUI32 => Numeric { input_tys: vec![I32], result_tys: vec![I64] },
            I64TruncSF32 | I64TruncUF32 => Numeric { input_tys: vec![F32], result_tys: vec![I64] },
            I64TruncSF64 | I64TruncUF64 => Numeric { input_tys: vec![F64], result_tys: vec![I64] },
            F32ConvertSI32 | F32ConvertUI32 => Numeric { input_tys: vec![I32], result_tys: vec![F32] },
            F32ConvertSI64 | F32ConvertUI64 => Numeric { input_tys: vec![I64], result_tys: vec![F32] },
            F32DemoteF64 => Numeric { input_tys: vec![F64], result_tys: vec![F32] },
            F64ConvertSI32 | F64ConvertUI32 => Numeric { input_tys: vec![I32], result_tys: vec![F64] },
            F64ConvertSI64 | F64ConvertUI64 => Numeric { input_tys: vec![I64], result_tys: vec![F64] },
            F64PromoteF32 => Numeric { input_tys: vec![F32], result_tys: vec![F64] },
            I32ReinterpretF32 => Numeric { input_tys: vec![F32], result_tys: vec![I32] },
            I64ReinterpretF64 => Numeric { input_tys: vec![F64], result_tys: vec![I64] },
            F32ReinterpretI32 => Numeric { input_tys: vec![I32], result_tys: vec![F32] },
            F64ReinterpretI64 => Numeric { input_tys: vec![I64], result_tys: vec![F64] },

            /* Binary */

            I32Eq | I32Ne | I32LtS | I32LtU | I32GtS | I32GtU | I32LeS | I32LeU | I32GeS | I32GeU => Numeric { input_tys: vec![I32, I32], result_tys: vec![I32] },
            I64Eq | I64Ne | I64LtS | I64LtU | I64GtS | I64GtU | I64LeS | I64LeU | I64GeS | I64GeU => Numeric { input_tys: vec![I64, I64], result_tys: vec![I32] },

            F32Eq | F32Ne | F32Lt | F32Gt | F32Le | F32Ge => Numeric { input_tys: vec![F32, F32], result_tys: vec![I32] },
            F64Eq | F64Ne | F64Lt | F64Gt | F64Le | F64Ge => Numeric { input_tys: vec![F64, F64], result_tys: vec![I32] },

            I32Add | I32Sub | I32Mul | I32DivS | I32DivU | I32RemS | I32RemU | I32And | I32Or | I32Xor | I32Shl | I32ShrS | I32ShrU | I32Rotl | I32Rotr => Numeric { input_tys: vec![I32, I32], result_tys: vec![I32] },
            I64Add | I64Sub | I64Mul | I64DivS | I64DivU | I64RemS | I64RemU | I64And | I64Or | I64Xor | I64Shl | I64ShrS | I64ShrU | I64Rotl | I64Rotr => Numeric { input_tys: vec![I64, I64], result_tys: vec![I64] },
            F32Add | F32Sub | F32Mul | F32Div | F32Min | F32Max | F32Copysign => Numeric { input_tys: vec![F32, F32], result_tys: vec![F32] },
            F64Add | F64Sub | F64Mul | F64Div | F64Min | F64Max | F64Copysign => Numeric { input_tys: vec![F64, F64], result_tys: vec![F64] },

            /* Memory */

            I32Load(memarg) => Memory(MemoryOp::Load, I32, memarg),
            I64Load(memarg) => Memory(MemoryOp::Load, I64, memarg),
            F32Load(memarg) => Memory(MemoryOp::Load, F32, memarg),
            F64Load(memarg) => Memory(MemoryOp::Load, F64, memarg),

            I32Load8S(memarg) => Memory(MemoryOp::Load, I32, memarg),
            I32Load8U(memarg) => Memory(MemoryOp::Load, I32, memarg),
            I32Load16S(memarg) => Memory(MemoryOp::Load, I32, memarg),
            I32Load16U(memarg) => Memory(MemoryOp::Load, I32, memarg),
            I64Load8S(memarg) => Memory(MemoryOp::Load, I64, memarg),
            I64Load8U(memarg) => Memory(MemoryOp::Load, I64, memarg),
            I64Load16S(memarg) => Memory(MemoryOp::Load, I64, memarg),
            I64Load16U(memarg) => Memory(MemoryOp::Load, I64, memarg),
            I64Load32S(memarg) => Memory(MemoryOp::Load, I64, memarg),
            I64Load32U(memarg) => Memory(MemoryOp::Load, I64, memarg),

            I32Store(memarg) => Memory(MemoryOp::Store, I32, memarg),
            I64Store(memarg) => Memory(MemoryOp::Store, I64, memarg),
            F32Store(memarg) => Memory(MemoryOp::Store, F32, memarg),
            F64Store(memarg) => Memory(MemoryOp::Store, F64, memarg),

            I32Store8(memarg) => Memory(MemoryOp::Store, I32, memarg),
            I32Store16(memarg) => Memory(MemoryOp::Store, I32, memarg),
            I64Store8(memarg) => Memory(MemoryOp::Store, I64, memarg),
            I64Store16(memarg) => Memory(MemoryOp::Store, I64, memarg),
            I64Store32(memarg) => Memory(MemoryOp::Store, I64, memarg),

            _ => Other,
        }
    }

    pub fn to_instr_name(&self) -> String {
        let mut s = String::new();
        // use fmt::Debug implementation to get String repr of enum variant
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
}