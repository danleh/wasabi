(module
    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $propagateArgToSink (param i32)
        local.get 0
        call $sink
    )

    (func $f (local $loc i32)
        i32.const 33
        local.set $loc
        local.get $loc
        call $source

        local.get $loc
        call $propagateArgToSink
    )

    (start $f)
)