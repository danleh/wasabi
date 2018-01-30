#![feature(attr_literals)]

#[macro_use]
extern crate hello_world_derive;

pub trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
enum FrenchToast {
    // TODO disallow derive(ParseWasm) for enums where variants are not explicitly given an opcode
    Bla,
    // TODO implement derive(ParseWasm) for Unit enum variants
    #[tag = 0x40] Nop,
    // TODO implement derive(ParseWasm) for field enums
    #[tag = 0x41] Variant(u32),
}

#[derive(HelloWorld)]
// TODO implement derive(ParseWasm) for tuple structs
struct Waffles(u32);

#[derive(HelloWorld)]
// TODO implement derive(ParseWasm) for structs with named fields
struct WafflesField {
    field_name: u32
}

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}