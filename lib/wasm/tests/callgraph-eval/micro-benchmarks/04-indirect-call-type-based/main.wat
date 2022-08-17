(module
    (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 0 ;; This selects the function to call.
        call_indirect (param) (result)
    )
    (func $a (param) (result)
        i32.const 23
        call $print
    )
    (func $b (param $p i32) (result)
        local.get $p
        call $print
    )
    (table $table 2 funcref)
    (elem $table (i32.const 0$a $b)
    ;; Targets of the call indirect can be subsetted by type.
    ;; Here: only $a is in table AND type-compatible with call_indirect type.
    ;; $main -> $a -> $print
    ;; NOT $main -> $b -> $print
)