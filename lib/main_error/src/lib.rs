use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

pub struct MainDisplay(Box<fmt::Display>);

impl<D: fmt::Display + 'static> From<D> for MainDisplay {
    fn from(d: D) -> Self {
        MainDisplay(Box::new(d))
    }
}

impl fmt::Debug for MainDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct MainError(Box<Error>);

// NOTE 1) We do not impl Error for MainError.
// This avoids an error because of overlapping From impl. (if MainError impl's Error itself,
// then From<Error> overlaps with the reflexive impl From<T> for any T in the core crate.)
// It is also not necessary, because Result<(), E> for main() only requires E: Debug, not E: Error.
// NOTE 2) We manually impl Debug for MainError instead of deriving it.
// Rust uses Debug to present the error of main() to the user, which doesn't look nice by default.
// So we hack around by using the "pretty" Display trait bound inside our Debug impl.

impl<E: Into<Box<Error>>> From<E> for MainError {
    fn from(e: E) -> Self {
        MainError(e.into())
    }
}

impl fmt::Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)?;
        let mut source = self.0.source();
        while let Some(error) = source {
            write!(f, "\ncaused by: {}", error)?;
            source = error.source();
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ErrorWithCause {
    message: String,
    source: Option<Box<Error>>,
}

impl fmt::Display for ErrorWithCause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorWithCause {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source {
            Some(ref x) => Some(x.as_ref()),
            None => None
        }
    }
}

impl From<ParseIntError> for ErrorWithCause {
    fn from(e: ParseIntError) -> Self {
        ErrorWithCause {
            message: "error with cause: parse int error".into(),
            source: Some(Box::new(e)),
        }
    }
}