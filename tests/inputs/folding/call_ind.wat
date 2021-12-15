(module
    (table $table 1 anyfunc)
    (func $f
        i32.const 0
        call_indirect
    )
    (func $indirect nop)
    (elem (i32.const 0) $indirect)
    (export "f" (func $f))
)
