- Move away from Travis CI (no longer offered for OSS projects) to GitHub actions (?)
    * add test also for `wasm` AST/instrumentation library

# Larger Refactorings, Simplification, Long-term Maintainability

- Keep low-level/high-level structure of wasabi-wasm crate, but:
    * Do not derive low-level parsers from (owning) datatypes, but rather use bytcodealliance's wasmparser: supports many more extensions, streaming parser, no need to maintain my own macro/low-level structure
    * Make high-level AST non-owning by converting everything owning (like Vec<Type>) to iterators?
        - Difficult for things that we splice together from different sources (e.g., Function from type, function, and code sections?)

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
- Documentation on how to run (integration style) tests
- Keep the READMEs on Github consistent with the website, by having only a single Markdown file for both (and, e.g., linking to the website instead of duplicating in Github; or generate the website from the Github READMEs)

# Engineering, Small Features, Tests

- proper error handling in library and binary with this_error and anyhow crates
    * implement own Error type
    * replace panics with ```Result<_, wasabi::Error>```
    * derive ```From``` and ```Error``` traits
- Update syn/quote of wasm_binary crate to 1.0 (to stay up to date + remove crate duplication in wasabi crate)
- Compile wasabi to WebAssembly for Node.JS and in-browser usage (with wasm-bindgen?)
- Reduce memory allocations (see eval/perf/ heaptrack data) in hook_map::instr() and Hook::new()
    * more borrowing, less String
- fix wrong handling of ```call_indirect``` in Firefox: exported function object .name property is NOT the Wasm index -> BUG report and compare with Chrome!
- automatic (```cargo test```-able) integration tests for analyses 
    * using Wasm in Node.js
    * make sure null- or log-all-analysis run without exception
- How to handle Wasm traps?
    * (Hacky:) replace Wasm function by Wasm -> JS -> Wasm wrapper that does 
        ```
        try { 
            original_wasm_func()
        } catch (e) {
            trap_hook()
        }
        ```
