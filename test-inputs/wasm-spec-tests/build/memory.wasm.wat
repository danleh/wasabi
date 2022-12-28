(module
  (type $t0 (func (result i32)))
  (type $t1 (func (result f64)))
  (type $t2 (func (param i32) (result i32)))
  (type $t3 (func (param i64) (result i64)))
  (func $data (type $t0) (result i32)
    i32.const 0
    i32.load8_u
    i32.const 65
    i32.eq
    i32.const 3
    i32.load8_u
    i32.const 167
    i32.eq
    i32.and
    i32.const 6
    i32.load8_u
    i32.const 0
    i32.eq
    i32.const 19
    i32.load8_u
    i32.const 0
    i32.eq
    i32.and
    i32.and
    i32.const 20
    i32.load8_u
    i32.const 87
    i32.eq
    i32.const 23
    i32.load8_u
    i32.const 77
    i32.eq
    i32.and
    i32.const 24
    i32.load8_u
    i32.const 0
    i32.eq
    i32.const 1023
    i32.load8_u
    i32.const 0
    i32.eq
    i32.and
    i32.and
    i32.and)
  (func $cast (type $t1) (result f64)
    i32.const 8
    i64.const -12345
    i64.store
    i32.const 8
    f64.load
    i64.const -12345
    f64.reinterpret_i64
    f64.eq
    if $I0
      f64.const 0x0p+0 (;=0;)
      return
    end
    i32.const 9
    i64.const 0
    i64.store align=1
    i32.const 15
    i32.const 16453
    i32.store16 align=1
    i32.const 9
    f64.load align=1)
  (func $i32_load8_s (type $t2) (param $p0 i32) (result i32)
    i32.const 8
    local.get $p0
    i32.store8
    i32.const 8
    i32.load8_s)
  (func $i32_load8_u (type $t2) (param $p0 i32) (result i32)
    i32.const 8
    local.get $p0
    i32.store8
    i32.const 8
    i32.load8_u)
  (func $i32_load16_s (type $t2) (param $p0 i32) (result i32)
    i32.const 8
    local.get $p0
    i32.store16
    i32.const 8
    i32.load16_s)
  (func $i32_load16_u (type $t2) (param $p0 i32) (result i32)
    i32.const 8
    local.get $p0
    i32.store16
    i32.const 8
    i32.load16_u)
  (func $i64_load8_s (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store8
    i32.const 8
    i64.load8_s)
  (func $i64_load8_u (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store8
    i32.const 8
    i64.load8_u)
  (func $i64_load16_s (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store16
    i32.const 8
    i64.load16_s)
  (func $i64_load16_u (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store16
    i32.const 8
    i64.load16_u)
  (func $i64_load32_s (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store32
    i32.const 8
    i64.load32_s)
  (func $i64_load32_u (type $t3) (param $p0 i64) (result i64)
    i32.const 8
    local.get $p0
    i64.store32
    i32.const 8
    i64.load32_u)
  (memory $M0 1)
  (export "data" (func $data))
  (export "cast" (func $cast))
  (export "i32_load8_s" (func $i32_load8_s))
  (export "i32_load8_u" (func $i32_load8_u))
  (export "i32_load16_s" (func $i32_load16_s))
  (export "i32_load16_u" (func $i32_load16_u))
  (export "i64_load8_s" (func $i64_load8_s))
  (export "i64_load8_u" (func $i64_load8_u))
  (export "i64_load16_s" (func $i64_load16_s))
  (export "i64_load16_u" (func $i64_load16_u))
  (export "i64_load32_s" (func $i64_load32_s))
  (export "i64_load32_u" (func $i64_load32_u))
  (data $d0 (i32.const 0) "ABC\a7D")
  (data $d1 (i32.const 20) "WASM"))
