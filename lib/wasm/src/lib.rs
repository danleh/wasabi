#![feature(attr_literals, specialization, core_intrinsics, test)]

#[macro_use]
extern crate binary_derive;
extern crate byteorder;
#[macro_use]
extern crate derive_new;
extern crate leb128;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate test;
extern crate test_utilities;

pub mod ast;
mod binary;
pub use self::binary::WasmBinary;

#[cfg(test)]
mod tests;

/*
 * convenience for working files (which is the most common io::Read/Write anyway)
 */

use ast::{highlevel, lowlevel};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

impl lowlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::decode(&mut BufReader::new(File::open(path)?))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        self.encode(&mut BufWriter::new(File::create(path)?))
    }
}

impl highlevel::Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(lowlevel::Module::from_file(path)?.into())
    }

    pub fn to_file<P: AsRef<Path>>(self, path: P) -> io::Result<usize> {
        let module: lowlevel::Module = self.into();
        module.to_file(path)
    }
}
