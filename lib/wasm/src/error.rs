//! Typed errors and warnings when parsing/encoding of modules.
 
use crate::extensions::WasmExtension;

/// Used only for errors (not recoverable, i.e., parsing stops and does not return an AST).
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct ParseError(
    // Put behind a box to limit size of `Result::Err` variant to a single pointer.
    Box<ParseIssue>
);

pub type ParseWarnings = Vec<ParseIssue>;

/// Used both for warnings (recoverable, i.e., parsing can continue afterwards) and errors
/// (not recoverable, i.e., parsing stops and does not return an AST).
#[derive(Debug, thiserror::Error)]
pub enum ParseIssue {
    #[error("error parsing WebAssembly binary at offset 0x{:x}: {}", .0.offset(), .0.message())]
    Wasmparser(#[from] wasmparser::BinaryReaderError),

    #[error("error parsing WebAssembly binary at offset 0x{:x}: {}", offset, message)]
    Message {
        offset: usize,
        message: &'static str,
        #[source]
        source: Option<Box<ParseIssue>>,
    },

    #[error("index out of bounds at offset 0x{:x}: invalid {} index {}", offset, index_space, index)]
    Index {
        offset: usize,
        index: u32,
        index_space: &'static str,
    },

    #[error("unsupported WebAssembly extension at offset 0x{:x}: {} (see also {})", offset, extension.name(), extension.url())]
    Unsupported {
        offset: usize,
        extension: WasmExtension
    },

    #[error(transparent)]
    Io(#[from] std::io::Error)
}

// Convenience constructors/methods.
impl ParseIssue {
    pub fn message(offset: usize, message: &'static str, source: Option<Box<ParseIssue>>) -> Self {
        ParseIssue::Message { offset, message, source }
    }

    pub fn index(offset: usize, index: u32, index_space: &'static str) -> Self {
        ParseIssue::Index { offset, index, index_space }
    }

    pub fn unsupported(offset: usize, extension: WasmExtension) -> Self {
        ParseIssue::Unsupported { offset, extension }
    }

    pub fn offset(&self) -> Option<usize> {
        match self {
            ParseIssue::Wasmparser(err) => Some(err.offset()),
            ParseIssue::Message { offset, .. } => Some(*offset),
            ParseIssue::Index { offset, .. } => Some(*offset),
            ParseIssue::Unsupported { offset, .. } => Some(*offset),
            ParseIssue::Io(_) => None,
        }
    }
}

impl ParseError {
    pub fn new(issue: ParseIssue) -> Self {
        ParseError(Box::new(issue))
    }

    pub fn offset(&self) -> Option<usize> {
        self.0.offset()
    }
}

// Allow conversion of everything that can be converted into a `ParseIssue`
// also into the `ParseError` wrapper directly.
impl<T> From<T> for ParseError 
where T : Into<ParseIssue>
{
    fn from(err: T) -> Self {
        ParseError(Box::new(err.into()))
    }
}


#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EncodeError(
    // Put the actual error behind a box, to keep the size down to a single pointer.
    Box<EncodeErrorInner>
);

#[derive(Debug, thiserror::Error)]
pub enum EncodeErrorInner {
    #[error("error while encoding WebAssembly module to binary format: {}", .0)]
    Message(String),

    #[error("invalid WebAssembly module: reference (e.g. in instructions) to {} {} which is not declared in previous sections", index_space, index)]
    Index {
        index: u32,
        index_space: &'static str,
    },

    #[error(transparent)]
    Io(#[from] std::io::Error)
}

impl EncodeError {
    pub fn message(message: String) -> Self {
        EncodeError(Box::new(EncodeErrorInner::Message(message)))
    }

    pub fn index<T>(index: crate::Idx<T>, index_space: &'static str) -> Self {
        EncodeError(Box::new(EncodeErrorInner::Index { index: index.to_u32(), index_space }))
    }
}

// Allow conversion of everything that can be converted into a `ParseIssue`
// also into the `ParseError` wrapper directly.
impl<T> From<T> for EncodeError 
where T : Into<EncodeErrorInner>
{
    fn from(err: T) -> Self {
        EncodeError(Box::new(err.into()))
    }
}
