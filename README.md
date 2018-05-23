# Wasabi

## Installation and Setup

- Dependencies and Tools
    * Git, CMake, and GCC or Clang for building the dependencies (those for sure, but possibly more)
    * **Firefox** >= 52 (which is what I use, or Chrome >= 57) for running WebAssembly
    * **WebAssembly Binary Toolkit (WABT)**: ```wat2wasm```/```wasm2wat``` for converting Wasm binaries to/from text, ```wasm-objdump``` for inspecting binaries, and ```wasm-interp``` for a simple interpreter.
    ```bash
    git clone --recursive https://github.com/WebAssembly/wabt
    cd wabt
    make
    
    # add binaries to $PATH, e.g., by appending the following line to ~/.profile
    export PATH="path/to/your/wabt/bin:$PATH"
    
    # test
    wat2wasm
    > usage: wat2wasm [options] filename
    ```
    
    * **Emscripten**: ```emcc``` for compiling C/C++ programs to WebAssembly.
    ```bash
    git clone https://github.com/juj/emsdk.git
    cd emsdk
    ./emsdk install latest
    ./emsdk activate latest
    
    # add emcc to $PATH, e.g., by appending the following line to ~/.profile
    source path/to/your/emsdk/emsdk_env.sh
    
    # test
    emcc --version
    > emcc (Emscripten gcc/clang-like replacement) 1.38.1
    ``` 
    
    * **Rust** (>=1.27.0 nightly): ```cargo``` as Rust's package manager and build tool (no need to call ```rustc``` manually) and ```rustup``` for managing different Rust toolchain versions (e.g., nightly vs. stable).
    ```bash
    curl https://sh.rustup.rs -o rustup-init.sh
    # follow instructions (typically just enter 1 to proceed)
    # should automatically change ~/.profile to include the binaries in $PATH
    sh rustup-init.sh --default-toolchain=nightly
    
    # test
    cargo --version
    > cargo 1.27.0-nightly
    ```

- **Wasabi** itself
```bash
git clone https://github.com/danleh/wasabi.git
cd wasabi/wasabi/
# download dependencies from https://crates.io and compile with optimizations
cargo build --release

# test
target/release/wasabi
> Usage: wasabi <input_wasm_file> [<output_dir>]

# (optional:) add target/release/wasabi to $PATH
```

## Usage Tutorial

- **Create** WebAssembly Programs
    * Manually:
    ```sexp
    ;; paste into hello.wat
    (module
      (import "host" "print" (func $i (param i32)))
      (func $somefun
        i32.const 42
        call $i)
      (export "somefun" (func $somefun))
    )
    ```
    ```bash
    # assemble binary Wasm file
    wat2wasm hello.wat
    
    # run binary (imported function host.print is provided by the interpreter)
    wasm-interp --host-print --run-all-exports hello.wasm
    > called host host.print(i32:42) =>
    > somefun() =>
    ```
    
    * From C with Emscripten:
    ```C
    // paste into hello.c
    #include <stdio.h>
    int main(int argc, char const *argv[]) {
        printf("Hello, world!\n");
        return 0;
    }
    ```
    ```bash
    # emscripten produces asm.js by default, so use WASM=1 flag
    # note that this generates 3 files: 
    # - hello.wasm: actual binary
    # - hello.js: glue code for compiling and running WebAssembly in the browser, uses fetch() to get hello.wasm
    # - hello.html: website that emulates a console, includes hello.js
    emcc hello.c -s WASM=1 -o hello.html
    
    # (necessary for Chrome, optional for Firefox:) some-origin policy disallows getting 
    # hello.wasm file from inside hello.js unless it is served from an actual webserver
    # emrun is just a minimal webserver provided by emscripten  
    emrun --no_browser --port 8080 .
  
    # browse to local webserver with Firefox or Chrome
    firefox http://localhost:8080/hello.html
    chromium-browser http://localhost:8080/hello.html

    # (optional:) inspect the produced binary with wasm2wat or wasm-objdump
    wasm2wat hello.wasm -o hello.wat
    wasm-objdump hello.wasm -hdx | less
    ```

- **Instrument** with Wasabi for the browser
    ```bash
    # start with C to Wasm (via Emscripten) project from previous point, that is:  
    ls
    > hello.c  hello.html  hello.js  hello.wasm

    # instrument hello.wasm, produces 2 files in out/:
    # - out/hello.wasm: instrumented binary, with imported hooks and calls to these hooks inserted between instructions
    # - out/hello.js: Wasabi glue code, i.e., low-level monomorphized hooks and some statically extracted information about the binary
    wasabi hello.wasm
  
    # copy Wasabi JavaScript loader and long.js library to out/
    cp path/to/wasabi/wasabi/lib/js/* out/
    ls out/
    > hello.js  hello.wasm  long.js  long.js.map  wasabi.js

    # replace original binary with instrumented one
    mv hello.wasm hello.wasm.orig
    cp out/hello.wasm .

    # include Wasabi loader, Wasabi glue code, and long.js library into emscripten-generated HTML harness (FIXME hacky)
    sed -i '/<script async type="text\/javascript" src="hello.js"><\/script>/a <script src="out/wasabi.js"></script><script src="out/hello.js"></script><script src="out/long.js"></script>' hello.html

    # should still work as before instrumentation
    emrun --no_browser --port 8080 .
    firefox http://localhost:8080/hello.html
    ```

- **Analyze** with Wasabi in the browser
    ```js
    // paste into log-all.js
  /*
   * User-facing API for dynamic analyses.
   */
  
  function start(location) {
      console.log(location, "start");
  }
  
  function nop(location) {
      console.log(location, "nop");
  }
  
  function unreachable(location) {
      console.log(location, "unreachable");
  }
  
  function if_(location, condition) {
      console.log(location, "if, condition =", condition);
  }
  
  function br(location, target) {
      console.log(location, "br, to label", target.label, "(==", target.location, ")");
  }
  
  function br_if(location, conditionalTarget, condition) {
      console.log(location, "br_if, (conditionally) to label", conditionalTarget.label, "(==", conditionalTarget.location, "), condition =", condition);
  }
  
  function br_table(location, table, defaultTarget, tableIdx) {
      console.log(location, "br_table, table =", table, ", default target =", defaultTarget, ", table index =", tableIdx);
  }
  
  function begin(location, type) {
      console.log(location, "begin", type);
  }
  
  function end(location, type, beginLocation) {
      console.log(location, "end", type, "(begin @", beginLocation, ")");
  }
  
  function drop(location, value) {
      console.log(location, "drop, value =", value);
  }
  
  function select(location, cond, first, second) {
      console.log(location, "select, condition =", cond, "first =", first, "second =", second);
  }
  
  function call_pre(location, targetFunc, indirect, args) {
      console.log(location, (indirect ? "indirect" : "direct"), "call", "to func #", targetFunc, "args =", args);
  }
  
  function call_post(location, values) {
      console.log(location, "call result =", values);
  }
  
  function return_(location, values) {
      console.log(location, (location.instr === -1) ? "implicit" : "explicit","return, values = ", values);
  }
  
  function const_(location, value) {
      console.log(location, "const, value =", value);
  }
  
  function unary(location, op, input, result) {
      console.log(location, op, "input =", input, "result =", result);
  }
  
  function binary(location, op, first, second, result) {
      console.log(location, op, "first =", first, " second =", second, "result =", result);
  }
  
  function load(location, op, memarg, value) {
      console.log(location, op, "value =", value, "from =", memarg);
  }
  
  function store(location, op, memarg, value) {
      console.log(location, op, "value =", value, "to =", memarg);
  }
  
  function memory_size(location, currentSizePages) {
      console.log(location, "memory_size, size (in pages) =", currentSizePages);
  }
  
  function memory_grow(location, byPages, previousSizePages) {
      console.log(location, "memory_grow, delta (in pages) =", byPages, "previous size (in pages) =", previousSizePages);
  }
  
  function local(location, op, localIndex, value) {
      console.log(location, op, "local #", localIndex, "value =", value);
  }
  
  function global(location, op, globalIndex, value) {
      console.log(location, op, "global #", globalIndex, "value =", value);
  }
    ```
    ```bash
    # include analysis in emscripten-generated HTML harness (FIXME hacky)
    sed -i '/<script async type="text\/javascript" src="hello.js"><\/script>/a <script src="log-all.js"></script>' hello.html
  
    # run in browser again, see lots of console output
    emrun --no_browser --port 8080 .
    firefox http://localhost:8080/hello.html
    ```

-  Running in and instrumenting for Node.js: **TODO**