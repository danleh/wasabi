(module
    (table $table 1 anyfunc)
    (func $f
        i32.const 0
        call_indirect
    )
    (func $indirectly
        nop
    )
    (export "indirectTest" (func $indirectly))
    (elem (i32.const 0) $indirectly)
    (start $f)
)
