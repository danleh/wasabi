;; If the table is not exported and not mutated,
;; and the element initialization offset is statically known,
;; then the call_indirect call target depends fully on the
;; index expression.
;; Here, it is masked to the last bit, so can only be 0 or 1.
;; Precise call graph: $main -> $a, $main -> $index
;; NOT $main -> $not-reachable
;; Reachable functions: $main, $index, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        call $index ;; Even if this index is not statically known.
        i32.const 1
        i32.and     ;; It is masked with 1, so it can only be 0 or 1.
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
    (table $table 3 funcref)
    (elem $table (i32.const 0) $a $a $not-reachable)
)