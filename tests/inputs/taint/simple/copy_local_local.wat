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

        ;; copy from locA to locB
        local.get $locA
        local.set $locB

        ;; pass locB to sink
        local.get $locB
        call $sink

        ;; sanity check: should print 5
        local.get $locB
        call $print
    )

    (start $f)
)