# Wasabi

## Installation and Setup

- Dependencies and Tools
    * Git, CMake, and GCC or Clang (at least, possibly other stuff too) 
    * **WebAssembly Binary Toolkit (WABT)**: ```wat2wasm```/```wasm2wat``` for converting Wasm binaries to/from text, and ```wasm-objdump``` for inspecting the binaries.
    ```bash
    git clone --recursive https://github.com/WebAssembly/wabt
    cd wabt
    make
    # add binaries to $PATH, e.g., by appending the following line to ~/.profile
    export PATH="path/to/your/wabt/bin:$PATH"
    # test: should print usage
    wat2wasm
    ```
    
    * **Emscripten**: ```emcc``` for compiling C/C++ programs to WebAssembly.
    ```bash
    git clone https://github.com/juj/emsdk.git
    cd emsdk
    ./emsdk install latest
    ./emsdk activate latest
    # add emcc to $PATH, e.g., by appending the following line to ~/.profile
    source path/to/your/emsdk/emsdk_env.sh
    # test: should print "emcc (Emscripten gcc/clang-like replacement) 1.38.1" or similar
    emcc --version
    ``` 
    
    * **Rust** (> 1.27.0 nightly): ```cargo``` as Rust's package manager and build tool (no need to call ```rustc``` manually) and ```rustup``` for managing different Rust versions (e.g. nightly vs. stable).
    ```bash
    curl https://sh.rustup.rs -o rustup-init.sh
    # follow instructions (typically just enter 1 to proceed)
    # should automatically change ~/.profile to include the binaries in $PATH
    sh rustup-init.sh --default-toolchain=nightly
    # test: should print "cargo 1.27.0-nightly (af3f1cd29 2018-05-03)" or similar
    cargo --version 
    ```

- **Wasabi** itself
```bash
git clone https://github.com/danleh/wasabi.git
cd wasabi/wasabi/
# download dependencies from https://crates.io and compile
cargo build --release
cp target/releas/wasabi .
# test: should print usage
./wasabi
```

## Usage Tutorial

- Creating WebAssembly Programs
    * Manually
    * From C with Emscripten
- Instrument with Wasabi
- Analyze with Wasabi...
    * ...in the Browser
    * ...in Node.js