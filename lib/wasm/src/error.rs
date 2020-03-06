use std::error;
use std::fmt;
use std::io;
use std::string::FromUtf8Error;
use wasabi_leb128::ParseLeb128Error;

/// Errors while parsing a WebAssembly binary.
// TODO Can we also have errors when writing a Wasm binary?
// Or is our AST correct by construction? (I don't think so in all cases...)
#[derive(Debug)]
pub struct Error {
    /// Position of the reader when parsing failed.
    // TODO Is this before or after or at least near the position where parsing failed?
    // TODO remove Option, make it required
    pub offset: Option<u64>,
    /// The type of error, e.g., wrong magic number.
    pub kind: ErrorKind,
    /// The underlying cause of the error, if there was one.
    pub source: Option<Box<dyn error::Error + 'static + Send>>,
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
    Leb128,
    /// A string was not valid UTF-8.
    Utf8,
    /// An I/O error that is not specific to Wasm parsing, e.g., unexpected end of input.
    Io,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(offset) = self.offset {
            write!(f, "invalid Wasm binary at offset 0x{:x} ({}): {}", offset, offset, self.kind)
        } else {
            write!(f, "invalid Wasm binary: {}", self.kind)
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorKind::MagicNumber => f.write_str("invalid magic number"),
            ErrorKind::Version => f.write_str("invalid version, can only parse WebAssembly v1 (MVP)"),
            ErrorKind::Tag(grammar_element, got) => write!(f, "invalid tag 0x{:02x} for {}", got, grammar_element.to_ascii_lowercase()),
            ErrorKind::Leb128 => f.write_str("invalid LEB128 number"),
            ErrorKind::Utf8 => f.write_str("invalid UTF-8 string"),
            ErrorKind::Io => f.write_str("I/O error")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|err| err.as_ref() as &_)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        // TODO Add offset where possible: comment out this impl, then fix all errors by explicitly
        // converting to wasm::Error where possible and adding offset information.
        Error { offset: None, kind: ErrorKind::Io, source: Some(Box::new(e)) }
    }
}

impl From<ParseLeb128Error> for Error {
    fn from(e: ParseLeb128Error) -> Self {
        Error { offset: None, kind: ErrorKind::Leb128, source: Some(Box::new(e)) }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error { offset: None, kind: ErrorKind::Utf8, source: Some(Box::new(e)) }
    }
}

impl Error {
    // TODO integrate with_offset into new(), as soon as offset is required
    pub fn new(kind: ErrorKind) -> Self {
        Error { offset: None, kind, source: None }
    }

    /// Convenience constructor.
    pub fn invalid_tag(grammar_element: &'static str, tag: u8) -> Self {
        Error { offset: None, kind: ErrorKind::Tag(grammar_element, tag), source: None }
    }

    // Get the current offset in the file, memory buffer etc. and enrich the error with it.
    pub fn with_offset(mut self, seek: &mut impl io::Seek) -> Self {
        let current_position = seek.seek(io::SeekFrom::Current(0));
        self.offset = current_position.ok();
        self
    }

    pub fn with_source(mut self, source: impl error::Error + 'static + Send) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}