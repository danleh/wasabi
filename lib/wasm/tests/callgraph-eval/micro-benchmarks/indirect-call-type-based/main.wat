;; Targets of the call indirect can be constrained by type.
;; Here: only $a is in table AND type-compatible with call_indirect type.
;; Entry point: $main
;; Precise call graph: $main -> $a
;; NOT $main -> $not-reachable
;; Reachable functions: $main, $a
(module
    (func $main (export "main")
        i32.const 0 ;; This selects the function to call.
        call_indirect (param) (result)
    )
    (func $a (param) (result))
    (func $not-reachable (param $p i32) (result))
    (table $table 2 funcref)
    (elem $table (i32.const 0) $a $not-reachable)
)