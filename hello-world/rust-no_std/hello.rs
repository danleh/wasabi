#![no_main]
#![no_std]
#![feature(lang_items)]

// necessary for no_std to work
#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[no_mangle]
pub fn start() -> u32 {
  0
//	let heap_array = Box::new([1, 2, 3]);
//	return heap_array[0] + 1337;
}

