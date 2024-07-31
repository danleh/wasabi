(module
  (type $t0 (func (param i32 externref i32)))
  (type $t1 (func (param i32) (result externref)))
  (func $fill (type $t0) (param $p0 i32) (param $p1 externref) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    table.fill $T0)
  (func $fill-abbrev (type $t0) (param $p0 i32) (param $p1 externref) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    table.fill $T0)
  (func $get (type $t1) (param $p0 i32) (result externref)
    local.get $p0
    table.get $T0)
  (table $T0 10 externref)
  (export "fill" (func $fill))
  (export "fill-abbrev" (func $fill-abbrev))
  (export "get" (func $get)))
