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

mod extensions;

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

    pub fn from_file_with_offsets_wasmparser(path: impl AsRef<Path>) -> Result<(Self, Offsets), Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        let (module, offsets, warnings) = ast::wasmparser::parse_module_with_offsets(&bytes)?;
        for warning in warnings {
            println!("warning: {}", warning);
        }
        Ok((module, offsets))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        let module: lowlevel::Module = self.into();
        module.to_file(path)
    }

    pub fn to_bytes<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        let module: lowlevel::Module = self.into();
        module.encode(writer)
    }

    pub fn to_bytes_wasmparser<W: io::Write>(&self, writer: &mut W) -> io::Result<usize> {
        todo!()
    }
}
