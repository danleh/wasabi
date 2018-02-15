use self::lowlevel::Module;
use binary::WasmBinary;

use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

pub mod highlevel;
pub mod lowlevel;

/// convenience for working files (which is the most common io::Read/Write anyway).
impl Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::decode(&mut BufReader::new(File::open(path)?))
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<usize> {
        self.encode(&mut BufWriter::new(File::create(path)?))
    }
}