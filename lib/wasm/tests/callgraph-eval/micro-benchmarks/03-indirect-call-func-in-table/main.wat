(module
    (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 0 ;; This selects the function to call.
        call_indirect (param) (result)
    )
    (func $a
        i32.const 23
        call $print
    )
    (func $b
        i32.const 42
        call $print
    )
    (table $table 1 funcref)
    (elem $table (i32.const 0$a)
    ;; Targets of the call indirect must be in the table.
    ;; Here: only $a is in table.
    ;; $main -> $a -> $print
    ;; NOT $main -> $b -> $print
)