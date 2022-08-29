;; Which functions are reachable from the host? All functions that are present in an exported table.
;; Entry point: $main
;; Precise call graph: $main -> $imported (JavaScript) -> $reachable, $not-reachable-c -> $not-reachable-d
;; Reachable functions: $main, $imported, $reachable
(module
    ;; Uncomment the following lines to show that $reachable can be called from the host!
    ;; (import "host" "print" (func $print (param i32)))
    (import "host" "imported" (func $imported))
    (func $main (export "main")
        call $imported
    )
    (func $reachable
        ;; i32.const 42
        ;; call $print
    )

    (func $not-reachable-c
        call $not-reachable-d)
    (func $not-reachable-d)

    (table $table (export "table") 1 funcref)
    (elem $table (i32.const 0) $reachable)
)