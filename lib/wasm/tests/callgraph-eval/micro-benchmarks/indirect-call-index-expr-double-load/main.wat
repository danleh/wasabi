;; Especially for C++ code, a double indirection through memory is common. See below.
;; Entry point: $main
;; Precise call graph: $main -> $a
;; NOT $main -> $not-reachable (that would be sound, but not precise)
;; Reachable functions: $main, $a
(module
    ;; (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 1337  ;; Some constant address.
        i32.load  ;; Load vtable pointer from address.
        i32.load  ;; Load function pointer from vtable.
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
    
    ;; Think of this as the vtable of a class, pointing to the methods of that class.
    (data $memory (i32.const 1024) "\01\00\00\00") ;; little endian 1 -> index will be 1.
    
    ;; Think of this as the allocation of an object, whose first field is the vtable.
    (data $memory (i32.const 1337) "\00\04\00\00") ;; little endian 0x400 -> points to 1024.
    
    (table $table 2 funcref)
    (elem $table (i32.const 0) $not-reachable $a)
)