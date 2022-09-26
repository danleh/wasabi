(module
    (func $f0
        ;; Use different types for clarity/checking by wat2wasm.
        i32.const 1
        i64.const 2
        f32.const 3
        f64.const 4
        call $f1
    )
    (func $f1 (param i32 i64 f32 f64)
        nop
    )
)
