(module
  (import "imports" "output" (func $i (param i32)))
  (func $start
    i32.const 1337
    call $i)
  (start $start)
)