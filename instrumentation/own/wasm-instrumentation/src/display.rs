use ast::Module;
use binary::WasmBinary;
//use std::fmt::{Display, Error, Formatter, Result};
use std::process::Command;
use tempfile::NamedTempFile;

impl Module {
    pub fn wat(&self) -> String {
        let mut tmpfile = NamedTempFile::new().unwrap();
        self.encode(&mut tmpfile).unwrap();

        let output = Command::new("wasm2wat")
            .arg(tmpfile.path().as_os_str())
            .output()
            .unwrap();
        let mut buffer = String::from_utf8(output.stdout).unwrap();
        buffer.push_str(&String::from_utf8(output.stderr).unwrap());

        buffer
    }
}

// TODO implement proper own display trait:
// - wat is too high-level (doesn't show empty sections etc)
// - fmt::Debug is too low-level/annyoing (shows Leb128 and WithSize, always nested)
// don't show Leb128 and WithSize
// indent each section, vector etc.
// but do not indent indices and func types and other short elements

//impl Display for Module {
//    fn fmt(&self, f: &mut Formatter) -> Result {
//    }
//}
