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
    // TODO add grammar_element: TypeId and remove from Size, Tag, Eof
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
    Size { grammar_element: &'static str, expected: u32, actual: usize },
    /// Could not parse grammar element, because of an unknown tag (variant).
    Tag { grammar_element: &'static str, actual_tag: u8 },
    /// Could not parse grammar element (first argument), because input ended too early.
    Eof { grammar_element: &'static str },
    /// A number was not valid LEB128 or could not be parsed to the target number type.
    Leb128(ParseLeb128Error),
    /// A string was not valid UTF-8.
    Utf8(FromUtf8Error),
    /// An I/O error that is not specific to WebAssembly parsing.
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "invalid Wasm binary at offset 0x{:x} ({}): {}", self.offset, self.offset, self.kind)
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ErrorKind::MagicNumber { actual } =>
                write!(f, "invalid magic number 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}",
                       actual[0], actual[1], actual[2], actual[3]),
            ErrorKind::Version { actual } =>
                write!(f, "invalid version 0x{:x}, can only parse WebAssembly v1 (MVP)", actual),
            ErrorKind::Size { grammar_element, expected, actual } =>
                write!(f, "invalid size of {}, expected {} bytes, but read {} bytes", grammar_element, expected, actual),
            ErrorKind::Eof { grammar_element } =>
                write!(f, "unexpected end of input while parsing {}", grammar_element),
            ErrorKind::Tag { grammar_element, actual_tag } =>
                write!(f, "unexpected tag byte 0x{:02x} for {}", actual_tag, grammar_element),
            ErrorKind::Leb128(_) => f.write_str("invalid LEB128 number"),
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

/// Extension trait for `io::Error` to add error offset and grammar element that is currently parsed.
/// Also converts io::Errors with ErrorKind::UnexpectedEof to Eof errors in our error hierarchy.
pub trait IoResultExt<T> {
    fn add_err_info(self: Self, offset: usize, grammar_element: &'static str) -> Result<T, Error>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn add_err_info(self: io::Result<T>, offset: usize, grammar_element: &'static str) -> Result<T, Error> {
        self.map_err(|io_err| Error::from_io_err(io_err, offset, grammar_element))
    }
}

/// Extension trait for other error types that can be wrapped to our own type.
/// Adds the error offset as additional information.
pub trait ResultExt<T> {
    fn add_err_info(self: Self, offset: usize) -> Result<T, Error>;
}

impl<T> ResultExt<T> for Result<T, ParseLeb128Error> {
    fn add_err_info(self: Result<T, ParseLeb128Error>, offset: usize) -> Result<T, Error> {
        self.map_err(|err| Error { offset, kind: ErrorKind::Leb128(err) })
    }
}

impl<T> ResultExt<T> for Result<T, FromUtf8Error> {
    fn add_err_info(self: Result<T, FromUtf8Error>, offset: usize) -> Result<T, Error> {
        self.map_err(|err| Error { offset, kind: ErrorKind::Utf8(err) })
    }
}

impl Error {
    pub fn new(offset: usize, kind: ErrorKind) -> Self {
        Error { offset, kind }
    }

    /// Convenience constructor for the most common kind of error.
    pub fn invalid_tag(offset: usize, grammar_element: &'static str, actual_tag: u8) -> Self {
        Error::new(offset, ErrorKind::Tag { grammar_element, actual_tag })
    }

    /// Conversion from an io::Error has special variant for Eof, where additional information is added.
    pub fn from_io_err(io_err: io::Error, offset: usize, grammar_element: &'static str) -> Self {
        // Special case EOF io::Errors, because we want to add information which grammar element
        // was currently in the process of parsing.
        let kind = if io_err.kind() == io::ErrorKind::UnexpectedEof {
            ErrorKind::Eof { grammar_element }
        } else {
            ErrorKind::Io(io_err)
        };
        Error::new(offset, kind)
    }
}