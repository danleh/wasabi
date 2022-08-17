(module
    (import "host" "print" (func $print (param i32)))
    (import "host" "element_offset" (global $element_offset i32))
    (func $main
        i32.const 1
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
    (table $table 3 funcref)
    ;; Which table index gets initialized depends on the imported global variable.
    (elem $table (global.get $element_offset) $a $b)
    (start $main)
    ;; Targets of the call indirect can be subsetted by type.
    ;; Here: only $a is in table AND type-compatible with call_indirect type.
    ;; $main -> $a -> $print
    ;; NOT $main -> $b -> $print
)