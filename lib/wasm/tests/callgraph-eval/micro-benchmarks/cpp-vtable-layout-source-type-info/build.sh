#!/bin/bash
# Compile without exceptions and RTTI to make the binary smaller and a bit easier to understand.
# Compile with debug info for better function names.
# But optimize for a realistic binary.
emcc -O3 -g -fno-exceptions -fno-rtti main.cpp -o main.wasm
ls -lah
wasm2wat --generate-names -f main.wasm -o main.wasm.wat

wasmtime main.wasm
# Should print B
wasmtime main.wasm asd
# Should print A

# For tracing the call (needs a build of WABT with WASI support)
wasm-interp --wasi --trace main.wasm | grep call
#0.  676: V:0  | call $3
#0.  684: V:0  | call $12
#1.  708: V:0  | call $13
#2.  812: V:6  | call_import $0
#2. 1084: V:6  | call_import $1
#2. 1160: V:6  | call $14
#3. 1300: V:8  | call $6
#4.  536: V:9  | call $4
#5.  184: V:12 | call_indirect $0, $0
#4.  552: V:10 | call $5
#5.  436: V:13 | call_indirect $0, $0
#0.  692: V:1  | call $17
#1. 1364: V:1  | call $18
#1. 1372: V:1  | call $16
#2. 1352: V:1  | call $18
#1. 1380: V:1  | call $18
#1. 1396: V:2  | call $15
#2. 1332: V:3  | call_import $2