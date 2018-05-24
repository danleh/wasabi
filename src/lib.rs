#![feature(test, from_ref)]

extern crate wasm;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate test;
extern crate wasm_validate;

pub mod instrument;
#[cfg(test)]
mod tests;