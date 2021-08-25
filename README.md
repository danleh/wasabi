# Wasabi

## Tutorial at PLDI 2019

We offered a tutorial on how to use Wasabi for dynamically analyzing WebAssembly at [PLDI 2019](https://conf.researchr.org/home/pldi-2019). Although the conference is now over, all the material is online at http://wasabi.software-lab.org/tutorial-pldi2019/. In particular, the [slides are available here](http://wasabi.software-lab.org/wasabi_tutorial_slides.pdf) and the hands-on tasks are in this repo under [tutorial/](https://github.com/danleh/wasabi/tree/master/tutorial).

## Installation from Source

- Dependencies and tools
    * Git, CMake, and GCC or Clang for building the dependencies (those for sure, but possibly more)
    * **Firefox** >= 52 (which is what I use, or Chrome >= 57) for running WebAssembly
    * **WebAssembly Binary Toolkit (WABT)**: ```wat2wasm```/```wasm2wat``` for converting Wasm binaries to/from text, ```wasm-objdump``` for inspecting binaries, and ```wasm-interp``` for a simple interpreter. (See https://github.com/WebAssembly/wabt#cloning.)
    ```bash
    git clone --recursive https://github.com/WebAssembly/wabt
    cd wabt
    make
    
    # add binaries to $PATH, e.g., by appending the following line to ~/.profile or ~/.bashrc
    export PATH="path/to/your/wabt/bin:$PATH"
    
    # test
    wat2wasm
    > usage: wat2wasm [options] filename
    ```
    
    * **Emscripten**: ```emcc``` for compiling C/C++ programs to WebAssembly. (See https://emscripten.org/docs/getting_started/downloads.html.)
    ```bash
    git clone https://github.com/emscripten-core/emsdk.git
    cd emsdk
    ./emsdk install latest
    ./emsdk activate latest
    
    # add emcc to $PATH, e.g., by appending the following line to ~/.profile or ~/.bashrc
    # WARNING unfortunately, this also exports a quite old emscripten-included clang version,
    # so maybe do this manually before using emscripten and not for all shells. 
    source path/to/your/emsdk/emsdk_env.sh
    
    # test
    emcc --version
    > emcc (Emscripten gcc/clang-like replacement) 1.38.1
    ``` 
    
    * **Rust**: ```cargo``` as Rust's package manager and build tool (no need to call ```rustc``` manually) and ```rustup``` for managing different Rust toolchain versions. See https://www.rust-lang.org/tools/install. If there are build errors, please make sure you use a recent stable version of Rust.
    ```bash
    curl https://sh.rustup.rs -o rustup-init.sh
    # follow instructions (typically just enter 1 to proceed)
    # should automatically change ~/.profile to include the binaries in $PATH
    sh rustup-init.sh
    
    # test
    cargo --version
    > cargo 1.41.1-stable
    ```

- **Wasabi** itself
```bash
git clone https://github.com/danleh/wasabi.git
cd wasabi
# download dependencies from https://crates.io, compile with optimizations, make wasabi binary available in $PATH
cargo install --path .

# test
wasabi
> Error: expected at least one argument
> Usage: wasabi <input_wasm_file> [<output_dir>]
```

## Alternative Setup via Docker

- Thanks to [ctfhacker](https://github.com/ctfhacker) for the Dockerfile contribution.
- After having cloned this repo, you build the Docker image with
```bash
docker build --rm -t wasabi .
```
- Once built, you can use the container via (assuming you have a hello.wasm file in your working directory)
```bash
ls
> hello.wasm
docker run --rm -t -v `pwd`:/data  wasabi /data/hello.wasm /data
```

## Usage Tutorial

- **Create** WebAssembly programs
    * Manually:
    ```sexp
    ;; paste into hello-manual.wat
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
    wat2wasm hello-manual.wat
    
    # run binary (imported function host.print is provided by the interpreter)
    wasm-interp --host-print --run-all-exports hello-manual.wasm
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

- Apply **Wasabi** to WebAssembly programs in the **browser** 
    * Step 1: **Instrument**
        ```bash
        # start with C to Wasm (via Emscripten) project from previous point, that is:  
        ls
        > hello.c  hello.html  hello.js  hello.wasm
    
        # instrument hello.wasm, produces 2 files in out/:
        # - out/hello.wasm: instrumented binary, with imported hooks and calls to these hooks inserted between instructions
        # - out/hello.wasabi.js: Wasabi loader, runtime, and generated program-dependent JavaScript (low-level monomorphized hooks and statically extracted information about the binary)
        wasabi hello.wasm
    
        # replace original binary with instrumented one and copy generated JavaScript 
        mv hello.wasm hello.orig.wasm
        cp out/* .
    
        # insert Wasabi-generated JavaScript into emscripten-generated HTML harness (FIXME hacky)
        sed -i '/<script async type="text\/javascript" src="hello.js"><\/script>/a <script src="hello.wasabi.js"></script>' hello.html
    
        # should still work as before instrumentation
        emrun --no_browser --port 8080 .
        firefox http://localhost:8080/hello.html
        ```

    * Step 2: **Analyze**
        ```bash
        # use example analysis that just logs all instructions with their inputs and results
        cp /path/to/wasabi/analyses/log-all.js .
      
        # include analysis in emscripten-generated HTML harness (FIXME hacky)
        # NOTE analysis must be loaded *after* the Wasabi-generated JavaScript
        sed -i '/<script src="hello.wasabi.js"><\/script>/a <script src="log-all.js"></script>' hello.html
      
        # run in browser again, see lots of output on JavaScript console
        emrun --no_browser --port 8080 .
        firefox http://localhost:8080/hello.html
        ```

-  Running WebAssembly and instrumenting with Wasabi for **Node.js**: **TODO**
