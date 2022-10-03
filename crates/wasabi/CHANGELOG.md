This document shall list the largest breaking changes for the Wasabi binary.
In particular, breaking changes for the Wasm parsing and AST library are not captured here, since it is regarded as internal to Wasabi.

# v0.3.0 (2020-03-05)

- Update command-line interface to use clap and structopt for parsing.
    * Since the output directory is now taken as an option instead of the second argument, this is a breaking change. 

# Between 2018-11 and 2019-12

- No precise changelog kept, certainly some breaking changes, e.g., the following:
- Rename instructions and hooks to match latest Wasm spec, e.g., `get_local` to `local.get` or `i32.trunc_s/F32` to `i32.trunc_f32_s`.
- Add `op` argument to `const` hook, to be able to distinguish i32s from i64s, or floats without decimal parts from integers in JavaScript.

# v0.2.0 (2018-05-08)

- Rename project from `wasm-instrumentation` to `wasabi`.

# Before 2018-05

Not public yet.
Most features only partially working or not at all.
Also no documentation.
