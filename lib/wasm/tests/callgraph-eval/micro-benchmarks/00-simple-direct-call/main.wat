(module
    (import "host" "print" (func $print (param i32))) 0
    (func $main (export "main") 
        call $func
    )
    (func $func 
        i32.const 42
        call $print
    )
    ;; No exported function, only direct calls.
    ;; Simplest case, should have fully precise call graph:
    ;; $main -> $func -> $print
)
