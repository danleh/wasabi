(module
  (type $t0 (func (param i32)))
  (func $no_dce.i32.load (type $t0) (param $p0 i32)
    local.get $p0
    i32.load
    drop)
  (func $no_dce.i32.load16_s (type $t0) (param $p0 i32)
    local.get $p0
    i32.load16_s
    drop)
  (func $no_dce.i32.load16_u (type $t0) (param $p0 i32)
    local.get $p0
    i32.load16_u
    drop)
  (func $no_dce.i32.load8_s (type $t0) (param $p0 i32)
    local.get $p0
    i32.load8_s
    drop)
  (func $no_dce.i32.load8_u (type $t0) (param $p0 i32)
    local.get $p0
    i32.load8_u
    drop)
  (func $no_dce.i64.load (type $t0) (param $p0 i32)
    local.get $p0
    i64.load
    drop)
  (func $no_dce.i64.load32_s (type $t0) (param $p0 i32)
    local.get $p0
    i64.load32_s
    drop)
  (func $no_dce.i64.load32_u (type $t0) (param $p0 i32)
    local.get $p0
    i64.load32_u
    drop)
  (func $no_dce.i64.load16_s (type $t0) (param $p0 i32)
    local.get $p0
    i64.load16_s
    drop)
  (func $no_dce.i64.load16_u (type $t0) (param $p0 i32)
    local.get $p0
    i64.load16_u
    drop)
  (func $no_dce.i64.load8_s (type $t0) (param $p0 i32)
    local.get $p0
    i64.load8_s
    drop)
  (func $no_dce.i64.load8_u (type $t0) (param $p0 i32)
    local.get $p0
    i64.load8_u
    drop)
  (func $no_dce.f32.load (type $t0) (param $p0 i32)
    local.get $p0
    f32.load
    drop)
  (func $no_dce.f64.load (type $t0) (param $p0 i32)
    local.get $p0
    f64.load
    drop)
  (memory $M0 1)
  (export "no_dce.i32.load" (func $no_dce.i32.load))
  (export "no_dce.i32.load16_s" (func $no_dce.i32.load16_s))
  (export "no_dce.i32.load16_u" (func $no_dce.i32.load16_u))
  (export "no_dce.i32.load8_s" (func $no_dce.i32.load8_s))
  (export "no_dce.i32.load8_u" (func $no_dce.i32.load8_u))
  (export "no_dce.i64.load" (func $no_dce.i64.load))
  (export "no_dce.i64.load32_s" (func $no_dce.i64.load32_s))
  (export "no_dce.i64.load32_u" (func $no_dce.i64.load32_u))
  (export "no_dce.i64.load16_s" (func $no_dce.i64.load16_s))
  (export "no_dce.i64.load16_u" (func $no_dce.i64.load16_u))
  (export "no_dce.i64.load8_s" (func $no_dce.i64.load8_s))
  (export "no_dce.i64.load8_u" (func $no_dce.i64.load8_u))
  (export "no_dce.f32.load" (func $no_dce.f32.load))
  (export "no_dce.f64.load" (func $no_dce.f64.load)))
