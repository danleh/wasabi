(module
  (type $t0 (func))
  (type $t1 (func (param externref)))
  (type $t2 (func (param i32) (result externref)))
  (type $t3 (func (param i32) (result funcref)))
  (type $t4 (func (param i32) (result i32)))
  (func $f0 (type $t0))
  (func $init (type $t1) (param $p0 externref)
    i32.const 1
    local.get $p0
    table.set $T0
    i32.const 2
    i32.const 1
    table.get $T1
    table.set $T1)
  (func $get-externref (type $t2) (param $p0 i32) (result externref)
    local.get $p0
    table.get $T0)
  (func $get-funcref (type $t3) (param $p0 i32) (result funcref)
    local.get $p0
    table.get $T1)
  (func $is_null-funcref (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    call $get-funcref
    ref.is_null)
  (table $T0 2 externref)
  (table $T1 3 funcref)
  (export "init" (func $init))
  (export "get-externref" (func $get-externref))
  (export "get-funcref" (func $get-funcref))
  (export "is_null-funcref" (func $is_null-funcref))
  (elem $e0 (table $T1) (i32.const 1) func $f0))
