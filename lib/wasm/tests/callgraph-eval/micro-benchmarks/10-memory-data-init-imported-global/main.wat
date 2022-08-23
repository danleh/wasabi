(module
    (import "host" "print" (func $print (param i32)))
    (import "host" "data_offset" (global $data_offset i32))
    (func $main (export "main")
        i32.const 1337  ;; Some constant address.
        i32.load
        call $print
    )

    (memory $memory 1) ;; 1 page == 64KiB in size.

    ;; Initialzation of memory depends on imported global value.
    (data (global.get $data_offset) "\2A")
)