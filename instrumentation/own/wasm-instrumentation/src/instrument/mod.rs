pub mod direct;

mod add_hooks;
pub use self::add_hooks::add_hooks;

mod convert_i64;
mod js_codegen;
mod static_info;

// TODO Idea: provide two options of connecting user analysis (i.e., client instrumentation code)
// with the instrumented binary (i.e., the "host" code + hooks + import of callbacks):
// A) "dynamic/late binding": instrument host code once, write many analyses as separate WASM
//    modules. Use the JavaScript/host WASM API to "link" the two together at runtime, i.e.,
//    export analysis functions as JS functions and provide them as import object to the
//    instrumented binary.
//    Possibly suuper slow since we go WASM <-> JS <-> WASM
// B) "static binding": Still build the two modules seperately. But use "wasm linker" and "wasm
//    inliner" to optimize the cross-language boundary away.
//
//    Step 1 WASM linker: Append all contents from the analysis module onto the host binary. Then
//    replace all imported functions (in the host binary) with code from the exported functions
//    of the analysis module IFF their names match.
//    FIXME Problem with linking: only one memory and table section allowed, what to do if two?
//    - for Memory: replace all memory operations (in particular CurrentMemory and GrowMemory) with
//                  own versions, where the second memory is placed at an offset into the same
//                  memory space as the first one.
//    - for Tables: not so easy because of the default label target
//
//    Step 2 inlining (possibly by external tool, WABT?):
//    Trivial inlining: if function body is empty (since most callbacks won't be used by the
//    analysis module), remove the call to the function + setup of function arguments
