;; Import a single function from the host (just useful for primitive tracing here).
;; Should test that imported functions are accurrately reachable.
;; Entry point: $main
;; Precise call graph: $main -> $print
;; Reachable functions: $main, $print
(module
    (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 42
        call $print
    )
    (func $not-reachable)
)