use ast::{Data, Element, Export, ExportType, Expr, Func, FuncIdx, FuncType, Global, GlobalIdx, GlobalType, Import, ImportType, LabelIdx, Limits, LocalIdx, Locals, MemoryIdx, MemoryType, Module, Mut, Section, TableIdx, TableType, TypeIdx, ValType, WithSize};
use leb128::Leb128;
use std::fmt::{self, Write};
use std::io;
use utils::IndentExt;

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
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_str("Module:")?;
        self.sections.write(&mut w)
    }
}

impl WasmDisplay for Section {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        match *self {
            Section::Custom(ref bytes) => {
                w.write_str("custom section: ")?;
                bytes.write(&mut w)
            }
            Section::Type(ref func_types) => {
                w.write_str("type section:")?;
                write_enumerated(func_types, &mut w)
            }
            Section::Import(ref imports) => {
                w.write_str("import section:")?;
                imports.write(&mut w)
            }
            Section::Function(ref funcs) => {
                w.write_str("function section:")?;
                write_enumerated(funcs, &mut w)
            }
            Section::Table(ref tables) => {
                w.write_str("table section:")?;
                write_enumerated(tables, &mut w)
            }
            Section::Memory(ref mems) => {
                w.write_str("memory section:")?;
                write_enumerated(mems, &mut w)
            }
            Section::Global(ref globals) => {
                w.write_str("global section:")?;
                write_enumerated(globals, &mut w)
            }
            Section::Export(ref exports) => {
                w.write_str("export section:")?;
                exports.write(&mut w)
            }
            Section::Start(ref func_idx) => {
                w.write_str("start section:")?;
                {
                    let mut w = w.indent();
                    func_idx.write(&mut w)?;
                    w.write_char('\n')
                }
            }
            Section::Element(ref elements) => {
                w.write_str("element section:")?;
                elements.write(&mut w)
            }
            Section::Code(ref funcs) => {
                w.write_str("code section:")?;
                write_enumerated(funcs, &mut w)
            }
            Section::Data(ref data) => {
                w.write_str("data section:")?;
                data.write(&mut w)
            }
        }
    }
}

impl WasmDisplay for Func {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        let mut w = w.indent();

        w.write_str("\nlocals:")?;
        if self.locals.is_empty() {
            w.write_str(" none")?;
        } else {
            self.locals.write(&mut w)?;
        }

        w.write_str("\nbody: ")?;
        self.body.write(&mut w)
    }
}

impl WasmDisplay for Locals {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.type_.write(&mut w)?;
        write!(&mut w, " × {}", self.count.value)
    }
}

impl WasmDisplay for Element {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.table.write(&mut w)?;
        let mut w = w.indent();
        w.write_str("\nat offset: ")?;
        self.offset.write(&mut w)?;
        w.write_str("\ninit with:")?;
        self.init.write(&mut w)
    }
}

impl WasmDisplay for Data {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.memory.write(&mut w)?;
        let mut w = w.indent();
        w.write_str("\nat offset: ")?;
        self.offset.write(&mut w)?;
        w.write_str("\ninit with: ")?;
        self.init.write(&mut w)
    }
}

impl WasmDisplay for Global {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        self.type_.write(&mut w)?;
        w.write_str(" = ")?;
        self.init.write(&mut w)
    }
}

impl WasmDisplay for Expr {
    fn write(&self, w: &mut fmt::Write) -> fmt::Result {
        // FIXME
        w.write_str("TODO")
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
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_char('[')?;
        for (i, param) in self.params.iter().enumerate() {
            param.write(&mut w)?;
            if i + 2 <= self.params.len() {
                w.write_str(", ")?;
            }
        }
        w.write_str("] -> [")?;
        for (i, result) in self.results.iter().enumerate() {
            result.write(&mut w)?;
            if i + 2 <= self.results.len() {
                w.write_str(", ")?;
            }
        }
        w.write_char(']')
    }
}

impl WasmDisplay for Import {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "{:30} ", self.module.value.clone() + "." + &self.name.value)?;
        self.type_.write(&mut w)
    }
}

impl WasmDisplay for ImportType {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        match *self {
            ImportType::Function(ref type_) => type_.write(&mut w),
            ImportType::Table(ref table) => table.write(&mut w),
            ImportType::Memory(ref memory) => memory.write(&mut w),
            ImportType::Global(ref global) => global.write(&mut w),
        }
    }
}

impl WasmDisplay for Export {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "{:30} ", self.name.value)?;
        self.type_.write(&mut w)
    }
}

impl WasmDisplay for ExportType {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        match *self {
            ExportType::Function(ref type_) => type_.write(&mut w),
            ExportType::Table(ref table) => table.write(&mut w),
            ExportType::Memory(ref memory) => memory.write(&mut w),
            ExportType::Global(ref global) => global.write(&mut w),
        }
    }
}

impl WasmDisplay for TypeIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "type #{}", self.0.value)
    }
}

impl WasmDisplay for FuncIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "function #{}", self.0.value)
    }
}

impl WasmDisplay for TableIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "table #{}", self.0.value)
    }
}

impl WasmDisplay for MemoryIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "memory #{}", self.0.value)
    }
}

impl WasmDisplay for GlobalIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "global #{}", self.0.value)
    }
}

impl WasmDisplay for LocalIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "local #{}", self.0.value)
    }
}

impl WasmDisplay for LabelIdx {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        write!(&mut w, "label #{}", self.0.value)
    }
}

impl WasmDisplay for TableType {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_str("table ")?;
        self.1.write(&mut w)
    }
}

impl WasmDisplay for MemoryType {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_str("memory ")?;
        self.0.write(&mut w)
    }
}

impl WasmDisplay for Limits {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        match *self {
            Limits::Min(ref min) => write!(&mut w, "[{}, ∞)", min.value),
            Limits::MinMax(ref min, ref max) => write!(&mut w, "[{}, {})", min.value, max.value),
        }
    }
}

impl WasmDisplay for GlobalType {
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_str("global ")?;
        match self.1 {
            Mut::Const => w.write_str("const "),
            Mut::Var => w.write_str("mut "),
        }?;
        self.0.write(&mut w)
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
    fn write(&self, mut w: &mut fmt::Write) -> fmt::Result {
        w.write_char('[')?;
        for (i, byte) in self.iter().enumerate() {
            write!(&mut w, "0x{:02x}", byte)?;
            if i + 2 <= self.len() { // FIXME why +2??
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
