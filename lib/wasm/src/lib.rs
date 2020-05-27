// Export AST types directly under crate, without ast prefix.
mod ast;
pub use crate::ast::*;

// Export WasmBinary trait directly under the crate.
mod binary;
pub use crate::binary::WasmBinary;

// Export Error and ErrorKind directly under the crate.
mod error;
pub use crate::error::{Error, ErrorKind};

#[cfg(test)]
mod tests;

/*
 * Convenience for working files (which is the most common io::Read/Write anyway).
 */

use crate::error::AddErrInfo;
use crate::binary::DecodeState;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

impl lowlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut state = DecodeState::new();
        Self::decode(&mut BufReader::new(File::open(path).add_err_info::<File>(0)?), &mut state)
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        self.encode(&mut BufWriter::new(File::create(path)?))
    }
}

impl highlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(lowlevel::Module::from_file(path)?.into())
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        let module: lowlevel::Module = self.into();
        module.to_file(path)
    }
}
