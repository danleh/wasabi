(module
    (import "imports" "output" (func $print (param i32)))

    (func $source (param i32))
    (func $sink (param i32))

    (func $f (local $locA i32) (local $locB i32)
        i32.const 5
        set_local $locA

        ;; mark locA as tainted
        get_local $locA
        call $source

        ;; put fresh, untainted value into locB
        i32.const 23
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