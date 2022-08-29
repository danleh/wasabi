;; With the memory init offset from the host code (1337), the results are:
;; Precise call graph: $main -> $a
;; Reachable functions: $main, $a
;; A conservative approximation without analysis of the host code can include $non-reachable as well.
(module
    ;; (import "host" "print" (func $print (param i32)))
    (import "host" "data_offset" (global $data_offset i32))
    (func $main (export "main")
        i32.const 1337  ;; Some constant address.
        i32.load
        call_indirect (param) (result)
    )
    (func $a
        ;; i32.const 23
        ;; call $print
    )
    (func $not-reachable
        ;; i32.const 42
        ;; call $print
    )

    (memory $memory 1) ;; 1 page == 64KiB in size.

    ;; Initialzation of memory depends on imported global value.
    (data (global.get $data_offset) "\01\00\00\00")

    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $a)
)