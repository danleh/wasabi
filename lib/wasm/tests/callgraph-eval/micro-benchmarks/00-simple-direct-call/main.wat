(module
    (import "host" "print" (func $print (param i32)))
    (func $main
        call $func
    )
    (func $func
        i32.const 42
        call $print
    )
    (start $main)
    ;; No exported function, only direct calls.
    ;; Simplest case, should have fully precise call graph:
    ;; $main -> $func -> $print
)