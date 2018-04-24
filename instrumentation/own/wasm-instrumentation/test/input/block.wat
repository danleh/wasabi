(module
    (func $f (result i32)
    	i64.const 1337
    	i32.const 23
        block
            i32.const 32
            i32.const 42
            return
            ;; i32.add
            ;; drop
        end
        return
    )
    (func $g (result)
    	i32.const 12
    	return
    )
)
