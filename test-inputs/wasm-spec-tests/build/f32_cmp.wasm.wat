(module
  (type $t0 (func (param f32 f32) (result i32)))
  (func $eq (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.eq)
  (func $ne (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.ne)
  (func $lt (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.lt)
  (func $le (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.le)
  (func $gt (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.gt)
  (func $ge (type $t0) (param $p0 f32) (param $p1 f32) (result i32)
    local.get $p0
    local.get $p1
    f32.ge)
  (export "eq" (func $eq))
  (export "ne" (func $ne))
  (export "lt" (func $lt))
  (export "le" (func $le))
  (export "gt" (func $gt))
  (export "ge" (func $ge)))
