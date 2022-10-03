(module
    (func $f
        i32.const 1
        if
            i32.const 0
            ;; this branch is taken, execution proceeds past the end
            br_table 0
        else
            br 0
        end

        i32.const 0
        if
            br 0
        else
            i32.const 0
            ;; this branch is taken, execution proceeds past the end
            br_table 0
        end
    )
    (start $f)
)
