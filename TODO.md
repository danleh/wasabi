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

# Applications, Analyses

- (cf. tracing JITs): detect hot loops by counting ```begin_loop``` hook invocations per loop location