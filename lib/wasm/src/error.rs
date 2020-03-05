use std::error;
use std::fmt;

// TODO typeful Wasm parsing errors

/// Errors while parsing a WebAssembly binary.
// TODO Can we also have errors when writing a Wasm binary?
// Or is our AST correct by construction? (I don't think so in all cases...)
#[derive(Debug)]
pub struct Error {
    /// The type of error.
    kind: ErrorKind,
    /// Position of the reader when parsing failed.
    // TODO Is this before or after or at least near the position where parsing failed?
    offset: usize
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Wrong magic number, i.e., not b"\0asm".
    MagicNumber,
    /// Wrong version. We can only handle Wasm v1, i.e., the MVP.
    Version,
    /// Expected a valid tag for a grammar element (first argument), but got the second argument instead.
    Tag(&'static str, u8),
    /// A string was not valid UTF-8.
    Utf8(std::str::Utf8Error),
    /// A number was not valid LEB128 or could not be parsed to the target number type.
    Leb128(wasabi_leb128::ParseLeb128Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "error while parsing Wasm binary at offset 0x{:x} ({}): {}", self.offset, self.offset, self.kind)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorKind::MagicNumber => f.write_str("invalid magic number"),
            ErrorKind::Version => f.write_str("invalid version"),
            ErrorKind::Tag(grammar_element, got) =>         write!(f, "invalid tag 0x{:x} for {}", got, grammar_element),
            ErrorKind::Utf8(err) => f.write_str("invalid UTF-8 string"),
            ErrorKind::Leb128(err) => f.write_str("invalid LEB128 number"),
        }
        // TODO if verbose flag is given, also output self.source(), if it exists.
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Utf8(e) => Some(e),
            ErrorKind::Leb128(e) => Some(e),
            _ => None,
        }
    }
}

//impl From<io::Error> for ParseLeb128Error {
//    fn from(e: io::Error) -> Self {
//        ParseLeb128Error::Other(e)
//    }
//}