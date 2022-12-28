(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32 i32 i32)))
  (type $t2 (func (param i32) (result i32)))
  (func $f0 (type $t0) (result i32)
    i32.const 0)
  (func $f1 (type $t0) (result i32)
    i32.const 1)
  (func $f2 (type $t0) (result i32)
    i32.const 2)
  (func $copy (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    table.copy $T0 $T0)
  (func $call (type $t2) (param $p0 i32) (result i32)
    local.get $p0
    call_indirect $T0 (type $t0))
  (table $T0 10 funcref)
  (export "copy" (func $copy))
  (export "call" (func $call))
  (elem $e0 (i32.const 0) func $f0 $f1 $f2))
