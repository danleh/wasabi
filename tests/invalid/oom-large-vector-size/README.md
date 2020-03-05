Found with AFL, then manually debugged and reduced to minimal test case.

Problem: Wasm parsing fails with too large allocation request on `oom.wasm`, e.g., `memory allocation of 205888945984 bytes failed`.

The binary contains a Wasm vector (see https://webassembly.github.io/spec/core/binary/conventions.html#vectors) with a very large length (encoded as an LEB128).
If we blindly trust this length for pre-allocating a Rust vector in our parser, its allocation request will fail.

Interesting: `wasm-objdump` and `wasm2wat` have the same issue and both fail with a C++ `bad_alloc` exception.

Fix: Limit pre-allocating to realistic, smaller sizes (e.g., 1 MB).
Then, the pre-allocation itself will work, but later during parsing we should fail on the file (because parsing of the elements should fail at some point.)