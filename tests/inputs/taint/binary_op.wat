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

        ;; binary operation involving tainted locA
        get_local $locA
        i32.const 2
        i32.add

        ;; pass result to sink
        call $sink
    )

    (start $f)
)