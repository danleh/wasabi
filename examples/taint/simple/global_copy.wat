(module
    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (global $globA i32 (i32.const 55))

    (func $f
        ;; mark globA as tainted
        global.get $globA
        call $source

        global.get $globA
        call $sink
    )

    (start $f)
)