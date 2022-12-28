(module
    (func $start
        (local $loop i32)
        i32.const 3
        local.set $loop
        loop
            ;; subtract 1 from loop counter
            local.get $loop
            i32.const -1
            i32.add
            local.tee $loop

            ;; backward branch (== continue) while $loop > 0
            i32.const 0
            i32.gt_s
            br_if 0
        end
    )
  (start $start))
