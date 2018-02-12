use ast::Module;
use binary::WasmBinary;
use std::io;
//use std::fmt::{Display, Error, Formatter, Result};
use std::process::Command;
use tempfile::NamedTempFile;
use serde_json;

impl Module {
    pub fn wat(&self) -> io::Result<String> {
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

    // TODO customize JSON output as I described below for Display in general, i.e.,
    // - implement serde for Leb128 and WithSize myself
    // - use some formatting/"JSON-explorer" tool to make things nice to display, maybe there is even sth with Oppen's pretty printing approach (80 chars linewidth)
    pub fn json(&self) -> io::Result<String> {
        serde_json::to_string(&self).map_err(|err|
            io::Error::new(io::ErrorKind::InvalidData, err.to_string()))
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
