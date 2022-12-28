(module
  (type $t0 (func (param i32 i64)))
  (type $t1 (func (param i32) (result i32)))
  (func $store (type $t0) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i64.store align=4)
  (func $load (type $t1) (param $p0 i32) (result i32)
    local.get $p0
    i32.load)
  (memory $M0 1)
  (export "store" (func $store))
  (export "load" (func $load)))
