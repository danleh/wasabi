(module
    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $propagateArgToSink (param i32)
        local.get 0
        call $sink
    )

    (func $f
        i32.const 33
        call $propagateArgToSink
    )

    (start $f)
)