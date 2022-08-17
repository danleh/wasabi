(module
    (import "host" "print" (func $print (param i32)))
    (func $main (export "main")
        i32.const 1337  ;; Some constant address.
        i32.load
        call_indirect (param) (result)
    )
    (func $a
        i32.const 23
        call $print
    )
    (func $b
        i32.const 42
        call $print
    )
    
    (memory $memory 1) ;; 1 page == 64KiB in size.
    
    (data (i32.const 1337) "\01\00\00\00") ;; little endian 1 -> index will be 1.
    
    (table $table 3 funcref)
    (elem $table (i32.const 0) $b $a)
    ;; If the table is not exported and not mutated,
    ;; and the element initialization offset is statically known,
    ;; then the call_indirect call target depends fully on the
    ;; index expression.
    ;; Here, it is a memory load.
    ;; This brings us to pointer analysis.
    ;; In this case:
    ;; - load is from a constant address
    ;; - memory is not exported -> cannot be mutated by the host
    ;; - initialzation of memory with data section uses constant offset -> initialization of memory is known statically
    ;; - no memory write (unlikely for programs using memory) -> all memory contents are statically known
    ;; IF we make the assumption that loads of call_indirect indices read from constant memory
    ;; $main -> $a -> $print
    ;; NOT $main -> $b -> $print
)