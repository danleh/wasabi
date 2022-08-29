;; Direct calls, two levels deep.
;; Entry point: $main
;; Precise call graph: $main -> $a -> $b, $c -> $d
;; Reachable functions:
;;   $main
;;   $a
;;   $b
(module
    (func $main (export "main")
        call $a
    )
    (func $a
        call $b
    )
    (func $b)
    
    (func $c-not-reachable
        call $d-not-reachable
    )
    (func $d-not-reachable)
)