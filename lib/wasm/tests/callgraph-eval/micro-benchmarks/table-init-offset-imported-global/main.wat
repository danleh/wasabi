;; The layout of the table after initialization depends on the imported global variable.
;; Entry point: $main
;; Precise call graph: $main -> $a
;; Without knowledge of the host code, one must also assume $main -> $b for soundness.
;; Reachable functions: $main, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (import "host" "element_offset" (global $element_offset i32))
    (func $main (export "main")
        i32.const 1
        call_indirect (param) (result)
    )
    (func $a
        ;; i32.const 23
        ;; call $print
    )
    (func $b
        ;; i32.const 42
        ;; call $print
    )
    (table $table 3 funcref)
    (elem $table (global.get $element_offset) $a $b)
)