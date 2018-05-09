#![feature(attr_literals, specialization, test, core_intrinsics, from_ref)]
#![allow(dead_code)]

extern crate byteorder;
#[macro_use]
extern crate binary_derive;
#[macro_use]
extern crate derive_new;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate test;
extern crate walkdir;
extern crate leb128;

use wasm::ast::{highlevel, lowlevel};
use wasm::WasmBinary;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

pub mod wasm;
pub mod instrument;
#[cfg(test)]
mod tests;

/// convenience for working files (which is the most common io::Read/Write anyway).
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
