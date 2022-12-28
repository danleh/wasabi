(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i32) (result i64)))
  (type $t2 (func (param i32) (result f32)))
  (type $t3 (func (param i32) (result f64)))
  (type $t4 (func (param i32 i32)))
  (type $t5 (func (param i32 i64)))
  (type $t6 (func (param i32 f32)))
  (type $t7 (func (param i32 f64)))
  (func $i32.load (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.load)
  (func $i64.load (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load)
  (func $f32.load (type $t2) (param $p0 i32) (result f32)
    local.get $p0
    f32.load)
  (func $f64.load (type $t3) (param $p0 i32) (result f64)
    local.get $p0
    f64.load)
  (func $i32.load8_s (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_s)
  (func $i32.load8_u (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_u)
  (func $i32.load16_s (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.load16_s)
  (func $i32.load16_u (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.load16_u)
  (func $i64.load8_s (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load8_s)
  (func $i64.load8_u (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load8_u)
  (func $i64.load16_s (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load16_s)
  (func $i64.load16_u (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load16_u)
  (func $i64.load32_s (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load32_s)
  (func $i64.load32_u (type $t1) (param $p0 i32) (result i64)
    local.get $p0
    i64.load32_u)
  (func $i32.store (type $t4) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i32.store)
  (func $i64.store (type $t5) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i64.store)
  (func $f32.store (type $t6) (param $p0 i32) (param $p1 f32)
    local.get $p0
    local.get $p1
    f32.store)
  (func $f64.store (type $t7) (param $p0 i32) (param $p1 f64)
    local.get $p0
    local.get $p1
    f64.store)
  (func $i32.store8 (type $t4) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i32.store8)
  (func $i32.store16 (type $t4) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i32.store16)
  (func $i64.store8 (type $t5) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i64.store8)
  (func $i64.store16 (type $t5) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i64.store16)
  (func $i64.store32 (type $t5) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i64.store32)
  (memory $M0 1)
  (export "i32.load" (func $i32.load))
  (export "i64.load" (func $i64.load))
  (export "f32.load" (func $f32.load))
  (export "f64.load" (func $f64.load))
  (export "i32.load8_s" (func $i32.load8_s))
  (export "i32.load8_u" (func $i32.load8_u))
  (export "i32.load16_s" (func $i32.load16_s))
  (export "i32.load16_u" (func $i32.load16_u))
  (export "i64.load8_s" (func $i64.load8_s))
  (export "i64.load8_u" (func $i64.load8_u))
  (export "i64.load16_s" (func $i64.load16_s))
  (export "i64.load16_u" (func $i64.load16_u))
  (export "i64.load32_s" (func $i64.load32_s))
  (export "i64.load32_u" (func $i64.load32_u))
  (export "i32.store" (func $i32.store))
  (export "i64.store" (func $i64.store))
  (export "f32.store" (func $f32.store))
  (export "f64.store" (func $f64.store))
  (export "i32.store8" (func $i32.store8))
  (export "i32.store16" (func $i32.store16))
  (export "i64.store8" (func $i64.store8))
  (export "i64.store16" (func $i64.store16))
  (export "i64.store32" (func $i64.store32))
  (data $d0 (i32.const 0) "abcdefgh")
  (data $d1 (i32.const 65528) "abcdefgh"))
