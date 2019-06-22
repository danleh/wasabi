(module
  (import "imports" "output" (func $print (param i32)))
  (func $myfunction
    i32.const 42
    call $print)
  (start $myfunction)
)
