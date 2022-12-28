(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func (result i32)))
  (type $t2 (func (result i64)))
  (type $t3 (func (result f32)))
  (type $t4 (func (result f64)))
  (type $t5 (func (param i32) (result i32)))
  (type $t6 (func (param i64) (result i64)))
  (type $t7 (func (param f32) (result f32)))
  (type $t8 (func (param f64) (result f64)))
  (type $t9 (func (param i64 f32 f64 i32 i32)))
  (type $t10 (func (param i64 f32 f64 i32 i32) (result i64)))
  (type $t11 (func (param i64 f32 f64 i32 i32) (result f64)))
  (type $t12 (func))
  (type $t13 (func (param i32)))
  (type $t14 (func (param i32 i32) (result i32)))
  (type $t15 (func (param i64) (result i32)))
  (func $type-local-i32 (type $t1) (result i32)
    (local $l0 i32)
    i32.const 0
    local.tee $l0)
  (func $type-local-i64 (type $t2) (result i64)
    (local $l0 i64)
    i64.const 0
    local.tee $l0)
  (func $type-local-f32 (type $t3) (result f32)
    (local $l0 f32)
    f32.const 0x0p+0 (;=0;)
    local.tee $l0)
  (func $type-local-f64 (type $t4) (result f64)
    (local $l0 f64)
    f64.const 0x0p+0 (;=0;)
    local.tee $l0)
  (func $type-param-i32 (type $t5) (param $p0 i32) (result i32)
    i32.const 10
    local.tee $p0)
  (func $type-param-i64 (type $t6) (param $p0 i64) (result i64)
    i64.const 11
    local.tee $p0)
  (func $type-param-f32 (type $t7) (param $p0 f32) (result f32)
    f32.const 0x1.633334p+3 (;=11.1;)
    local.tee $p0)
  (func $type-param-f64 (type $t8) (param $p0 f64) (result f64)
    f64.const 0x1.8666666666666p+3 (;=12.2;)
    local.tee $p0)
  (func $type-mixed (type $t9) (param $p0 i64) (param $p1 f32) (param $p2 f64) (param $p3 i32) (param $p4 i32)
    (local $l5 f32) (local $l6 i64) (local $l7 i64) (local $l8 f64)
    i64.const 0
    local.tee $p0
    i64.eqz
    drop
    f32.const 0x0p+0 (;=0;)
    local.tee $p1
    f32.neg
    drop
    f64.const 0x0p+0 (;=0;)
    local.tee $p2
    f64.neg
    drop
    i32.const 0
    local.tee $p3
    i32.eqz
    drop
    i32.const 0
    local.tee $p4
    i32.eqz
    drop
    f32.const 0x0p+0 (;=0;)
    local.tee $l5
    f32.neg
    drop
    i64.const 0
    local.tee $l6
    i64.eqz
    drop
    i64.const 0
    local.tee $l7
    i64.eqz
    drop
    f64.const 0x0p+0 (;=0;)
    local.tee $l8
    f64.neg
    drop)
  (func $write (type $t10) (param $p0 i64) (param $p1 f32) (param $p2 f64) (param $p3 i32) (param $p4 i32) (result i64)
    (local $l5 f32) (local $l6 i64) (local $l7 i64) (local $l8 f64)
    f32.const -0x1.333334p-2 (;=-0.3;)
    local.tee $p1
    drop
    i32.const 40
    local.tee $p3
    drop
    i32.const -7
    local.tee $p4
    drop
    f32.const 0x1.6p+2 (;=5.5;)
    local.tee $l5
    drop
    i64.const 6
    local.tee $l6
    drop
    f64.const 0x1p+3 (;=8;)
    local.tee $l8
    drop
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
  (func $result (type $t11) (param $p0 i64) (param $p1 f32) (param $p2 f64) (param $p3 i32) (param $p4 i32) (result f64)
    (local $l5 f32) (local $l6 i64) (local $l7 i64) (local $l8 f64)
    i64.const 1
    local.tee $p0
    f64.convert_i64_u
    f32.const 0x1p+1 (;=2;)
    local.tee $p1
    f64.promote_f32
    f64.const 0x1.a666666666666p+1 (;=3.3;)
    local.tee $p2
    i32.const 4
    local.tee $p3
    f64.convert_i32_u
    i32.const 5
    local.tee $p4
    f64.convert_i32_s
    f32.const 0x1.6p+2 (;=5.5;)
    local.tee $l5
    f64.promote_f32
    i64.const 6
    local.tee $l6
    f64.convert_i64_u
    i64.const 0
    local.tee $l7
    f64.convert_i64_u
    f64.const 0x1p+3 (;=8;)
    local.tee $l8
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add
    f64.add)
  (func $f11 (type $t12))
  (func $as-block-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      local.tee $p0
      call $f11
    end)
  (func $as-block-mid (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      call $f11
      i32.const 1
      local.tee $p0
      call $f11
    end)
  (func $as-block-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      call $f11
      call $f11
      i32.const 1
      local.tee $p0
    end)
  (func $as-loop-first (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      i32.const 3
      local.tee $p0
      call $f11
    end)
  (func $as-loop-mid (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      call $f11
      i32.const 4
      local.tee $p0
      call $f11
    end)
  (func $as-loop-last (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      call $f11
      call $f11
      i32.const 5
      local.tee $p0
    end)
  (func $as-br-value (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 9
      local.tee $p0
      br $B0
    end)
  (func $as-br_if-cond (type $t13) (param $p0 i32)
    block $B0
      i32.const 1
      local.tee $p0
      br_if $B0
    end)
  (func $as-br_if-value (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 8
      local.tee $p0
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 9
      local.tee $p0
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t13) (param $p0 i32)
    block $B0
      i32.const 0
      local.tee $p0
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 10
      local.tee $p0
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-index (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 11
      local.tee $p0
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-return-value (type $t5) (param $p0 i32) (result i32)
    i32.const 7
    local.tee $p0
    return)
  (func $as-if-cond (type $t5) (param $p0 i32) (result i32)
    i32.const 2
    local.tee $p0
    if $I0 (result i32)
      i32.const 0
    else
      i32.const 1
    end)
  (func $as-if-then (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 3
      local.tee $p0
    else
      local.get $p0
    end)
  (func $as-if-else (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      local.get $p0
    else
      i32.const 4
      local.tee $p0
    end)
  (func $as-select-first (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const 5
    local.tee $p0
    local.get $p0
    local.get $p1
    select)
  (func $as-select-second (type $t14) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.const 6
    local.tee $p0
    local.get $p1
    select)
  (func $as-select-cond (type $t5) (param $p0 i32) (result i32)
    i32.const 0
    i32.const 1
    i32.const 7
    local.tee $p0
    select)
  (func $f32 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t5) (param $p0 i32) (result i32)
    i32.const 12
    local.tee $p0
    i32.const 2
    i32.const 3
    call $f32)
  (func $as-call-mid (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 13
    local.tee $p0
    i32.const 3
    call $f32)
  (func $as-call-last (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    i32.const 14
    local.tee $p0
    call $f32)
  (func $as-call_indirect-first (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    local.tee $p0
    i32.const 2
    i32.const 3
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-mid (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    local.tee $p0
    i32.const 3
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-last (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    i32.const 3
    local.tee $p0
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-index (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    i32.const 3
    i32.const 0
    local.tee $p0
    call_indirect $T0 (type $t0))
  (func $as-local.set-value (type $t12)
    (local $l0 i32)
    i32.const 1
    local.tee $l0
    local.set $l0)
  (func $as-local.tee-value (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    local.tee $p0
    local.tee $p0)
  (func $as-global.set-value (type $t12)
    (local $l0 i32)
    i32.const 1
    local.tee $l0
    global.set $g0)
  (func $as-load-address (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    local.tee $p0
    i32.load)
  (func $as-loadN-address (type $t5) (param $p0 i32) (result i32)
    i32.const 3
    local.tee $p0
    i32.load8_s)
  (func $as-store-address (type $t13) (param $p0 i32)
    i32.const 30
    local.tee $p0
    i32.const 7
    i32.store)
  (func $as-store-value (type $t13) (param $p0 i32)
    i32.const 2
    i32.const 1
    local.tee $p0
    i32.store)
  (func $as-storeN-address (type $t13) (param $p0 i32)
    i32.const 1
    local.tee $p0
    i32.const 7
    i32.store8)
  (func $as-storeN-value (type $t13) (param $p0 i32)
    i32.const 2
    i32.const 1
    local.tee $p0
    i32.store16)
  (func $as-unary-operand (type $t7) (param $p0 f32) (result f32)
    f32.const nan:0xf1e2 (;=nan;)
    local.tee $p0
    f32.neg)
  (func $as-binary-left (type $t5) (param $p0 i32) (result i32)
    i32.const 3
    local.tee $p0
    i32.const 10
    i32.add)
  (func $as-binary-right (type $t5) (param $p0 i32) (result i32)
    i32.const 10
    i32.const 4
    local.tee $p0
    i32.sub)
  (func $as-test-operand (type $t5) (param $p0 i32) (result i32)
    i32.const 0
    local.tee $p0
    i32.eqz)
  (func $as-compare-left (type $t5) (param $p0 i32) (result i32)
    i32.const 43
    local.tee $p0
    i32.const 10
    i32.le_s)
  (func $as-compare-right (type $t5) (param $p0 i32) (result i32)
    i32.const 10
    i32.const 42
    local.tee $p0
    i32.ne)
  (func $as-convert-operand (type $t15) (param $p0 i64) (result i32)
    i64.const 41
    local.tee $p0
    i32.wrap_i64)
  (func $as-memory.grow-size (type $t5) (param $p0 i32) (result i32)
    i32.const 40
    local.tee $p0
    memory.grow)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 0))
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
  (export "result" (func $result))
  (export "as-block-first" (func $as-block-first))
  (export "as-block-mid" (func $as-block-mid))
  (export "as-block-last" (func $as-block-last))
  (export "as-loop-first" (func $as-loop-first))
  (export "as-loop-mid" (func $as-loop-mid))
  (export "as-loop-last" (func $as-loop-last))
  (export "as-br-value" (func $as-br-value))
  (export "as-br_if-cond" (func $as-br_if-cond))
  (export "as-br_if-value" (func $as-br_if-value))
  (export "as-br_if-value-cond" (func $as-br_if-value-cond))
  (export "as-br_table-index" (func $as-br_table-index))
  (export "as-br_table-value" (func $as-br_table-value))
  (export "as-br_table-value-index" (func $as-br_table-value-index))
  (export "as-return-value" (func $as-return-value))
  (export "as-if-cond" (func $as-if-cond))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-second" (func $as-select-second))
  (export "as-select-cond" (func $as-select-cond))
  (export "as-call-first" (func $as-call-first))
  (export "as-call-mid" (func $as-call-mid))
  (export "as-call-last" (func $as-call-last))
  (export "as-call_indirect-first" (func $as-call_indirect-first))
  (export "as-call_indirect-mid" (func $as-call_indirect-mid))
  (export "as-call_indirect-last" (func $as-call_indirect-last))
  (export "as-call_indirect-index" (func $as-call_indirect-index))
  (export "as-local.set-value" (func $as-local.set-value))
  (export "as-local.tee-value" (func $as-local.tee-value))
  (export "as-global.set-value" (func $as-global.set-value))
  (export "as-load-address" (func $as-load-address))
  (export "as-loadN-address" (func $as-loadN-address))
  (export "as-store-address" (func $as-store-address))
  (export "as-store-value" (func $as-store-value))
  (export "as-storeN-address" (func $as-storeN-address))
  (export "as-storeN-value" (func $as-storeN-value))
  (export "as-unary-operand" (func $as-unary-operand))
  (export "as-binary-left" (func $as-binary-left))
  (export "as-binary-right" (func $as-binary-right))
  (export "as-test-operand" (func $as-test-operand))
  (export "as-compare-left" (func $as-compare-left))
  (export "as-compare-right" (func $as-compare-right))
  (export "as-convert-operand" (func $as-convert-operand))
  (export "as-memory.grow-size" (func $as-memory.grow-size))
  (elem $e0 (i32.const 0) func $f32))
