;; What is the behavior of the called imported host function? Which functions can it call?
;; Call graph depends on this assumption.
;; Entry point: $main
;; Precise call graph: 
;;   $main -> $imported (JavaScript) -> $export1 -> $a
;; Without analyzing the host code, one must also assume: 
;;   $main -> $imported (JavaScript) -> $export2 -> $b
;; (but let us not include that here in the precise call graph, because we assume an all-knowing host code analysis)
;; Reachable functions: 
;;   $main
;;   $imported
;;   $export1
;;   $a
(module
    (import "host" "imported" (func $imported))
    (func $main (export "main")
        call $imported
    )
    (func $export1 (export "export1")
        call $a
    )
    (func $export2 (export "export2")
        call $b
    )

    (func $a)
    (func $b)
    (func $not-reachable)
)