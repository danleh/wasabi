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

# (optional:) add wasabi to $PATH
```

## Usage Tutorial

- Creating WebAssembly Programs
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
    ```
- Instrument with Wasabi
- Analyze with Wasabi...
    * ...in the Browser
    * ...in Node.js