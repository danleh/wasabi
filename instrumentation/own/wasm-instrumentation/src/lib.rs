#![feature(attr_literals, specialization, conservative_impl_trait, test)]

#[macro_use]
extern crate custom_derive;
extern crate byteorder;
extern crate rayon;
extern crate walkdir;
extern crate test;
extern crate tempfile;

mod leb128;
mod ast;
mod binary;
mod instrument;
mod display;
mod utils;
#[cfg(test)]
mod tests;