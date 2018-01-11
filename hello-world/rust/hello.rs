#![no_main]
#[no_mangle]
pub fn main () {
}

// BUG when compiled without optimizations (-O): "RuntimeError: index out of bounds"
// see https://github.com/rust-lang/rust/issues/46367
#[no_mangle]
pub fn test() -> i32 {
	let vec = vec![1, 2, 3];
	return vec[2] + 1337;
}

// #[no_mangle]
// pub fn test() -> i32 {
// 	return 1337;
// }