#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
	fn log(s: u8);
}

#[wasm_bindgen]
pub fn instrument(bytes: &[u8]) -> Vec<u8> {
	let mut vec = Vec::new();
	for &byte in bytes {
		vec.push(byte+1);
	}
	vec
}