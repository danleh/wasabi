(module
  (type $t0 (func))
  (type $t1 (func (param i32) (result externref)))
  (type $t2 (func (param i32) (result funcref)))
  (type $t3 (func (param i32 externref)))
  (type $t4 (func (param i32 funcref)))
  (type $t5 (func (param i32 i32)))
  (type $t6 (func (param i32) (result i32)))
  (func $f0 (type $t0))
  (func $get-externref (type $t1) (param $p0 i32) (result externref)
    local.get $p0
    table.get $T0)
  (func $get-funcref (type $t2) (param $p0 i32) (result funcref)
    local.get $p0
    table.get $T1)
  (func $set-externref (type $t3) (param $p0 i32) (param $p1 externref)
    local.get $p0
    local.get $p1
    table.set $T0)
  (func $set-funcref (type $t4) (param $p0 i32) (param $p1 funcref)
    local.get $p0
    local.get $p1
    table.set $T1)
  (func $set-funcref-from (type $t5) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    table.get $T1
    table.set $T1)
  (func $is_null-funcref (type $t6) (param $p0 i32) (result i32)
    local.get $p0
    call $get-funcref
    ref.is_null)
  (table $T0 1 externref)
  (table $T1 2 funcref)
  (export "get-externref" (func $get-externref))
  (export "get-funcref" (func $get-funcref))
  (export "set-externref" (func $set-externref))
  (export "set-funcref" (func $set-funcref))
  (export "set-funcref-from" (func $set-funcref-from))
  (export "is_null-funcref" (func $is_null-funcref))
  (elem $e0 (table $T1) (i32.const 1) func $f0))
