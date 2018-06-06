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
        | - | - | - |
        | Wat | Analysis in JS | Console output |
        | - | - | - |
- Documentation on how to run (integration style, see below) tests

# Features

- automatic (```cargo test```-able) integration tests for analyses 
    * using Wasm in Node.js
    * make sure null- or log-all-analysis run without exception
- Q: should ```*_end``` hook be called *after* ```...end``` instruction for non-loops? (otherwise ```br 0``` will "jump" over the hook) 
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
        - how to handle memory/tables

# Applications, Analyses

- (cf. tracing JITs): detect hot loops by counting ```begin_loop``` hook invocations per loop location