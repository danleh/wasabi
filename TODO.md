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

- Debug where heavy runtime slowdown comes from, see 2018_07_30 16_53 Office Lens.jpg. Options
    1. added stack manipulation instructions (const, get_local, set_local etc)
        - if so: use single-integer instruction locations, not ```{func, instr}```. You can easily map them back to func + instr by saving the function offsets (do not forget to account for instr === -1 for "virtual instructions") and computing in which range the loc falls.
    2. calls in general, even if to wasm function -> to compare against previous point: replace calls by matching number of ```drop```s
    3. calls to JS, i.e., interop -> to eval this problem: replace JS calls (imported functions) by function stubs implemented in wasm
    4. JS code 
        - to eval if its this: remove JS code in low-level hooks (this is what I already did quickly: seems to bring only ~20%, i.e., is not the main factor)
        - possible solution if allocating op strings (e.g., "i32.load_u8s") is expensive: use ```function.caller.name``` inside a ```op()``` utility function
        - get rid of low-level <> high-level split altogether by (e.g. for br_table) putting all behavior prior to the user-defined analysis functions inside the Wasm module itself. For br_table we could do a "trampoline" function that uses a br_table itself to call end hooks at runtime.
- Reduce memory allocations (see eval/perf/ heaptrack data) in hook_map::instr() and Hook::new()
    * more borrowing, less String
- Idea: "probabilistic instrumentation", combine ideas from sampling with instrumentation by instrumenting uniform randomly, e.g. with p=0.1. E.g., for cryptominer detection or memory performance analyses it would suffice to look only into every tenth binary instruction or memory access, not every single one. Tradeoff: accuracy vs. performance.
- fix wrong handling of ```call_indirect``` in Firefox: exported function object .name property is NOT the Wasm index -> BUG report and compare with Chrome!
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
    * paper name idea: "Streaming and parallel instrumentation"
    * evaluation: latency from "first request sent" to "first function executed" will be MUCH lower with streaming instrumentation vs "blocking"
- Long term/follow up: Analysis in Wasm (not JS)
    * needs merging of analysis code and program code
    * paper name idea: "wasm-merge: Linking without relocation info"
    * how to handle memory/tables
    * for Memory: "multiplex" two memories into a single one, by replacing maintaining a base pointer for the "second" memory and increasing the memory size to the sum of both memories. Each ```memory.grow``` and ```memory.size``` instruction is replaced by an appropriate function (that moves the second memory portion if the first portion has to grow and that maintains this base pointer). Each memory access to the second section is prepended by a load of the global base pointer + add instruction.
    * for Tables: insert runtime comparison of Table index, if out of bounds for this "original modules table", perform
    an indirect call to the 

# Applications, Analyses

- (cf. tracing JITs): detect hot loops by counting ```begin_loop``` hook invocations per loop location