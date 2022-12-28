(module
  (type $t0 (func (param f64 f64) (result i32)))
  (func $eq (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.eq)
  (func $ne (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.ne)
  (func $lt (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.lt)
  (func $le (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.le)
  (func $gt (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.gt)
  (func $ge (type $t0) (param $p0 f64) (param $p1 f64) (result i32)
    local.get $p0
    local.get $p1
    f64.ge)
  (export "eq" (func $eq))
  (export "ne" (func $ne))
  (export "lt" (func $lt))
  (export "le" (func $le))
  (export "gt" (func $gt))
  (export "ge" (func $ge)))
