(module
    (import "imports" "output" (func $print (param i32)))

    (func $source (param i32))
    (func $sink (param i32))

    (memory 1024)  ;; TODO what's the memory_type?

    (func $f (local $locA i32) (local $locB i32)
        i32.const 7
        set_local $locA

        ;; mark locA as tainted
        get_local $locA
        call $source

        ;; copy tainted value via memory
        i32.const 42    ;; address for store
        get_local $locA ;; value for store
        i32.store

        i32.const 42    ;; address for load
        i32.load

        set_local $locB
        i32.const 75    ;; another address for store
        get_local $locB
        i32.store

        i32.const 75    ;; address for load
        i32.load

        ;; pass value to sink
        call $sink
    )

    (start $f)
)