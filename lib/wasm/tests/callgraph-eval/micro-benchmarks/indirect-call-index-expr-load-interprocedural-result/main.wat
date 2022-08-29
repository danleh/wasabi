;; Similar to indirect-call-index-expr-load-const-addr, but the pointer/address comes from another
;; function (i.e., requires interprocedural analysis).
;; Entry point: $main
;; Precise call graph: $main -> $index, $main -> $a
;; Reachable functions: $main, $call_indirect, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        call $index
        i32.load
        call_indirect (param) (result)
    )
    (func $index (result i32) 
        i32.const 1337 ;; This is the pointer/address of the table index in memory.
    )
    (func $a
        ;; i32.const 23
        ;; call $print
    )
    (func $not-reachable
        ;; i32.const 42
        ;; call $print
    )
    
    (memory $memory 1) ;; 1 page == 64KiB in size.
    (data $memory (i32.const 1337) "\01\00\00\00") ;; little endian 1 -> index will be 1.
    
    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $a)
)