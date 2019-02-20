#![deny(missing_docs)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! Print errors with [`Display`] instead of [`Debug`] when using `?` in `main()`.
//!
//! Use like `fn main() -> Result<(), MainError>`. See below for more details.
//!
//! # The Problem
//!
//! Since [Rust 1.26](https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#main-can-return-a-result), `main` can return a [`Result<T, E>`](core::result).
//! This enables the use of `?` for convenient error handling ([RFC](https://github.com/rust-lang/rfcs/pull/1937)). For example:
//!
//! ```should_panic
//! # use std::num::ParseIntError;
//! fn main() -> Result<(), ParseIntError> {
//!     let num: i32 = "not a number".parse()?; // will fail and print an error
//!     // ...
//! #    Ok(())
//! }
//! ```
//!
//! Unfortunately, the error is printed via [`Debug`] ([hardcoded in the standard library](https://doc.rust-lang.org/src/std/process.rs.html#1618-1624)), which gives not very pretty or human-friendly output.
//! For example, the error above is printed as:
//!
//! ```output
//! Error: ParseIntError { kind: InvalidDigit }
//! ```
//!
//! # The Solution
//!
//! This crate provides [`MainError`] as a drop-in replacement for the error type `E` in your `main`'s `Result<T, E>`.
//! It prints the error via [`Display`] instead of [`Debug`], which yields a nicer error message.
//! For example, the program above can be changed to
//!
//! ```should_panic
//! use main_error::MainError;
//!
//! fn main() -> Result<(), MainError> {
//!     let _: i32 = "not a number".parse()?;
//!     // ...
//! #    Ok(())
//! }
//! ```
//!
//! and now prints:
//! ```output
//! Error: invalid digit found in string
//! ```
//!
//! # Details and Drawbacks
//!
//! - [`MainError`] stores the original error as `Box<dyn Error>`.
//!   This incurs one allocation (on conversion) and one virtual call (on printing).
//!   Since there can be exactly one error like this before the program ends, this cost is insignificant.
//! - [`MainError`] implements [`From`] for all types that can be converted into a `Box<dyn Error>`.
//!     1. This allows it to be used in place of any type that implements the [`Error`] trait (see example above).
//!     2. It can also be used in place of any type that can be _converted_ to a `Box<dyn Error>`, e.g., `String`.
//! - [`MainError`] does not implement the [`Error`] trait itself.
//!     1. It _doesn't have to_, because the standard library only requires `E: Debug` for `main() -> Result<T, E>`.
//!     2. It _doesn't need to_, because the [`Error`] trait is mostly for interoperability between libraries, whereas [`MainError`] should only be used in `main`.
//!     3. It simply _cannot_, because this would create an overlapping `impl`.
//!        [`MainError`] can be converted from `Into<Box<dyn Error>>`.
//!        `Into<Box<dyn Error>>` [is implemented](https://doc.rust-lang.org/src/std/error.rs.html#219) for `E: Error` itself.
//!        If [`MainError`] impl's `Error`, it would mean [`MainError`] could be converted from itself.
//!        This collides with the [reflexive `impl<T> From<T> for T` in core](https://doc.rust-lang.org/nightly/src/core/convert.rs.html#445-449).
//! - [`MainError`] implements [`Debug`] in terms of [`Display`] of the underlying error.
//!   This is hacky, but unfortunately, [`Debug`] as the output for the `main` error case is stable now.
//!   The `"Error: "` part at the beginning of the output comes [from the standard library](https://doc.rust-lang.org/src/std/process.rs.html#1621), thus it cannot be changed.

use std::error::Error;
use std::fmt::{self, Debug, Display};

/// Newtype wrapper around a boxed [`std::error::Error`].
/// - It implements [`Debug`] so that it can be used in `fn main() -> Result<(), MainError>`.
/// - It implements [`From<E>`](From) for `E: Into<Box<dyn Error>>` so that it works as a drop-in for any type that can be converted into a boxed [`Error`] (i.e., an `Error` trait object).
///
/// `MainError` can only be constructed through its [`From`] impl:
/// Explicitly with `from`/`into` or implicitly through the `?` operator.
///
/// # Example
///
/// Explicit construction via `MainError::from`:
/// ```
/// # use main_error::MainError;
/// let e = MainError::from("something convertible to Box<dyn Error>");
/// ```
///
/// Or via `into()` when the target type can be inferred from the context:
/// ```should_panic
/// # use main_error::MainError;
/// fn main() -> Result<(), MainError> {
///     Err("something convertible to Box<dyn Error>".into())
/// }
/// ```
///
/// Or even easier via `?`:
/// ```should_panic
/// # use main_error::MainError;
/// fn main() -> Result<(), MainError> {
///     Err("something convertible to Box<dyn Error>")?
/// }
/// ```
pub struct MainError(Box<dyn Error>);

impl<E: Into<Box<dyn Error>>> From<E> for MainError {
    fn from(e: E) -> Self {
        MainError(e.into())
    }
}

// impl Debug (to satisfy trait bound for main()-Result error reporting), but use Display of wrapped
// error internally (for nicer output).
impl Debug for MainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)?;
        let mut source = self.0.source();
        while let Some(error) = source {
            write!(f, "\ncaused by: {}", error)?;
            source = error.source();
        }
        Ok(())
    }
}
