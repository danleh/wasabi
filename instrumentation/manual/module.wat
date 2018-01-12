(module
  (import "imports" "output" (func $i (param i32)))
  (export "start" (func $start))
  (func $start
    i32.const 43
    call $i)
  (start $start))