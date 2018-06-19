(module
    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $addToTainted (param i32) (result i32) (local $locA i32)
        ;; create tainted value
        i32.const 49
        set_local $locA
        get_local $locA
        call $source

        ;; add tainted value to argument and return the result
        get_local $locA
        get_local 0
        i32.add
        return
    )

    (func $f
        i32.const -7
        call $addToTainted
        call $sink
    )

    (start $f)
)