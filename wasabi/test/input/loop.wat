(module
    (func $start
        (local $loop i32)
        i32.const 3
        set_local $loop
        loop
            get_local $loop
            i32.const -1
            i32.add
            tee_local $loop

            i32.const 0
            i32.gt_s
            br_if 0
        end
    )
  (start $start))
