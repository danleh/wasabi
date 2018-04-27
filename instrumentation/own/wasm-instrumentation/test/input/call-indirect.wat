(module
    (table $table 1 anyfunc)
    (func $f
        i32.const 0
        call_indirect
    )
    (func $indirect
        nop
    )
    (export "f" (func $f))
    (elem (i32.const 0) $indirect)
    (start $f)
)
