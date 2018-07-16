# Documentation, Usability

- API documentation
    * meaning of parameters?
         - e.g., what is the "op" of store?
         - e.g., what are the properties of "memarg"?
    * when do the callbacks get called?
- "Harness generator", e.g. ```$ wasabi-harness bla.wasm analysis.js``` that
    * by convention: bla.wasm contains ```start``` functions, uses no imports
    * generates "host harness" that runs bla.wasm
    * instruments bla.wasm and includes analysis.js in host harness
- Website, cf. https://jacksongl.github.io/files/demo/jalangiff/index.html
    * with online demo (basically in-browser of "harness generator" above): 

        | Wat | Analysis in JS | Console output |
        | --- | --- | --- |
- Documentation on how to run (integration style, see below) tests

# Features

- automatic (```cargo test```-able) integration tests for analyses 
    * using Wasm in Node.js
    * make sure null- or log-all-analysis run without exception
- proper error handling in library and binary with [failure](https://boats.gitlab.io/failure/intro.html) crate
    * implement own Error type
    * replace panics with ```Result<_, wasabi::Error>```
    * implement ```From``` and ```Error``` traits
- How to handle Wasm traps?
    * (Hacky:) replace Wasm function by Wasm -> JS -> Wasm wrapper that does 
        ```
        try { 
            original_wasm_func()
        } catch (e) {
            trap_hook()
        }
        ```
- Compile Wasabi from Rust -> Wasm and run in the browser
    * No more need to run Wasabi "offline", no need to include generated JS file
- Dynamic re-instrumentation for Wasm: optimizing away hooks of functions that were not provided by
the user (e.g. by detection default function stubs through a ```// EMPTY FUNCTION, PLEASE OPTIMIZE AWAY``` magic
comment)
- Long term/follow up: streaming instrumentation
- Long term/follow up: Analysis in Wasm (not JS)
    * needs merging of analysis code and program code
    * how to handle memory/tables
    * for Memory: "multiplex" two memories into a single one, by replacing maintaining a base pointer for the "second" memory and increasing the memory size to the sum of both memories. Each ```memory.grow``` and ```memory.size``` instruction is replaced by an appropriate function (that moves the second memory portion if the first portion has to grow and that maintains this base pointer). Each memory access to the second section is prepended by a load of the global base pointer + add instruction.
    * for Tables: insert runtime comparison of Table index, if out of bounds for this "original modules table", perform
    an indirect call to the 

# Applications, Analyses

- (cf. tracing JITs): detect hot loops by counting ```begin_loop``` hook invocations per loop location