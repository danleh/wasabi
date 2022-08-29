;; The WASI standard defines export named _start.
;; Entry point: give none, should figure out $start itself (TODO or should it!? Can we assume tools are WASI-aware?)
;; Precise call graph: _start = $start -> $reachable
;; Reachable functions: $start, $reachable
(module
    (func $start (export "_start")
        call $reachable
    )
    (func $reachable)
    (func $not-reachable)

    ;; Demanded by WASI: always need a memory.
    (memory $memory (export "memory") 0)
)