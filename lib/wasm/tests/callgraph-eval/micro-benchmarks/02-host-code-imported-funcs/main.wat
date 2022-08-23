(module
    (import "host" "print" (func $print (param i32)))
    (import "host" "imported" (func $imported))
    (func $main (export "main")
        call $imported
    )
    (func $export1 (export "export1")
        i32.const 23
        call $print
    )
    (func $export2 (export "export2")
        i32.const 42
        call $print
    )
    ;; What is the behavior of the called imported host function?
    ;; Call graph depends on this assumption.
    ;; $main -> $imported (JavaScript) -> $export2 -> $print
    ;; NOT $main -> $imported (JavaScript) -> $export1 -> $print
    ;; 2 -> 1, 1 -> [3,4], 3->0, 4->0
)