(module
    (func $start
        f32.const 1.0
        f32.const 0.0
        i32.const 0
        select
        drop

        i64.const 23
        i64.const 1337
        i32.const 1
        select
        drop
    )
    (start $start)
)