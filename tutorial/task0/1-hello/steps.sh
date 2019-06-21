wat2wasm hello.wat
hexdump -C hello.wasm
wasm2wat hello.wasm
wasm-objdump -hxd hello.wasm
python -m SimpleHTTPServer
