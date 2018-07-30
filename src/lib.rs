// #![feature(alloc_system)]

extern crate wasm;
extern crate rayon;
extern crate parking_lot;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_plain;
extern crate test_utilities;

pub mod instrument;
pub mod config;

#[cfg(test)]
mod tests;

// // for analyzing heap usage with heaptrack
// extern crate alloc_system;
// use alloc_system::System;
// #[global_allocator]
// static A: System = System;