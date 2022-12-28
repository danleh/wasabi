(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32) (result i32)))
  (func $f0 (type $t0) (result i32)
    i32.const 1)
  (func $f1 (type $t0) (result i32)
    i32.const 2)
  (func $callt (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    call_indirect $T0 (type $t0))
  (table $T0 2 2 funcref)
  (export "callt" (func $callt))
  (elem $e0 (i32.const 0) func $f0 $f1))
