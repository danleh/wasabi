(module
    (import "host" "print" (func $print (param i32)))
    (func $export1 (export "export1")
        call $a
    )
    (func $export2 (export "export2")
        call $b
    )
    (func $a
        i32.const 23
        call $print
    )
    (func $b
        i32.const 42
        call $print
    )
    ;; Which exported function is called from the host?
    ;; Call graph depends on this assumption.
    ;; A) $export1 -> $a -> $print
    ;; B) $export2 -> $b -> $print
)