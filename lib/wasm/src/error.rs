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
    /// I.e., the byte offset in the input binary just before the parsing error.
    pub offset: usize,
    /// Grammar element that we attempted to parse when the error occurred.
    // TODO change to TypeId
    pub grammar_element: &'static str,
    /// The type of error.
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// Wrong magic number, i.e., not b"\0asm".
    MagicNumber { actual: [u8; 4] },
    /// Wrong version. We can only handle WebAssembly v1, i.e., the MVP.
    Version { actual: u32 },
    /// A grammar element with a specified size in bytes was longer than expected.
    Size { expected: u32, actual: usize },
    /// Unknown tag (variant) for a particular grammar element, e.g., unknown opcode.
    Tag { actual: u8 },
    /// Input ended too early, but we expected more bytes to be available.
    Eof,
    /// A number was not valid LEB128 or could not be parsed to the target number type.
    Leb128(ParseLeb128Error),
    /// A string was not valid UTF-8.
    Utf8(FromUtf8Error),
    /// An I/O error that is not specific to WebAssembly parsing.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wasm binary at offset 0x{:x} ({}): ", self.offset, self.offset)?;
        match self.kind {
            ErrorKind::MagicNumber { actual } =>
                write!(f, "invalid magic number 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}",
                       actual[0], actual[1], actual[2], actual[3]),
            ErrorKind::Version { actual } =>
                write!(f, "version was 0x{:x}, but can only parse WebAssembly v1 (MVP)", actual),
            ErrorKind::Size { expected, actual } =>
                write!(f, "wrong size for {}: expected (declared in the binary) {} bytes, but actually read {} bytes", self.grammar_element, expected, actual),
            ErrorKind::Tag { actual } =>
                write!(f, "unexpected tag byte 0x{:02x} for {}", actual, self.grammar_element),
            ErrorKind::Eof =>
                write!(f, "unexpected end of input while reading {}", self.grammar_element),
            ErrorKind::Leb128(_) =>
                write!(f, "invalid LEB128 number inside {}", self.grammar_element),
            ErrorKind::Utf8(_) => f.write_str("invalid UTF-8 string"),
            ErrorKind::Io(_) => f.write_str("I/O error")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Leb128(err) => Some(err),
            ErrorKind::Utf8(err) => Some(err),
            ErrorKind::Io(err) => Some(err),
            _ => None
        }
    }
}

/// Extension trait for Result<T, E> that allows convenient conversion of the error to our own type.
pub trait ResultExt<T> {
    /// Adds the error offset and the current parsed type as additional information to the existing
    /// error. Does not change the Ok(_) variant of the Result.
    fn add_err_info(self: Self, offset: usize, grammar_element: &'static str) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T, ParseLeb128Error> {
    fn add_err_info(self: Result<T, ParseLeb128Error>, offset: usize, grammar_element: &'static str) -> Result<T, Error> {
        self.map_err(|err| Error { offset, grammar_element, kind: ErrorKind::Leb128(err) })
    }
}

impl<T> ResultExt<T> for Result<T, FromUtf8Error> {
    fn add_err_info(self: Result<T, FromUtf8Error>, offset: usize, grammar_element: &'static str) -> Result<T, Error> {
        self.map_err(|err| Error { offset, grammar_element, kind: ErrorKind::Utf8(err) })
    }
}

impl<T> ResultExt<T> for Result<T, io::Error> {
    fn add_err_info(self: io::Result<T>, offset: usize, grammar_element: &'static str) -> Result<T, Error> {
        self.map_err(|io_err| Error::from_io_err(io_err, offset, grammar_element))
    }
}

impl Error {
    pub fn new(offset: usize, grammar_element: &'static str, kind: ErrorKind) -> Self {
        Error { offset, grammar_element, kind }
    }

    /// Convenience constructor for the most common kind of error.
    pub fn invalid_tag(offset: usize, grammar_element: &'static str, actual_tag: u8) -> Self {
        Error::new(offset, grammar_element, ErrorKind::Tag { actual: actual_tag })
    }

    /// Conversion from an io::Error has special variant for Eof, where additional information is added.
    pub fn from_io_err(io_err: io::Error, offset: usize, grammar_element: &'static str) -> Self {
        // Special case EOF io::Errors, because we want to add information which grammar element
        // was currently in the process of parsing.
        let kind = if io_err.kind() == io::ErrorKind::UnexpectedEof {
            ErrorKind::Eof
        } else {
            ErrorKind::Io(io_err)
        };
        Error::new(offset, grammar_element, kind)
    }
}