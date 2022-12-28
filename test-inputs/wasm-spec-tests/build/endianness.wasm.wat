(module
  (type $t0 (func (param i32 i32)))
  (type $t1 (func (param i32 i64)))
  (type $t2 (func (param i32) (result i32)))
  (type $t3 (func (param i32) (result i64)))
  (type $t4 (func (param i64) (result i64)))
  (type $t5 (func (param f32) (result f32)))
  (type $t6 (func (param f64) (result f64)))
  (func $f0 (type $t0) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    i32.store8
    local.get $p0
    i32.const 1
    i32.add
    local.get $p1
    i32.const 8
    i32.shr_u
    i32.store8)
  (func $f1 (type $t0) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    call $f0
    local.get $p0
    i32.const 2
    i32.add
    local.get $p1
    i32.const 16
    i32.shr_u
    call $f0)
  (func $f2 (type $t1) (param $p0 i32) (param $p1 i64)
    local.get $p0
    local.get $p1
    i32.wrap_i64
    call $f1
    local.get $p0
    i32.const 4
    i32.add
    local.get $p1
    i64.const 32
    i64.shr_u
    i32.wrap_i64
    call $f1)
  (func $f3 (type $t2) (param $p0 i32) (result i32)
    local.get $p0
    i32.load8_u
    local.get $p0
    i32.const 1
    i32.add
    i32.load8_u
    i32.const 8
    i32.shl
    i32.or)
  (func $f4 (type $t2) (param $p0 i32) (result i32)
    local.get $p0
    call $f3
    local.get $p0
    i32.const 2
    i32.add
    call $f3
    i32.const 16
    i32.shl
    i32.or)
  (func $f5 (type $t3) (param $p0 i32) (result i64)
    local.get $p0
    call $f4
    i64.extend_i32_u
    local.get $p0
    i32.const 4
    i32.add
    call $f4
    i64.extend_i32_u
    i64.const 32
    i64.shl
    i64.or)
  (func $i32_load16_s (type $t2) (param $p0 i32) (result i32)
    i32.const 0
    local.get $p0
    call $f0
    i32.const 0
    i32.load16_s)
  (func $i32_load16_u (type $t2) (param $p0 i32) (result i32)
    i32.const 0
    local.get $p0
    call $f0
    i32.const 0
    i32.load16_u)
  (func $i32_load (type $t2) (param $p0 i32) (result i32)
    i32.const 0
    local.get $p0
    call $f1
    i32.const 0
    i32.load)
  (func $i64_load16_s (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i32.wrap_i64
    call $f0
    i32.const 0
    i64.load16_s)
  (func $i64_load16_u (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i32.wrap_i64
    call $f0
    i32.const 0
    i64.load16_u)
  (func $i64_load32_s (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i32.wrap_i64
    call $f1
    i32.const 0
    i64.load32_s)
  (func $i64_load32_u (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i32.wrap_i64
    call $f1
    i32.const 0
    i64.load32_u)
  (func $i64_load (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    call $f2
    i32.const 0
    i64.load)
  (func $f32_load (type $t5) (param $p0 f32) (result f32)
    i32.const 0
    local.get $p0
    i32.reinterpret_f32
    call $f1
    i32.const 0
    f32.load)
  (func $f64_load (type $t6) (param $p0 f64) (result f64)
    i32.const 0
    local.get $p0
    i64.reinterpret_f64
    call $f2
    i32.const 0
    f64.load)
  (func $i32_store16 (type $t2) (param $p0 i32) (result i32)
    i32.const 0
    local.get $p0
    i32.store16
    i32.const 0
    call $f3)
  (func $i32_store (type $t2) (param $p0 i32) (result i32)
    i32.const 0
    local.get $p0
    i32.store
    i32.const 0
    call $f4)
  (func $i64_store16 (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i64.store16
    i32.const 0
    call $f3
    i64.extend_i32_u)
  (func $i64_store32 (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i64.store32
    i32.const 0
    call $f4
    i64.extend_i32_u)
  (func $i64_store (type $t4) (param $p0 i64) (result i64)
    i32.const 0
    local.get $p0
    i64.store
    i32.const 0
    call $f5)
  (func $f32_store (type $t5) (param $p0 f32) (result f32)
    i32.const 0
    local.get $p0
    f32.store
    i32.const 0
    call $f4
    f32.reinterpret_i32)
  (func $f64_store (type $t6) (param $p0 f64) (result f64)
    i32.const 0
    local.get $p0
    f64.store
    i32.const 0
    call $f5
    f64.reinterpret_i64)
  (memory $M0 1)
  (export "i32_load16_s" (func $i32_load16_s))
  (export "i32_load16_u" (func $i32_load16_u))
  (export "i32_load" (func $i32_load))
  (export "i64_load16_s" (func $i64_load16_s))
  (export "i64_load16_u" (func $i64_load16_u))
  (export "i64_load32_s" (func $i64_load32_s))
  (export "i64_load32_u" (func $i64_load32_u))
  (export "i64_load" (func $i64_load))
  (export "f32_load" (func $f32_load))
  (export "f64_load" (func $f64_load))
  (export "i32_store16" (func $i32_store16))
  (export "i32_store" (func $i32_store))
  (export "i64_store16" (func $i64_store16))
  (export "i64_store32" (func $i64_store32))
  (export "i64_store" (func $i64_store))
  (export "f32_store" (func $f32_store))
  (export "f64_store" (func $f64_store)))
