(module
    (func $f
        block $b1
            block $b0
                i32.const 1
                br_table $b0 $b1 2
            end
        end
    )
    (start $f)
)
