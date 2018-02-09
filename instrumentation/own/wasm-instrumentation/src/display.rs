use std::fmt::{Display, Formatter, Result};
use ast::{Module, Section};

impl Display for Module {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_str("Module\n")
        // TODO implement
        // don't show Leb128 and WithSize
        // indent each section, vector etc.
        // but do not indent indices and func types and other short elements
    }
}
