#![no_main]
#![no_std]
#![feature(lang_items)]

// necessary for no_std to work
#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

pub enum Val {
	I32(i32),
	I64(i64),
	F32(f32),
	F64(f64),
}

#[no_mangle]
pub fn start(slice: &[i32]) -> Option<i32> {
	slice.first().cloned() // interestingly, this is RVO in action (only with call to .cloned()!): start() gets another parameter, which is a ptr to the place where the Option<i32> shall be stored, then it is written to that ptr
}

