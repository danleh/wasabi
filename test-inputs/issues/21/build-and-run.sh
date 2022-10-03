#!/bin/bash

# needs WABT and Node.js >v14 installed

# clean
echo "Clean old files..."
rm -f test.wasm
rm -f test.stdout.txt
rm -rf out/

echo "Build and run original and instrumented binaries..."
# build original test file
wat2wasm test.wat

# run and store original trace for later comparison with instrumented binary
wasm-interp --dummy-import-func --run-all-exports test.wasm > test.stdout.txt

# instrument the binary
cargo run -- --node test.wasm
# (provide a human readable version of the instrumented binary)
wasm2wat --generate-names -f out/test.wasm -o out/test.wasm.wat

# run the instrumented binary and compare their traces
wasm-interp --dummy-import-func --run-all-exports out/test.wasm | grep -v __wasabi > out/test.no_wasabi.stdout.txt
# (this is only for debugging)
wasm-interp --dummy-import-func --run-all-exports out/test.wasm > out/test.stdout.txt

echo "Diffing wasm-interp trace before and after instrumentation..."
diff test.stdout.txt out/test.no_wasabi.stdout.txt || {
    echo "Mismatching trace before and after instrumentation! Inspect test.stdout.txt files!"
    exit 1
}

# run with analysis in Node.js (>=v14)
echo "Testing for correct resolved function indices in JavaScript analysis..."
node --unhandled-rejections=strict test.js 2>&1 | grep 'Test failure' && {
    echo "Test failure, run test.js to see the error message!"
    exit 1
}

echo "Test success!"
