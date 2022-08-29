;; In this case the memory is mutable, so one would need to model it.
;; Entry point: $main
;; Precise call graph: $main -> $a
;; NOT $main -> $not-reachable (that would be sound, but not precise)
;; Reachable functions: $main, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 1337  ;; Some constant address.
        i32.const 1     ;; The table index.
        i32.store

        ;; Load the just stored table index again.
        i32.const 1337  
        i32.load
        call_indirect (param) (result)
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
    
    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $a)
)