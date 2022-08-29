;; To know the index expression one has to track data flow through functions.
;; Here: index value is the parameter of a function.
;; Entry point: $main
;; Precise call graph: $main -> $call_indirect -> $a
;; Reachable functions: $main, $call_indirect, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 1 ;; This is the table index value.
        call $call_indirect
    )
    (func $call_indirect (param $p i32) 
        local.get $p
        call_indirect
    )
    (func $a
        ;; i32.const 23
        ;; call $print
    )
    (func $not-reachable
        ;; i32.const 42
        ;; call $print
    )
    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $a)
)