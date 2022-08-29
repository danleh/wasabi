;; Targets of the call indirect must be in the table.
;; Here: only $a is in table.
;; Entry point: $main
;; Precise call graph: $main -> $a
;; NOT $main -> $not-reachable
;; Reachable functions: $main, $a
(module
    (func $main (export "main")
        i32.const 0 ;; This selects the function to call.
        call_indirect (param) (result)
    )
    (func $a)
    (func $not-reachable)
    (table $table 1 funcref)
    (elem $table (i32.const 0) $a)
)