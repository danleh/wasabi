use std::{io, fmt, error};
use std::string::FromUtf8Error;

use wasabi_leb128::ParseLeb128Error;

/// Errors while parsing a WebAssembly binary.
// TODO For now these errors appear only when parsing a binary to our AST format, but not when
// serializing our AST to a binary. Obviously, our AST is not correct by construction, so we should
// think about converting some of those errors from panics to their own error type.
#[derive(Debug)]
pub struct Error(
    /// Put the actual error behind a box, otherwise the size of the Result is huge (~80 bytes).
    Box<Repr>
);

#[derive(Debug)]
struct Repr {
    /// Position of the reader when parsing failed.
    /// I.e., the byte offset in the input binary just before the parsing error.
    offset: usize,

    /// The type of error.
    kind: ErrorKind,

    /// Grammar element that we attempted to parse when the error occurred.
    /// Tries to be more high-level than just primitive types. E.g., if parsing of a section size
    /// failed, we would want "Section" here, not "u32".
    // TODO This is makeshift run-time type information, is there a better way to do this?
    // Some things I tried that do not work:
    // - PhantomData: pollutes Error with a type parameter and becomes too unwieldy.
    // - TypeId: does not allow to derive the type_name from it, it is basically just a u64, so no
    // way to communicate the type to a user, e.g., in an error message.
    // - Any: would have to produce a value of that type, but the point is that I could not parse
    // that type in the first place, so would have to introduce a dummy value.
    // - hand-written &str: does not work for WithSize<T>, since I cannot hand-write a description
    // for the generic T.
    grammar_element: String,

    source: Option<Box<dyn error::Error + 'static + Send + Sync>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ErrorKind {
    /// Wrong magic number, i.e., not b"\0asm".
    MagicNumber { actual: [u8; 4] },
    /// Wrong version. We can only handle WebAssembly v1, i.e., the MVP.
    Version { actual: u32 },
    /// A grammar element with a specified size in bytes was longer than expected.
    Size { expected: u32, actual: usize },
    /// Unknown tag (variant) for a particular grammar element, e.g., unknown opcode.
    Tag { actual: u8 },
    /// Input ended too early, but we expected more bytes for the current grammar element.
    Eof,
    /// A number was not valid LEB128 or could not be parsed to the target number type.
    Leb128,
    /// A string was not valid UTF-8.
    Utf8,
    /// An I/O error that is not specific to WebAssembly.
    Io,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Wasm binary at offset 0x{:x} ({}): ", self.0.offset, self.0.offset)?;
        match self.0.kind {
            ErrorKind::MagicNumber { actual } =>
                write!(f, "invalid magic number 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}",
                       actual[0], actual[1], actual[2], actual[3]),
            ErrorKind::Version { actual } =>
                write!(f, "version was {}, but can only parse WebAssembly v1 (MVP)", actual),
            ErrorKind::Size { expected, actual } =>
                write!(f, "declared size doesn't match after parsing {}: expected {} bytes, but actually read {} bytes", &self.0.grammar_element, expected, actual),
            ErrorKind::Tag { actual } =>
                write!(f, "unexpected tag byte 0x{:02x} for {}", actual, &self.0.grammar_element),
            ErrorKind::Eof =>
                write!(f, "unexpected end of input while parsing {}", &self.0.grammar_element),
            ErrorKind::Leb128 =>
                write!(f, "invalid LEB128 number while parsing {}", &self.0.grammar_element),
            ErrorKind::Utf8 => f.write_str("invalid UTF-8 string"),
            ErrorKind::Io => f.write_str("I/O error")
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source.as_ref().map(|e| e.as_ref() as &_)
    }
}

/// Extension trait for Result<T, E> that allows convenient conversion of the error to our own type
/// by adding additional information (unlike what the Into trait could do).
pub trait AddErrInfo<T> {
    /// Adds the error offset and the current parsed type as additional information to the existing
    /// error. Does not change the Ok(_) variant of the Result.
    fn add_err_info<GrammarElement>(self: Self, offset: usize) -> Result<T, Error>;
}

impl<T> AddErrInfo<T> for Result<T, ParseLeb128Error> {
    fn add_err_info<GrammarElement>(self: Result<T, ParseLeb128Error>, offset: usize) -> Result<T, Error> {
        self.map_err(|err|
            Error::with_source::<GrammarElement, _>(offset, ErrorKind::Leb128, err))
    }
}

impl<T> AddErrInfo<T> for Result<T, FromUtf8Error> {
    fn add_err_info<GrammarElement>(self: Result<T, FromUtf8Error>, offset: usize) -> Result<T, Error> {
        self.map_err(|err|
            Error::with_source::<GrammarElement, _>(offset, ErrorKind::Utf8, err))
    }
}

impl<T> AddErrInfo<T> for Result<T, io::Error> {
    fn add_err_info<GrammarElement>(self: io::Result<T>, offset: usize) -> Result<T, Error> {
        self.map_err(|io_err|
            Error::from_io_err::<GrammarElement>(io_err, offset))
    }
}

/// Extension trait for Result<T, Error> (i.e., Result with our own error type).
pub trait SetErrElem<T> {
    /// Update the grammar element that was currently parsed when the error happened to something
    /// more specific (e.g., "Section" instead of a not very useful "u32").
    fn set_err_elem<GrammarElement>(self: Self) -> Self;
}

impl<T> SetErrElem<T> for Result<T, Error> {
    fn set_err_elem<GrammarElement>(self: Self) -> Self {
        self.map_err(|mut err| {
            err.0.grammar_element = grammar_element::<GrammarElement>();
            err
        })
    }
}

impl Error {
    pub fn new<GrammarElement>(offset: usize, kind: ErrorKind) -> Self {
        Error(Box::new(Repr {
            offset,
            kind,
            grammar_element: grammar_element::<GrammarElement>(),
            source: None,
        }))
    }

    pub fn with_source<GrammarElement, E: error::Error + 'static + Send + Sync>(offset: usize, kind: ErrorKind, source: E) -> Self {
        let mut err = Error::new::<GrammarElement>(offset, kind);
        err.0.source = Some(Box::new(source));
        err
    }

    /// Convenience constructor for the most common kind of error.
    pub fn invalid_tag<GrammarElement>(offset: usize, actual_tag: u8) -> Self {
        Error::new::<GrammarElement>(offset, ErrorKind::Tag { actual: actual_tag })
    }

    /// Conversion from an io::Error has special variant for Eof, where additional information is added.
    pub fn from_io_err<GrammarElement>(io_err: io::Error, offset: usize) -> Self {
        // Special case EOF io::Errors, because we want to add information which grammar element
        // was currently in the process of parsing.
        if io_err.kind() == io::ErrorKind::UnexpectedEof {
            Error::new::<GrammarElement>(offset, ErrorKind::Eof)
        } else {
            Error::with_source::<GrammarElement, _>(offset, ErrorKind::Io, io_err)
        }
    }

    pub fn offset(&self) -> usize { self.0.offset }

    pub fn kind(&self) -> &ErrorKind { &self.0.kind }

    pub fn grammar_element(&self) -> &str { &self.0.grammar_element }
}

/// Utitlity function. Makes the type_name a bit more user-friendly by removing at least some common
/// type path prefixes.
fn grammar_element<T>() -> String {
    std::any::type_name::<T>().replace("alloc::vec::", "").replace("alloc::string::", "")
}
