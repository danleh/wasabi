(module
    (import "imports" "output" (func $print (param i32)))

    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $f (local $locA i32) (local $locB i32)
        i32.const 5
        set_local $locA

        ;; mark locA as tainted
        get_local $locA
        call $source

        ;; copy from locA to locB
        get_local $locA
        set_local $locB

        ;; pass locB to sink
        get_local $locB
        call $sink

        ;; sanity check: should print 5
        get_local $locB
        call $print
    )

    (start $f)
)