;; If the table is not exported and not mutated,
;; and the element initialization offset is statically known,
;; then the call_indirect call target depends fully on the index expression.
;; Here: it is passed via a local variable.
;; Entry points: $main
;; Precise call graph: $main -> $b
;; NOT $main -> $not-reachable
;; Reachable functions: $main, $b
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        (local $l i32)
        i32.const 1
        local.set $l
        local.get $l
        call_indirect (param) (result)
    )
    (func $not-reachable
        ;; i32.const 42
        ;; call $print
    )
    (func $b
        ;; i32.const 23
        ;; call $print
    )
    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $b)
)