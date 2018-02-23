#![no_main]
#![no_std]
#![feature(lang_items)]
#![allow(unused_variables, dead_code)]

// necessary for no_std to work
#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[derive(Copy, Clone)]
pub enum Val {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Copy, Clone)]
pub struct InstructionLocation {
    function: u32,
    instruction: u32,
}

#[no_mangle]
pub extern fn call(i: InstructionLocation, function: u32) {}

fn return_(i: InstructionLocation, values: &[Val]) {
    values.iter().map(|val| match val {
        &Val::I32(val) => print(val),
        _ => {},
    });
}

#[no_mangle]
pub extern fn return_i32(i: InstructionLocation, val0: i32) {
    return_(i, &[Val::I32(val0)]);
}

fn print(i: i32) {
    unsafe {
        print_(i);
    }
}

extern {
    pub fn print_(_: i32);
}