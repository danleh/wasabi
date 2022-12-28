(module
  (type $t0 (func (param i32) (result i64)))
  (type $t1 (func (param i64) (result i32)))
  (type $t2 (func (param f32) (result i32)))
  (type $t3 (func (param f64) (result i32)))
  (type $t4 (func (param f32) (result i64)))
  (type $t5 (func (param f64) (result i64)))
  (type $t6 (func (param i32) (result f32)))
  (type $t7 (func (param i64) (result f32)))
  (type $t8 (func (param i32) (result f64)))
  (type $t9 (func (param i64) (result f64)))
  (type $t10 (func (param f32) (result f64)))
  (type $t11 (func (param f64) (result f32)))
  (func $i64.extend_i32_s (type $t0) (param $p0 i32) (result i64)
    local.get $p0
    i64.extend_i32_s)
  (func $i64.extend_i32_u (type $t0) (param $p0 i32) (result i64)
    local.get $p0
    i64.extend_i32_u)
  (func $i32.wrap_i64 (type $t1) (param $p0 i64) (result i32)
    local.get $p0
    i32.wrap_i64)
  (func $i32.trunc_f32_s (type $t2) (param $p0 f32) (result i32)
    local.get $p0
    i32.trunc_f32_s)
  (func $i32.trunc_f32_u (type $t2) (param $p0 f32) (result i32)
    local.get $p0
    i32.trunc_f32_u)
  (func $i32.trunc_f64_s (type $t3) (param $p0 f64) (result i32)
    local.get $p0
    i32.trunc_f64_s)
  (func $i32.trunc_f64_u (type $t3) (param $p0 f64) (result i32)
    local.get $p0
    i32.trunc_f64_u)
  (func $i64.trunc_f32_s (type $t4) (param $p0 f32) (result i64)
    local.get $p0
    i64.trunc_f32_s)
  (func $i64.trunc_f32_u (type $t4) (param $p0 f32) (result i64)
    local.get $p0
    i64.trunc_f32_u)
  (func $i64.trunc_f64_s (type $t5) (param $p0 f64) (result i64)
    local.get $p0
    i64.trunc_f64_s)
  (func $i64.trunc_f64_u (type $t5) (param $p0 f64) (result i64)
    local.get $p0
    i64.trunc_f64_u)
  (func $i32.trunc_sat_f32_s (type $t2) (param $p0 f32) (result i32)
    local.get $p0
    i32.trunc_sat_f32_s)
  (func $i32.trunc_sat_f32_u (type $t2) (param $p0 f32) (result i32)
    local.get $p0
    i32.trunc_sat_f32_u)
  (func $i32.trunc_sat_f64_s (type $t3) (param $p0 f64) (result i32)
    local.get $p0
    i32.trunc_sat_f64_s)
  (func $i32.trunc_sat_f64_u (type $t3) (param $p0 f64) (result i32)
    local.get $p0
    i32.trunc_sat_f64_u)
  (func $i64.trunc_sat_f32_s (type $t4) (param $p0 f32) (result i64)
    local.get $p0
    i64.trunc_sat_f32_s)
  (func $i64.trunc_sat_f32_u (type $t4) (param $p0 f32) (result i64)
    local.get $p0
    i64.trunc_sat_f32_u)
  (func $i64.trunc_sat_f64_s (type $t5) (param $p0 f64) (result i64)
    local.get $p0
    i64.trunc_sat_f64_s)
  (func $i64.trunc_sat_f64_u (type $t5) (param $p0 f64) (result i64)
    local.get $p0
    i64.trunc_sat_f64_u)
  (func $f32.convert_i32_s (type $t6) (param $p0 i32) (result f32)
    local.get $p0
    f32.convert_i32_s)
  (func $f32.convert_i64_s (type $t7) (param $p0 i64) (result f32)
    local.get $p0
    f32.convert_i64_s)
  (func $f64.convert_i32_s (type $t8) (param $p0 i32) (result f64)
    local.get $p0
    f64.convert_i32_s)
  (func $f64.convert_i64_s (type $t9) (param $p0 i64) (result f64)
    local.get $p0
    f64.convert_i64_s)
  (func $f32.convert_i32_u (type $t6) (param $p0 i32) (result f32)
    local.get $p0
    f32.convert_i32_u)
  (func $f32.convert_i64_u (type $t7) (param $p0 i64) (result f32)
    local.get $p0
    f32.convert_i64_u)
  (func $f64.convert_i32_u (type $t8) (param $p0 i32) (result f64)
    local.get $p0
    f64.convert_i32_u)
  (func $f64.convert_i64_u (type $t9) (param $p0 i64) (result f64)
    local.get $p0
    f64.convert_i64_u)
  (func $f64.promote_f32 (type $t10) (param $p0 f32) (result f64)
    local.get $p0
    f64.promote_f32)
  (func $f32.demote_f64 (type $t11) (param $p0 f64) (result f32)
    local.get $p0
    f32.demote_f64)
  (func $f32.reinterpret_i32 (type $t6) (param $p0 i32) (result f32)
    local.get $p0
    f32.reinterpret_i32)
  (func $f64.reinterpret_i64 (type $t9) (param $p0 i64) (result f64)
    local.get $p0
    f64.reinterpret_i64)
  (func $i32.reinterpret_f32 (type $t2) (param $p0 f32) (result i32)
    local.get $p0
    i32.reinterpret_f32)
  (func $i64.reinterpret_f64 (type $t5) (param $p0 f64) (result i64)
    local.get $p0
    i64.reinterpret_f64)
  (export "i64.extend_i32_s" (func $i64.extend_i32_s))
  (export "i64.extend_i32_u" (func $i64.extend_i32_u))
  (export "i32.wrap_i64" (func $i32.wrap_i64))
  (export "i32.trunc_f32_s" (func $i32.trunc_f32_s))
  (export "i32.trunc_f32_u" (func $i32.trunc_f32_u))
  (export "i32.trunc_f64_s" (func $i32.trunc_f64_s))
  (export "i32.trunc_f64_u" (func $i32.trunc_f64_u))
  (export "i64.trunc_f32_s" (func $i64.trunc_f32_s))
  (export "i64.trunc_f32_u" (func $i64.trunc_f32_u))
  (export "i64.trunc_f64_s" (func $i64.trunc_f64_s))
  (export "i64.trunc_f64_u" (func $i64.trunc_f64_u))
  (export "i32.trunc_sat_f32_s" (func $i32.trunc_sat_f32_s))
  (export "i32.trunc_sat_f32_u" (func $i32.trunc_sat_f32_u))
  (export "i32.trunc_sat_f64_s" (func $i32.trunc_sat_f64_s))
  (export "i32.trunc_sat_f64_u" (func $i32.trunc_sat_f64_u))
  (export "i64.trunc_sat_f32_s" (func $i64.trunc_sat_f32_s))
  (export "i64.trunc_sat_f32_u" (func $i64.trunc_sat_f32_u))
  (export "i64.trunc_sat_f64_s" (func $i64.trunc_sat_f64_s))
  (export "i64.trunc_sat_f64_u" (func $i64.trunc_sat_f64_u))
  (export "f32.convert_i32_s" (func $f32.convert_i32_s))
  (export "f32.convert_i64_s" (func $f32.convert_i64_s))
  (export "f64.convert_i32_s" (func $f64.convert_i32_s))
  (export "f64.convert_i64_s" (func $f64.convert_i64_s))
  (export "f32.convert_i32_u" (func $f32.convert_i32_u))
  (export "f32.convert_i64_u" (func $f32.convert_i64_u))
  (export "f64.convert_i32_u" (func $f64.convert_i32_u))
  (export "f64.convert_i64_u" (func $f64.convert_i64_u))
  (export "f64.promote_f32" (func $f64.promote_f32))
  (export "f32.demote_f64" (func $f32.demote_f64))
  (export "f32.reinterpret_i32" (func $f32.reinterpret_i32))
  (export "f64.reinterpret_i64" (func $f64.reinterpret_i64))
  (export "i32.reinterpret_f32" (func $i32.reinterpret_f32))
  (export "i64.reinterpret_f64" (func $i64.reinterpret_f64)))
