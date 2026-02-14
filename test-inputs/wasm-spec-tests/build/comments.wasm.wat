(module
  (type $t0 (func (result i32)))
  (func $f1 (type $t0) (result i32)
    i32.const 1
    i32.const 2
    return)
  (func $f2 (type $t0) (result i32)
    i32.const 1
    i32.const 2
    return)
  (func $f3 (type $t0) (result i32)
    i32.const 1
    i32.const 2
    return)
  (export "f1" (func $f1))
  (export "f2" (func $f2))
  (export "f3" (func $f3)))
