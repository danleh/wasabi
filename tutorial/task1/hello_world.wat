(module
  (import "imports" "output" (func $print (param i32)))
  (func $somefun
    i32.const 42
    call $print)
  (export "somefun" (func $somefun))
)
