use ast::*;
use leb128::Leb128;
use std::fmt::{self, Write};
use std::io;
use utils::IndentExt;

#[allow(dead_code)]
impl Module {
    pub fn wat(&self) -> io::Result<String> {
        use binary::WasmBinary;
        use tempfile::NamedTempFile;
        use std::process::Command;

        let mut tmpfile = NamedTempFile::new().unwrap();
        self.encode(&mut tmpfile).unwrap();

        let output = Command::new("wasm2wat")
            .arg(tmpfile.path().as_os_str())
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();
        if !stderr.is_empty() {
            Err(io::Error::new(io::ErrorKind::InvalidData, stderr))
        } else {
            Ok(stdout)
        }
    }

    pub fn display(&self) -> String {
        let mut buf = String::new();
        self.write(&mut buf).expect("write to String should succeed");
        buf
    }
}

trait WasmDisplay {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result;
}

impl WasmDisplay for Module {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_str("Module:")?;
        self.sections.write(w)
    }
}

impl WasmDisplay for Section {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        match *self {
            Section::Custom(ref bytes) => {
                w.write_str("custom section: ")?;
                bytes.write(w)
            }
            Section::Type(ref func_types) => {
                w.write_str("type section:")?;
                write_enumerated(func_types, w)
            }
            Section::Import(ref imports) => {
                w.write_str("import section:")?;
                imports.write(w)
            }
            Section::Function(ref funcs) => {
                w.write_str("function section:")?;
                write_enumerated(funcs, w)
            }
            Section::Table(ref tables) => {
                w.write_str("table section:")?;
                write_enumerated(tables, w)
            }
            Section::Memory(ref mems) => {
                w.write_str("memory section:")?;
                write_enumerated(mems, w)
            }
            Section::Global(ref globals) => {
                w.write_str("global section:")?;
                write_enumerated(globals, w)
            }
            Section::Export(ref exports) => {
                w.write_str("export section:")?;
                exports.write(w)
            }
            Section::Start(ref func_idx) => {
                w.write_str("start section:")?;
                let mut w = w.indent();
                w.write_char('\n')?;
                func_idx.write(&mut w)
            }
            Section::Element(ref elements) => {
                w.write_str("element section:")?;
                elements.write(w)
            }
            Section::Code(ref funcs) => {
                w.write_str("code section:")?;
                write_enumerated(funcs, w)
            }
            Section::Data(ref data) => {
                w.write_str("data section:")?;
                data.write(w)
            }
        }
    }
}

impl WasmDisplay for Function {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        let mut w = w.indent();

        w.write_str("\nlocals: ")?;
        if let Some((first, rest)) = self.locals.split_first() {
            first.write(&mut w)?;
            for local in rest {
                w.write_str("\n        ")?;
                local.write(&mut w)?;
            }
        } else {
            w.write_str("none")?;
        }

        w.write_str("\nbody: ")?;
        self.body.write(&mut w)
    }
}

impl WasmDisplay for Locals {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        self.type_.write(w)?;
        write!(w, " × {}", self.count.value)
    }
}

impl WasmDisplay for Element {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.table.write(w)?;
        let mut w = w.indent();
        w.write_str("\nat offset: ")?;
        write_const_expr(&self.offset, &mut w)?;
        w.write_str("\ninit with: ")?;
        self.init.write(&mut w)
    }
}

impl WasmDisplay for Data {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.memory.write(w)?;
        let mut w = w.indent();
        w.write_str("\nat offset: ")?;
        write_const_expr(&self.offset, &mut w)?;
        w.write_str("\ninit with: ")?;
        self.init.write(&mut w)
    }
}

impl WasmDisplay for Global {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        self.type_.write(w)?;
        w.write_str(" = ")?;
        write_const_expr(&self.init, w)
    }
}

impl WasmDisplay for Expr {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        self.0.write(w)
    }
}

/// TODO comment doc
fn write_const_expr(expr: &Expr, w: &mut fmt::Write) -> fmt::Result {
    for (i, instr) in expr.0.iter().enumerate() {
        instr.write(w)?;
        if i < expr.0.len() - 1 {
            w.write_str("; ")?;
        }
    }
    Ok(())
}


impl WasmDisplay for Instr {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        match *self {
            Instr::Unreachable => w.write_str("Unreachable"),
            Instr::Nop => w.write_str("Nop"),

            Instr::Block(ref blockty, ref expr) => {
                w.write_str("Block with type ")?;
                blockty.write(w)?;
                expr.write(w)
            }
            Instr::Loop(ref blockty, ref expr) => {
                w.write_str("Loop with type ")?;
                blockty.write(w)?;
                expr.write(w)
            }
            Instr::If(ref blockty, ref expr) => {
                w.write_str("If with type ")?;
                blockty.write(w)?;
                expr.write(w)
            }
            Instr::Else(ref expr) => {
                w.write_str("Else ")?;
                expr.write(w)
            }
            Instr::End => w.write_str("End"),

            Instr::Br(ref idx) => {
                w.write_str("Br ")?;
                idx.write(w)
            }
            Instr::BrIf(ref idx) => {
                w.write_str("BrIf ")?;
                idx.write(w)
            }
            Instr::BrTable(ref vec_idx, ref idx) => {
                w.write_str("BrTable, default: ")?;
                idx.write(w)?;
                w.write_str(" table:")?;
                vec_idx.write(w)
            }

            Instr::Return => w.write_str("Return"),
            Instr::Call(ref idx) => {
                w.write_str("Call ")?;
                idx.write(w)
            }
            Instr::CallIndirect(ref type_idx, ref table_idx) => {
                w.write_str("CallIndirect ")?;
                table_idx.write(w)?;
                w.write_str(" with ")?;
                type_idx.write(w)
            }

            Instr::Drop => w.write_str("Drop"),
            Instr::Select => w.write_str("Select"),

            Instr::GetLocal(ref idx) => {
                w.write_str("GetLocal ")?;
                idx.write(w)
            }
            Instr::SetLocal(ref idx) => {
                w.write_str("SetLocal ")?;
                idx.write(w)
            }
            Instr::TeeLocal(ref idx) => {
                w.write_str("TeeLocal ")?;
                idx.write(w)
            }
            Instr::GetGlobal(ref idx) => {
                w.write_str("GetGlobal ")?;
                idx.write(w)
            }
            Instr::SetGlobal(ref idx) => {
                w.write_str("SetGlobal ")?;
                idx.write(w)
            }

            Instr::I32Load(ref memarg) => {
                w.write_str("I32Load ")?;
                memarg.write(w)
            }
            Instr::I64Load(ref memarg) => {
                w.write_str("I64Load ")?;
                memarg.write(w)
            }
            Instr::F32Load(ref memarg) => {
                w.write_str("F32Load ")?;
                memarg.write(w)
            }
            Instr::F64Load(ref memarg) => {
                w.write_str("F64Load ")?;
                memarg.write(w)
            }
            Instr::I32Load8S(ref memarg) => {
                w.write_str("I32Load8S ")?;
                memarg.write(w)
            }
            Instr::I32Load8U(ref memarg) => {
                w.write_str("I32Load8U ")?;
                memarg.write(w)
            }
            Instr::I32Load16S(ref memarg) => {
                w.write_str("I32Load16S ")?;
                memarg.write(w)
            }
            Instr::I32Load16U(ref memarg) => {
                w.write_str("I32Load16U ")?;
                memarg.write(w)
            }
            Instr::I64Load8S(ref memarg) => {
                w.write_str("I64Load8S ")?;
                memarg.write(w)
            }
            Instr::I64Load8U(ref memarg) => {
                w.write_str("I64Load8U ")?;
                memarg.write(w)
            }
            Instr::I64Load16S(ref memarg) => {
                w.write_str("I64Load16S ")?;
                memarg.write(w)
            }
            Instr::I64Load16U(ref memarg) => {
                w.write_str("I64Load16U ")?;
                memarg.write(w)
            }
            Instr::I64Load32S(ref memarg) => {
                w.write_str("I64Load32S ")?;
                memarg.write(w)
            }
            Instr::I64Load32U(ref memarg) => {
                w.write_str("I64Load32U ")?;
                memarg.write(w)
            }
            Instr::I32Store(ref memarg) => {
                w.write_str("I32Store ")?;
                memarg.write(w)
            }
            Instr::I64Store(ref memarg) => {
                w.write_str("I64Store ")?;
                memarg.write(w)
            }
            Instr::F32Store(ref memarg) => {
                w.write_str("F32Store ")?;
                memarg.write(w)
            }
            Instr::F64Store(ref memarg) => {
                w.write_str("F64Store ")?;
                memarg.write(w)
            }
            Instr::I32Store8(ref memarg) => {
                w.write_str("I32Store8 ")?;
                memarg.write(w)
            }
            Instr::I32Store16(ref memarg) => {
                w.write_str("I32Store16 ")?;
                memarg.write(w)
            }
            Instr::I64Store8(ref memarg) => {
                w.write_str("I64Store8 ")?;
                memarg.write(w)
            }
            Instr::I64Store16(ref memarg) => {
                w.write_str("I64Store16 ")?;
                memarg.write(w)
            }
            Instr::I64Store32(ref memarg) => {
                w.write_str("I64Store32 ")?;
                memarg.write(w)
            }

            Instr::CurrentMemory(ref idx) => {
                w.write_str("CurrentMemory ")?;
                idx.write(w)
            }
            Instr::GrowMemory(ref idx) => {
                w.write_str("GrowMemory ")?;
                idx.write(w)
            }

            Instr::I32Const(ref imm) => write!(w, "I32Const {}", imm.value),
            Instr::I64Const(ref imm) => write!(w, "I64Const {}", imm.value),
            Instr::F32Const(ref imm) => write!(w, "F32Const {}", imm),
            Instr::F64Const(ref imm) => write!(w, "F64Const {}", imm),

            Instr::I32Eqz => w.write_str("I32Eqz"),
            Instr::I32Eq => w.write_str("I32Eq"),
            Instr::I32Ne => w.write_str("I32Ne"),
            Instr::I32LtS => w.write_str("I32LtS"),
            Instr::I32LtU => w.write_str("I32LtU"),
            Instr::I32GtS => w.write_str("I32GtS"),
            Instr::I32GtU => w.write_str("I32GtU"),
            Instr::I32LeS => w.write_str("I32LeS"),
            Instr::I32LeU => w.write_str("I32LeU"),
            Instr::I32GeS => w.write_str("I32GeS"),
            Instr::I32GeU => w.write_str("I32GeU"),
            Instr::I64Eqz => w.write_str("I64Eqz"),
            Instr::I64Eq => w.write_str("I64Eq"),
            Instr::I64Ne => w.write_str("I64Ne"),
            Instr::I64LtS => w.write_str("I64LtS"),
            Instr::I64LtU => w.write_str("I64LtU"),
            Instr::I64GtS => w.write_str("I64GtS"),
            Instr::I64GtU => w.write_str("I64GtU"),
            Instr::I64LeS => w.write_str("I64LeS"),
            Instr::I64LeU => w.write_str("I64LeU"),
            Instr::I64GeS => w.write_str("I64GeS"),
            Instr::I64GeU => w.write_str("I64GeU"),
            Instr::F32Eq => w.write_str("F32Eq"),
            Instr::F32Ne => w.write_str("F32Ne"),
            Instr::F32Lt => w.write_str("F32Lt"),
            Instr::F32Gt => w.write_str("F32Gt"),
            Instr::F32Le => w.write_str("F32Le"),
            Instr::F32Ge => w.write_str("F32Ge"),
            Instr::F64Eq => w.write_str("F64Eq"),
            Instr::F64Ne => w.write_str("F64Ne"),
            Instr::F64Lt => w.write_str("F64Lt"),
            Instr::F64Gt => w.write_str("F64Gt"),
            Instr::F64Le => w.write_str("F64Le"),
            Instr::F64Ge => w.write_str("F64Ge"),
            Instr::I32Clz => w.write_str("I32Clz"),
            Instr::I32Ctz => w.write_str("I32Ctz"),
            Instr::I32Popcnt => w.write_str("I32Popcnt"),
            Instr::I32Add => w.write_str("I32Add"),
            Instr::I32Sub => w.write_str("I32Sub"),
            Instr::I32Mul => w.write_str("I32Mul"),
            Instr::I32DivS => w.write_str("I32DivS"),
            Instr::I32DivU => w.write_str("I32DivU"),
            Instr::I32RemS => w.write_str("I32RemS"),
            Instr::I32RemU => w.write_str("I32RemU"),
            Instr::I32And => w.write_str("I32And"),
            Instr::I32Or => w.write_str("I32Or"),
            Instr::I32Xor => w.write_str("I32Xor"),
            Instr::I32Shl => w.write_str("I32Shl"),
            Instr::I32ShrS => w.write_str("I32ShrS"),
            Instr::I32ShrU => w.write_str("I32ShrU"),
            Instr::I32Rotl => w.write_str("I32Rotl"),
            Instr::I32Rotr => w.write_str("I32Rotr"),
            Instr::I64Clz => w.write_str("I64Clz"),
            Instr::I64Ctz => w.write_str("I64Ctz"),
            Instr::I64Popcnt => w.write_str("I64Popcnt"),
            Instr::I64Add => w.write_str("I64Add"),
            Instr::I64Sub => w.write_str("I64Sub"),
            Instr::I64Mul => w.write_str("I64Mul"),
            Instr::I64DivS => w.write_str("I64DivS"),
            Instr::I64DivU => w.write_str("I64DivU"),
            Instr::I64RemS => w.write_str("I64RemS"),
            Instr::I64RemU => w.write_str("I64RemU"),
            Instr::I64And => w.write_str("I64And"),
            Instr::I64Or => w.write_str("I64Or"),
            Instr::I64Xor => w.write_str("I64Xor"),
            Instr::I64Shl => w.write_str("I64Shl"),
            Instr::I64ShrS => w.write_str("I64ShrS"),
            Instr::I64ShrU => w.write_str("I64ShrU"),
            Instr::I64Rotl => w.write_str("I64Rotl"),
            Instr::I64Rotr => w.write_str("I64Rotr"),
            Instr::F32Abs => w.write_str("F32Abs"),
            Instr::F32Neg => w.write_str("F32Neg"),
            Instr::F32Ceil => w.write_str("F32Ceil"),
            Instr::F32Floor => w.write_str("F32Floor"),
            Instr::F32Trunc => w.write_str("F32Trunc"),
            Instr::F32Nearest => w.write_str("F32Nearest"),
            Instr::F32Sqrt => w.write_str("F32Sqrt"),
            Instr::F32Add => w.write_str("F32Add"),
            Instr::F32Sub => w.write_str("F32Sub"),
            Instr::F32Mul => w.write_str("F32Mul"),
            Instr::F32Div => w.write_str("F32Div"),
            Instr::F32Min => w.write_str("F32Min"),
            Instr::F32Max => w.write_str("F32Max"),
            Instr::F32Copysign => w.write_str("F32Copysign"),
            Instr::F64Abs => w.write_str("F64Abs"),
            Instr::F64Neg => w.write_str("F64Neg"),
            Instr::F64Ceil => w.write_str("F64Ceil"),
            Instr::F64Floor => w.write_str("F64Floor"),
            Instr::F64Trunc => w.write_str("F64Trunc"),
            Instr::F64Nearest => w.write_str("F64Nearest"),
            Instr::F64Sqrt => w.write_str("F64Sqrt"),
            Instr::F64Add => w.write_str("F64Add"),
            Instr::F64Sub => w.write_str("F64Sub"),
            Instr::F64Mul => w.write_str("F64Mul"),
            Instr::F64Div => w.write_str("F64Div"),
            Instr::F64Min => w.write_str("F64Min"),
            Instr::F64Max => w.write_str("F64Max"),
            Instr::F64Copysign => w.write_str("F64Copysign"),
            Instr::I32WrapI64 => w.write_str("I32WrapI64"),
            Instr::I32TruncSF32 => w.write_str("I32TruncSF32"),
            Instr::I32TruncUF32 => w.write_str("I32TruncUF32"),
            Instr::I32TruncSF64 => w.write_str("I32TruncSF64"),
            Instr::I32TruncUF64 => w.write_str("I32TruncUF64"),
            Instr::I64ExtendSI32 => w.write_str("I64ExtendSI32"),
            Instr::I64ExtendUI32 => w.write_str("I64ExtendUI32"),
            Instr::I64TruncSF32 => w.write_str("I64TruncSF32"),
            Instr::I64TruncUF32 => w.write_str("I64TruncUF32"),
            Instr::I64TruncSF64 => w.write_str("I64TruncSF64"),
            Instr::I64TruncUF64 => w.write_str("I64TruncUF64"),
            Instr::F32ConvertSI32 => w.write_str("F32ConvertSI32"),
            Instr::F32ConvertUI32 => w.write_str("F32ConvertUI32"),
            Instr::F32ConvertSI64 => w.write_str("F32ConvertSI64"),
            Instr::F32ConvertUI64 => w.write_str("F32ConvertUI64"),
            Instr::F32DemoteF64 => w.write_str("F32DemoteF64"),
            Instr::F64ConvertSI32 => w.write_str("F64ConvertSI32"),
            Instr::F64ConvertUI32 => w.write_str("F64ConvertUI32"),
            Instr::F64ConvertSI64 => w.write_str("F64ConvertSI64"),
            Instr::F64ConvertUI64 => w.write_str("F64ConvertUI64"),
            Instr::F64PromoteF32 => w.write_str("F64PromoteF32"),
            Instr::I32ReinterpretF32 => w.write_str("I32ReinterpretF32"),
            Instr::I64ReinterpretF64 => w.write_str("I64ReinterpretF64"),
            Instr::F32ReinterpretI32 => w.write_str("F32ReinterpretI32"),
            Instr::F64ReinterpretI64 => w.write_str("F64ReinterpretI64"),
        }
    }
}

impl WasmDisplay for Memarg {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "alignment: {}, offset: {}", self.alignment.value, self.offset.value)
    }
}

impl WasmDisplay for ValType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_str(match *self {
            ValType::I32 => "i32",
            ValType::I64 => "i64",
            ValType::F32 => "f32",
            ValType::F64 => "f64",
        })
    }
}

impl WasmDisplay for FuncType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_char('[')?;
        for (i, param) in self.params.iter().enumerate() {
            param.write(w)?;
            if i < self.params.len() - 1 {
                w.write_str(", ")?;
            }
        }
        w.write_str("] -> [")?;
        for (i, result) in self.results.iter().enumerate() {
            result.write(w)?;
            if i < self.results.len() - 1 {
                w.write_str(", ")?;
            }
        }
        w.write_char(']')
    }
}

impl WasmDisplay for BlockType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        match self.0 {
            None => w.write_str("[]"),
            Some(ref ty) => {
                w.write_char('[')?;
                ty.write(w)?;
                w.write_char(']')
            }
        }
    }
}

impl WasmDisplay for Import {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "{:50} ", self.module.value.clone() + "." + &self.name.value)?;
        self.type_.write(w)
    }
}

impl WasmDisplay for ImportType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        match *self {
            ImportType::Function(ref type_) => type_.write(w),
            ImportType::Table(ref table) => table.write(w),
            ImportType::Memory(ref memory) => memory.write(w),
            ImportType::Global(ref global) => global.write(w),
        }
    }
}

impl WasmDisplay for Export {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "{:50} ", self.name.value)?;
        self.type_.write(w)
    }
}

impl WasmDisplay for ExportType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        match *self {
            ExportType::Function(ref type_) => type_.write(w),
            ExportType::Table(ref table) => table.write(w),
            ExportType::Memory(ref memory) => memory.write(w),
            ExportType::Global(ref global) => global.write(w),
        }
    }
}

impl WasmDisplay for TypeIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "type #{}", self.0.value)
    }
}

impl WasmDisplay for FunctionIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "function #{}", self.0.value)
    }
}

impl WasmDisplay for TableIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "table #{}", self.0.value)
    }
}

impl WasmDisplay for MemoryIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "memory #{}", self.0.value)
    }
}

impl WasmDisplay for GlobalIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "global #{}", self.0.value)
    }
}

impl WasmDisplay for LocalIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "local #{}", self.0.value)
    }
}

impl WasmDisplay for LabelIdx {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "label #{}", self.0.value)
    }
}

impl WasmDisplay for TableType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_str("table  ")?;
        self.1.write(w)
    }
}

impl WasmDisplay for MemoryType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_str("memory ")?;
        self.0.write(w)
    }
}

impl WasmDisplay for Limits {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        write!(w, "of size: (initial: {}, max: ", self.initial_size.value)?;
        if let Some(ref max) = self.max_size {
            write!(w, "{})", max.value)
        } else {
            w.write_str("unlimited)")
        }
    }
}

impl WasmDisplay for GlobalType {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_str("global ")?;
        match self.1 {
            Mut::Const => w.write_str("const "),
            Mut::Var => w.write_str("mut "),
        }?;
        self.0.write(w)
    }
}

/* Generic "AST combinators" */

impl<T: WasmDisplay> WasmDisplay for Leb128<T> {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        self.value.write(w)
    }
}

impl<T: WasmDisplay> WasmDisplay for WithSize<T> {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        self.content.write(w)
    }
}

impl<T: WasmDisplay> WasmDisplay for Vec<T> {
    default fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        let mut w = w.indent();
        for element in self {
            w.write_char('\n')?;
            element.write(&mut w)?;
        }
        Ok(())
    }
}

/// Specialization to display Vec<u8> on a single line and as hex.
impl WasmDisplay for Vec<u8> {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        w.write_char('[')?;
        for (i, byte) in self.iter().enumerate() {
            write!(w, "0x{:02x}", byte)?;
            if i < self.len() - 1 {
                w.write_str(", ")?;
            }
        }
        w.write_char(']')
    }
}

/// Helper for displaying Vecs with indices.
fn write_enumerated<T: WasmDisplay>(vec: &Vec<T>, mut w: &mut fmt::Write) -> fmt::Result {
    let mut w = w.indent();
    for (i, element) in vec.iter().enumerate() {
        write!(&mut w, "\n#{:<4} ", i)?;
        element.write(&mut w)?;
    }
    Ok(())
}
