;; If the table is imported, it is obviously reachable from the outside.
;; Hence any function in the element section becomes reachable too.
;; Precise call graph: $main -> $imported -> $reachable, $not-reachable-c -> $not-reachable-d
;; Reachable functions: $main, $imported, $reachable
(module
    ;; (import "host" "print" (func $print (param i32)))
    (import "host" "imported" (func $imported))
    (import "host" "table" (table $table 1 funcref))
    (func $main (export "main")
        call $imported
    )
    (func $reachable ;; This is reachable via the imported table!
    )

    (func $not-reachable-c ;; This is not reachable, because the host has no way of obtaining a reference to that function and putting it into the table.
        ;; i32.const 42
        ;; call $print
        call $not-reachable-d
    )
    (func $not-reachable-d)

    ;; Adds $a to the imported table.
    (elem $table (i32.const 0) $reachable)
)