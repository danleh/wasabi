use std::path::Path;

use crate::error::Error;
use crate::highlevel::Module;
use crate::lowlevel::Offsets;

pub fn parse_module_with_offsets(path: impl AsRef<Path>) -> Result<(Module, Offsets), Error> {
    todo!()
}
