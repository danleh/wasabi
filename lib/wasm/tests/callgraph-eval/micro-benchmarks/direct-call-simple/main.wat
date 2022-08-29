;; Only direct calls. Simplest case.
;; Entry point: $main
;; Fully precise call graph (1 edge):
;;   $main -> $reachable
;; Reachable functions:
;;   $main (by construction, if it's the entry point)
;;   $reachable
(module
    (func $main (export "main")
        call $reachable
    )
    (func $reachable)
    (func $not-reachable)
)