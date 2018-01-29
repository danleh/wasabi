#![feature(attr_literals)]

#[macro_use]
extern crate hello_world_derive;

pub trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
#[opcode = 0x41]
struct FrenchToast {
    #[opcode = "bla"] field: u32
}

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello_world();
    Waffles::hello_world();
}