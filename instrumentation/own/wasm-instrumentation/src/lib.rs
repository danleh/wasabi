#![feature(attr_literals, specialization, universal_impl_trait, conservative_impl_trait, test)]
#![allow(dead_code, unused_variables, unused_imports)]

#[macro_use]
extern crate custom_derive;
extern crate byteorder;
extern crate rayon;
extern crate test;
extern crate walkdir;
extern crate typed_arena;

use ast::lowlevel::Module;
use binary::WasmBinary;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

mod leb128;
mod ast;
mod binary;
mod instrument;
#[cfg(test)]
mod tests;

/// convenience for working files (which is the most common io::Read/Write anyway).
impl Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::decode(&mut BufReader::new(File::open(path)?))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        self.encode(&mut BufWriter::new(File::create(path)?))
    }
}
