;; The start function should be marked as reachable, even though not exported.
;; Entry point: specify none, should figure out $start itself
;; Precise call graph: $start -> $reachable
;; Reachable functions: $start, $reachable
(module
    (func $start
        call $reachable
    )
    (start $start)
    (func $reachable)
    (func $not-reachable)
)