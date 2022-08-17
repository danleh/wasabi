(module
    (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 0
        call_indirect
    )
    (func $export1 (export "export1")
        i32.const 23
        call $print
    )
    (func $export2 (export "export2")
        i32.const 42
        call $print
    )
    (table $table (export "table") 1 funcref)
    (elem $table (i32.const 0$export1)
    ;; Is the table mutated by the host code?
    ;; Call graph depends on this assumption.
    ;; If not table is not mutated:
    ;; $main -> $imported (JavaScript) -> $export1 -> $print
    ;; With the table mutation in main.js:
    ;; $main -> $imported (JavaScript) -> $export2 -> $print
    ;; The only functions that can be added into the table are exported ones.
)