(module
  (type $t0 (func))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i64)))
  (type $t3 (func (param f32)))
  (type $t4 (func (param f64)))
  (type $t5 (func (param i64 f32 f64 i32 i32)))
  (type $t6 (func (param i64 f32 f64 i32 i32) (result i64)))
  (func $type-local-i32 (type $t0)
    (local $l0 i32)
    i32.const 0
    local.set $l0)
  (func $type-local-i64 (type $t0)
    (local $l0 i64)
    i64.const 0
    local.set $l0)
  (func $type-local-f32 (type $t0)
    (local $l0 f32)
    f32.const 0x0p+0 (;=0;)
    local.set $l0)
  (func $type-local-f64 (type $t0)
    (local $l0 f64)
    f64.const 0x0p+0 (;=0;)
    local.set $l0)
  (func $type-param-i32 (type $t1) (param $p0 i32)
    i32.const 10
    local.set $p0)
  (func $type-param-i64 (type $t2) (param $p0 i64)
    i64.const 11
    local.set $p0)
  (func $type-param-f32 (type $t3) (param $p0 f32)
    f32.const 0x1.633334p+3 (;=11.1;)
    local.set $p0)
  (func $type-param-f64 (type $t4) (param $p0 f64)
    f64.const 0x1.8666666666666p+3 (;=12.2;)
    local.set $p0)
  (func $type-mixed (type $t5) (param $p0 i64) (param $p1 f32) (param $p2 f64) (param $p3 i32) (param $p4 i32)
    (local $l5 f32) (local $l6 i64) (local $l7 i64) (local $l8 f64)
    i64.const 0
    local.set $p0
    f32.const 0x0p+0 (;=0;)
    local.set $p1
    f64.const 0x0p+0 (;=0;)
    local.set $p2
    i32.const 0
    local.set $p3
    i32.const 0
    local.set $p4
    f32.const 0x0p+0 (;=0;)
    local.set $l5
    i64.const 0
    local.set $l6
    i64.const 0
    local.set $l7
    f64.const 0x0p+0 (;=0;)
    local.set $l8)
  (func $write (type $t6) (param $p0 i64) (param $p1 f32) (param $p2 f64) (param $p3 i32) (param $p4 i32) (result i64)
    (local $l5 f32) (local $l6 i64) (local $l7 i64) (local $l8 f64)
    f32.const -0x1.333334p-2 (;=-0.3;)
    local.set $p1
    i32.const 40
    local.set $p3
    i32.const -7
    local.set $p4
    f32.const 0x1.6p+2 (;=5.5;)
    local.set $l5
    i64.const 6
    local.set $l6
    f64.const 0x1p+3 (;=8;)
    local.set $l8
    local.get $p0
    f64.convert_i64_u
    local.get $p1
    f64.promote_f32
    local.get $p2
    local.get $p3
    f64.convert_i32_u
    local.get $p4
    f64.convert_i32_s
    local.get $l5
    f64.promote_f32
    local.get $l6
    f64.convert_i64_u
    local.get $l7
    f64.convert_i64_u
    local.get $l8
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    i64.trunc_f64_s)
  (func $as-block-value (type $t1) (param $p0 i32)
    block $B0
      i32.const 1
      local.set $p0
    end)
  (func $as-loop-value (type $t1) (param $p0 i32)
    loop $L0
      i32.const 3
      local.set $p0
    end)
  (func $as-br-value (type $t1) (param $p0 i32)
    block $B0
      i32.const 9
      local.set $p0
      br $B0
    end)
  (func $as-br_if-value (type $t1) (param $p0 i32)
    block $B0
      i32.const 8
      local.set $p0
      i32.const 1
      br_if $B0
    end)
  (func $as-br_if-value-cond (type $t1) (param $p0 i32)
    block $B0
      i32.const 6
      i32.const 9
      local.set $p0
      br_if $B0
    end)
  (func $as-br_table-value (type $t1) (param $p0 i32)
    block $B0
      i32.const 10
      local.set $p0
      i32.const 1
      br_table $B0
    end)
  (func $as-return-value (type $t1) (param $p0 i32)
    i32.const 7
    local.set $p0
    return)
  (func $as-if-then (type $t1) (param $p0 i32)
    local.get $p0
    if $I0
      i32.const 3
      local.set $p0
    end)
  (func $as-if-else (type $t1) (param $p0 i32)
    local.get $p0
    if $I0
    else
      i32.const 1
      local.set $p0
    end)
  (export "type-local-i32" (func $type-local-i32))
  (export "type-local-i64" (func $type-local-i64))
  (export "type-local-f32" (func $type-local-f32))
  (export "type-local-f64" (func $type-local-f64))
  (export "type-param-i32" (func $type-param-i32))
  (export "type-param-i64" (func $type-param-i64))
  (export "type-param-f32" (func $type-param-f32))
  (export "type-param-f64" (func $type-param-f64))
  (export "type-mixed" (func $type-mixed))
  (export "write" (func $write))
  (export "as-block-value" (func $as-block-value))
  (export "as-loop-value" (func $as-loop-value))
  (export "as-br-value" (func $as-br-value))
  (export "as-br_if-value" (func $as-br_if-value))
  (export "as-br_if-value-cond" (func $as-br_if-value-cond))
  (export "as-br_table-value" (func $as-br_table-value))
  (export "as-return-value" (func $as-return-value))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else)))
