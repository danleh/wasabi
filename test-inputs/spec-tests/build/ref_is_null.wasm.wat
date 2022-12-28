(module
  (type $t0 (func (param funcref) (result i32)))
  (type $t1 (func (param externref) (result i32)))
  (type $t2 (func))
  (type $t3 (func (param externref)))
  (type $t4 (func (param i32) (result i32)))
  (func $funcref (type $t0) (param $p0 funcref) (result i32)
    local.get $p0
    ref.is_null)
  (func $externref (type $t1) (param $p0 externref) (result i32)
    local.get $p0
    ref.is_null)
  (func $f2 (type $t2))
  (func $init (type $t3) (param $p0 externref)
    i32.const 1
    local.get $p0
    table.set $T1)
  (func $deinit (type $t2)
    i32.const 1
    ref.null func
    table.set $T0
    i32.const 1
    ref.null extern
    table.set $T1)
  (func $funcref-elem (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    table.get $T0
    call $funcref)
  (func $externref-elem (type $t4) (param $p0 i32) (result i32)
    local.get $p0
    table.get $T1
    call $externref)
  (table $T0 2 funcref)
  (table $T1 2 externref)
  (export "funcref" (func $funcref))
  (export "externref" (func $externref))
  (export "init" (func $init))
  (export "deinit" (func $deinit))
  (export "funcref-elem" (func $funcref-elem))
  (export "externref-elem" (func $externref-elem))
  (elem $e0 (i32.const 1) func $f2))
