;; If the table is not exported and not mutated,
;; and the element initialization offset is statically known,
;; then the call_indirect call target depends fully on the
;; index expression.
;; Here: constant 0 => entry 0 => function $a.
;; Entry points: $main
;; Precise call graph: $main -> $a
;; NOT $main -> $not-reachable
;; Reachable functions: $main, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 0
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
    (table $table 2 funcref)
    (elem $table (i32.const 0) $a $b)
)