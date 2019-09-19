(module
    (import "imports" "output" (func $print (param i32)))

    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $f (local $locA i32) (local $locB i32)
        i32.const 5
        local.set $locA

        ;; mark locA as tainted
        local.get $locA
        call $source

        ;; binary operation involving tainted locA
        local.get $locA
        i32.const 2
        i32.add

        ;; pass result to sink
        call $sink
    )

    (start $f)
)