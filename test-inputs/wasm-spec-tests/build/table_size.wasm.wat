(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32)))
  (func $size-t0 (type $t0) (result i32)
    table.size $T0)
  (func $size-t1 (type $t0) (result i32)
    table.size $T1)
  (func $size-t2 (type $t0) (result i32)
    table.size $T2)
  (func $size-t3 (type $t0) (result i32)
    table.size $T3)
  (func $grow-t0 (type $t1) (param $p0 i32)
    ref.null extern
    local.get $p0
    table.grow $T0
    drop)
  (func $grow-t1 (type $t1) (param $p0 i32)
    ref.null extern
    local.get $p0
    table.grow $T1
    drop)
  (func $grow-t2 (type $t1) (param $p0 i32)
    ref.null extern
    local.get $p0
    table.grow $T2
    drop)
  (func $grow-t3 (type $t1) (param $p0 i32)
    ref.null extern
    local.get $p0
    table.grow $T3
    drop)
  (table $T0 0 externref)
  (table $T1 1 externref)
  (table $T2 0 2 externref)
  (table $T3 3 8 externref)
  (export "size-t0" (func $size-t0))
  (export "size-t1" (func $size-t1))
  (export "size-t2" (func $size-t2))
  (export "size-t3" (func $size-t3))
  (export "grow-t0" (func $grow-t0))
  (export "grow-t1" (func $grow-t1))
  (export "grow-t2" (func $grow-t2))
  (export "grow-t3" (func $grow-t3)))
