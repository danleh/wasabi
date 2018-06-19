(module
    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $propagateArgToSink (param i32)
        get_local 0
        call $sink
    )

    (func $f (local $loc i32)
        i32.const 33
        set_local $loc
        get_local $loc
        call $source

        get_local $loc
        call $propagateArgToSink
    )

    (start $f)
)