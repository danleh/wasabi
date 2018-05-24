(module
    (func $f (result i32)
    	i64.const 1337
    	i32.const 23
        block
            ;; block starts its own stack, cannot access constants from before
            i32.const 32
            i32.const 42
            ;; return implicitly drops remaining stack values, i.e., i32.const 32
            return
            ;; dead code, but must still be well-typed, i.e., cannot have i64.add or missing drop
            i32.add
            drop
        end
        return
    )
)
