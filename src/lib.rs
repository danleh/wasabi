#![feature(from_ref)]

extern crate wasm;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate test_utilities;

pub mod instrument;

#[cfg(test)]
mod tests;