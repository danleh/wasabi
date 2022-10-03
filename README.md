# Wasabi: A Dynamic Analysis Framework for WebAssembly

Wasabi is a framework for dynamically analyzing WebAssembly binaries.
A high-level introduction, live demo, and more can be found at http://wasabi.software-lab.org.
Our ASPLOS 2019 paper [*Wasabi: A Framework for Dynamically Analyzing
WebAssembly*](https://software-lab.org/publications/asplos2019_Wasabi.pdf) (which won a best paper award) contains more in-depth explanations and evaluation results.
Check also the `README`s in the other directories of this repository.


## Source Code

Wasabi is built on binary instrumentation.
For that, we have developed our own WebAssembly parser, instrumentation library, and encoder in Rust.
This code may be useful independently of Wasabi as well.
In fact, we have used it also for other projects on WebAssembly analysis and instrumentation.
You can find the source code in `crates/` (the Rust term for libraries).


## Tutorial at PLDI 2019

We gave an introduction and tutorial on using Wasabi at PLDI 2019.
See `tutorial-pldi2019/` for more information.


## Installation

**Dependencies** and **useful tools**:
- Git, CMake, and GCC or Clang for building the dependencies (possibly more).
- A modern **browser**, e.g., Firefox or Chrome.
- **WebAssembly Binary Toolkit (WABT)**: https://github.com/WebAssembly/wabt.
```wat2wasm```/```wasm2wat``` for converting Wasm binaries to/from text, ```wasm-objdump``` for inspecting binaries, and ```wasm-interp``` for a simple interpreter. 
- **Emscripten**: https://emscripten.org. 
For compiling C/C++ programs to WebAssembly. 
- **Rust**: https://www.rust-lang.org/tools/install.
```cargo``` as Rust's package manager and build tool and ```rustup``` for managing different Rust toolchains versions.
If there are build errors, please make sure you use a recent stable version of Rust.

Building **Wasabi** itself:
```bash
git clone https://github.com/danleh/wasabi.git
cd wasabi/crates/wasabi
# Download dependencies from https://crates.io, compile with optimizations, make wasabi binary available in $PATH.
cargo install --path .

# Test:
wasabi
> Error: expected at least one argument
> Usage: wasabi <input_wasm_file> [<output_dir>]
```


## Short Usage Instructions

**Create** a WebAssembly program:

Option A) Manually:
```wasm
;; Paste into hello-manual.wat
(module
    (import "host" "print" (func $i (param i32)))
    (func $somefun
    i32.const 42
    call $i)
    (export "somefun" (func $somefun))
)
```
```bash
# Assemble to binary .wasm file
wat2wasm hello-manual.wat

# Run the binary. (The imported function host.print is provided by the interpreter.)
wasm-interp --host-print --run-all-exports hello-manual.wasm
> called host host.print(i32:42) =>
> somefun() =>
```

Option B) Compile from C with Emscripten:
```C
// Paste into hello.c
#include <stdio.h>
int main(int argc, char const *argv[]) {
    printf("Hello, world!\n");
    return 0;
}
```
```bash
# Emscripten produces a small wrapper website alongside the WebAssembly code, such that you can execute it in the browser.
emcc hello.c -o hello.html

# Due to browser security policies, you need to serve the website from a web server. 
emrun --no_browser --port 8080 .

# Open website with Firefox or Chrome.
firefox http://localhost:8080/hello.html
chromium-browser http://localhost:8080/hello.html

# (Optional:) Inspect the produced binary with wasm2wat or wasm-objdump.
wasm2wat hello.wasm -o hello.wat
wasm-objdump hello.wasm -hdx | less
```

Apply **Wasabi** to WebAssembly programs in the **browser**:
* Step 1: **Instrument**
    ```bash
    # Start with the C to Wasm (via Emscripten) project from the previous step:
    ls
    > hello.c  hello.html  hello.js  hello.wasm

    # Instrument hello.wasm, produces 2 files in out/:
    # - out/hello.wasm: instrumented binary, with imported hooks and calls to these hooks inserted between instructions
    # - out/hello.wasabi.js: Wasabi loader, runtime, and generated program-dependent JavaScript (low-level monomorphized hooks and statically extracted information about the binary)
    wasabi hello.wasm

    # Replace the original binary with the instrumented one and copy generated JavaScript.
    mv hello.wasm hello.orig.wasm
    cp out/* .

    # Insert Wasabi-generated JavaScript into Emscripten-generated HTML harness (FIXME hacky).
    sed -i '/<script async type="text\/javascript" src="hello.js"><\/script>/a <script src="hello.wasabi.js"></script>' hello.html

    # Should still run as before the instrumentation.
    emrun --no_browser --port 8080 .
    firefox http://localhost:8080/hello.html
    ```

* Step 2: **Analyze**
    ```bash
    # Use one of the example analyses, e.g., that logs all instructions with their inputs and results.
    cp /path/to/wasabi/analyses/log-all.js .
    
    # Include analysis in Emscripten-generated HTML harness (FIXME hacky).
    # NOTE The analysis must be loaded *after* the Wasabi-generated JavaScript.
    sed -i '/<script src="hello.wasabi.js"><\/script>/a <script src="log-all.js"></script>' hello.html
    
    # Run in the browser again, you should see lots of output on JavaScript console.
    emrun --no_browser --port 8080 .
    firefox http://localhost:8080/hello.html
    ```

You can find more example Wasabi analyses in `example-analyses/`.


## License

Wasabi is licensed under the MIT license.
See `LICENSE` for details.
Other code in this repository or used as a dependency may be licensed differently.
