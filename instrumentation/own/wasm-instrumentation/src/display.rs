use ast::Module;
use binary::WasmBinary;
use std::fmt::{Display, Error, Formatter, Result};
use std::process::Command;
use tempfile::NamedTempFile;

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO implement proper own display trait
        // don't show Leb128 and WithSize
        // indent each section, vector etc.
        // but do not indent indices and func types and other short elements

        let mut tmpfile = NamedTempFile::new().map_err(|_| Error)?;
        self.encode(&mut tmpfile).map_err(|_| Error)?;

        let output = Command::new("wasm2wat")
            .arg(tmpfile.path().as_os_str())
            .output()
            .map_err(|_| Error)?;
        let output = String::from_utf8(output.stdout)
            .map_err(|_| Error)?;

        f.write_str(&output)
    }
}
