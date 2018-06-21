(module
    (import "imports" "output" (func $print (param i32)))

    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (memory 1024)  ;; TODO what's the memory_type?

    (func $f (local $locA i32) (local $locB i32)
        i32.const 7
        set_local $locA

        ;; copy tainted value via memory
        i32.const 42    ;; address for store
        get_local $locA ;; value for store
        i32.store

        i32.const 42    ;; address for load
        i32.load

        ;; pass value to sink
        call $sink
    )

    (start $f)
)