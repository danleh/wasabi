;; To know the index expression one has to track data flow through functions.
;; Here: index value is the result of a function.
;; Entry point: $main
;; Precise call graph: $main -> $index, $main -> $a
;; Reachable functions: $main, $index, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        call $index
        call_indirect (param) (result)
    )
    (func $index (result i32) 
        i32.const 1
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