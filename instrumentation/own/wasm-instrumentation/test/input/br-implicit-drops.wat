(module
    (func $start
        block
            i32.const 42
            i64.const 1337
            i32.const 1
            br_if 0
            ;; below are implicitly dropped when branching
            drop
            drop
        end
    )
  (start $start))
