;; Is the table mutated by the host code?
;; Call graph depends on this assumption.
;; If the table is not mutated:
;;   $main -> $imported (JavaScript) -> $export1
;; With the table mutation in main.js:
;;   $main -> $imported (JavaScript) -> $export2
;; In this particular program, the most precise call graph should include knowledge about the host code, so it is:
;;   $main -> $imported (JavaScript) -> $export2
;; The only functions that can be added into the table by the host, are the ones that are reachable from the host.
;; Entry point: $main, $export1, $export2
;; Reachable functions:
;;   $main, $export1, $export2
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 0
        call_indirect
    )
    (func $export1 (export "export1")
        ;; i32.const 23
        ;; call $print
    )
    (func $export2 (export "export2")
        ;; i32.const 42
        ;; call $print
    )
    (func $not-reachable)
    (table $table (export "table") 1 funcref)
    (elem $table (i32.const 0) $export1)
)