use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use wasabi_leb128::ParseLeb128Error;

// TODO typeful Wasm parsing errors

/// Errors while parsing a WebAssembly binary.
// TODO Can we also have errors when writing a Wasm binary?
// Or is our AST correct by construction? (I don't think so in all cases...)
#[derive(Debug)]
pub struct Error {
    /// Position of the reader when parsing failed.
    // TODO Is this before or after or at least near the position where parsing failed?
    pub offset: Option<usize>,
    /// The type of error.
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Wrong magic number, i.e., not b"\0asm".
    MagicNumber,
    /// Wrong version. We can only handle Wasm v1, i.e., the MVP.
    Version,
    /// Expected a valid tag for a grammar element (first argument), but got the second argument instead.
    Tag(&'static str, u8),
    /// A number was not valid LEB128 or could not be parsed to the target number type.
    Leb128(ParseLeb128Error),
    /// A string was not valid UTF-8.
    Utf8(FromUtf8Error),
    /// A generic I/O error, e.g., unexpected end of data.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(offset) = self.offset {
            write!(f, "error while parsing Wasm binary at offset 0x{:x} ({}): {}", offset, offset, self.kind)
        } else {
            write!(f, "error while parsing Wasm binary: {}", self.kind)
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorKind::MagicNumber => f.write_str("invalid magic number"),
            ErrorKind::Version => f.write_str("invalid version, can only parse WebAssembly v1 (MVP)"),
            ErrorKind::Tag(grammar_element, got) => write!(f, "invalid tag 0x{:02x} for {}", got, grammar_element),
            ErrorKind::Leb128(err) => write!(f, "invalid LEB128 number: {}", err),
            ErrorKind::Utf8(_err) => f.write_str("invalid string, not UTF-8"),
            ErrorKind::Io(err) => err.fmt(f)
        }
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

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        // TODO Add offset where possible: comment out this impl, then fix all errors by explicitly
        // converting to wasm::Error where possible and adding offset information.
        Error { offset: None, kind: ErrorKind::Io(e) }
    }
}

impl From<ParseLeb128Error> for Error {
    fn from(e: ParseLeb128Error) -> Self {
        Error { offset: None, kind: ErrorKind::Leb128(e) }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error { offset: None, kind: ErrorKind::Utf8(e) }
    }
}

impl Error {
    /// Convenience constructor.
    pub fn invalid_tag(grammar_element: &'static str, tag: u8) -> Self {
        Error { offset: None, kind: ErrorKind::Tag(grammar_element, tag) }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}