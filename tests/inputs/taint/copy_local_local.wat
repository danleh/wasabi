(module
    (import "console" "log" (func $print (param i32)))
    (func $f (local $locA i32) (local $locB i32)
        i32.const 5
        set_local $locA


        i32.const 42
        call $print
    )
    (start $f)
)