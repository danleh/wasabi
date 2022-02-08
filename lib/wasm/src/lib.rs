// Export AST types directly under crate, without ast prefix.
mod ast;
pub use crate::ast::*;

// Export WasmBinary trait directly under the crate.
mod binary;
pub use crate::binary::WasmBinary;
pub use crate::binary::DecodeState;

// Export Error and ErrorKind directly under the crate.
mod error;
pub use crate::error::{Error, ErrorKind};

mod types;
mod folding;
pub mod wimpl;

pub mod callgraph;

#[cfg(test)]
mod tests;

/*
 * Convenience for working files (which is the most common io::Read/Write anyway).
 */

use crate::error::AddErrInfo;
use crate::lowlevel::Offsets;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

impl lowlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Self::from_file_with_offsets(path)?.0)
    }

    pub fn from_file_with_offsets<P: AsRef<Path>>(path: P) -> Result<(Self, Offsets), Error> {
        let mut state = DecodeState::new();
        let module = Self::decode(&mut BufReader::new(File::open(path).add_err_info::<File>(0)?), &mut state)?;
        let offsets = state.into_offsets(&module);
        Ok((module, offsets))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        self.encode(&mut BufWriter::new(File::create(path)?))
    }
}

impl highlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(lowlevel::Module::from_file(path)?.into())
    }

    /// The returned offsets are given in terms of the original "low-level" sections and functions.
    pub fn from_file_with_offsets<P: AsRef<Path>>(path: P) -> Result<(Self, Offsets), Error> {
        let (module, offsets) = lowlevel::Module::from_file_with_offsets(path)?;
        Ok((module.into(), offsets))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        let module: lowlevel::Module = self.into();
        module.to_file(path)
    }
}
