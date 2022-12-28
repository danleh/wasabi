(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (result i64)))
  (type $t4 (func (result f32)))
  (type $t5 (func (result f64)))
  (type $t6 (func (param i32 i32) (result i32)))
  (func $f0 (type $t1))
  (func $type-i32 (type $t1)
    return
    i32.ctz
    drop)
  (func $type-i64 (type $t1)
    return
    i64.ctz
    drop)
  (func $type-f32 (type $t1)
    return
    f32.neg
    drop)
  (func $type-f64 (type $t1)
    return
    f64.neg
    drop)
  (func $type-i32-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      return
      i32.ctz
    end)
  (func $type-i64-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 2
      return
      i64.ctz
    end)
  (func $type-f32-value (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.8p+1 (;=3;)
      return
      f32.neg
    end)
  (func $type-f64-value (type $t5) (result f64)
    block $B0 (result f64)
      f64.const 0x1p+2 (;=4;)
      return
      f64.neg
    end)
  (func $nullary (type $t1)
    return)
  (func $unary (type $t5) (result f64)
    f64.const 0x1.8p+1 (;=3;)
    return)
  (func $as-func-first (type $t2) (result i32)
    i32.const 1
    return
    i32.const 2)
  (func $as-func-mid (type $t2) (result i32)
    call $f0
    i32.const 2
    return
    i32.const 3)
  (func $as-func-last (type $t1)
    nop
    call $f0
    return)
  (func $as-func-value (type $t2) (result i32)
    nop
    call $f0
    i32.const 3
    return)
  (func $as-block-first (type $t1)
    block $B0
      return
      call $f0
    end)
  (func $as-block-mid (type $t1)
    block $B0
      call $f0
      return
      call $f0
    end)
  (func $as-block-last (type $t1)
    block $B0
      nop
      call $f0
      return
    end)
  (func $as-block-value (type $t2) (result i32)
    block $B0 (result i32)
      nop
      call $f0
      i32.const 2
      return
    end)
  (func $as-loop-first (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 3
      return
      i32.const 2
    end)
  (func $as-loop-mid (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 4
      return
      i32.const 2
    end)
  (func $as-loop-last (type $t2) (result i32)
    loop $L0 (result i32)
      nop
      call $f0
      i32.const 5
      return
    end)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 9
      return
      br $B0
    end)
  (func $as-br_if-cond (type $t1)
    block $B0
      return
      br_if $B0
    end)
  (func $as-br_if-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 8
      return
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 9
      return
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t3) (result i64)
    block $B0
      i64.const 9
      return
      br_table $B0 $B0 $B0
    end
    i64.const -1)
  (func $as-br_table-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 10
      return
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-index (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 11
      return
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-return-value (type $t3) (result i64)
    i64.const 7
    return
    return)
  (func $as-if-cond (type $t2) (result i32)
    i32.const 2
    return
    if $I0 (result i32)
      i32.const 0
    else
      i32.const 1
    end)
  (func $as-if-then (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 3
      return
    else
      local.get $p1
    end)
  (func $as-if-else (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      local.get $p1
    else
      i32.const 4
      return
    end)
  (func $as-select-first (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const 5
    return
    local.get $p0
    local.get $p1
    select)
  (func $as-select-second (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.const 6
    return
    local.get $p1
    select)
  (func $as-select-cond (type $t2) (result i32)
    i32.const 0
    i32.const 1
    i32.const 7
    return
    select)
  (func $f36 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t2) (result i32)
    i32.const 12
    return
    i32.const 2
    i32.const 3
    call $f36)
  (func $as-call-mid (type $t2) (result i32)
    i32.const 1
    i32.const 13
    return
    i32.const 3
    call $f36)
  (func $as-call-last (type $t2) (result i32)
    i32.const 1
    i32.const 2
    i32.const 14
    return
    call $f36)
  (func $as-call_indirect-func (type $t2) (result i32)
    i32.const 20
    return
    i32.const 1
    i32.const 2
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-first (type $t2) (result i32)
    i32.const 0
    i32.const 21
    return
    i32.const 2
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-mid (type $t2) (result i32)
    i32.const 0
    i32.const 1
    i32.const 22
    return
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-last (type $t2) (result i32)
    i32.const 0
    i32.const 1
    i32.const 2
    i32.const 23
    return
    call_indirect $T0 (type $t0))
  (func $as-local.set-value (type $t2) (result i32)
    (local $l0 f32)
    i32.const 17
    return
    local.set $l0
    i32.const -1)
  (func $as-local.tee-value (type $t2) (result i32)
    (local $l0 i32)
    i32.const 1
    return
    local.tee $l0)
  (func $as-global.set-value (type $t2) (result i32)
    i32.const 1
    return
    global.set $g0)
  (func $as-load-address (type $t4) (result f32)
    f32.const 0x1.b33334p+0 (;=1.7;)
    return
    f32.load)
  (func $as-loadN-address (type $t3) (result i64)
    i64.const 30
    return
    i64.load8_s)
  (func $as-store-address (type $t2) (result i32)
    i32.const 30
    return
    f64.const 0x1.cp+2 (;=7;)
    f64.store
    i32.const -1)
  (func $as-store-value (type $t2) (result i32)
    i32.const 2
    i32.const 31
    return
    i64.store
    i32.const -1)
  (func $as-storeN-address (type $t2) (result i32)
    i32.const 32
    return
    i32.const 7
    i32.store8
    i32.const -1)
  (func $as-storeN-value (type $t2) (result i32)
    i32.const 2
    i32.const 33
    return
    i64.store16
    i32.const -1)
  (func $as-unary-operand (type $t4) (result f32)
    f32.const 0x1.b33334p+1 (;=3.4;)
    return
    f32.neg)
  (func $as-binary-left (type $t2) (result i32)
    i32.const 3
    return
    i32.const 10
    i32.add)
  (func $as-binary-right (type $t3) (result i64)
    i64.const 10
    i64.const 45
    return
    i64.sub)
  (func $as-test-operand (type $t2) (result i32)
    i32.const 44
    return
    i32.eqz)
  (func $as-compare-left (type $t2) (result i32)
    i32.const 43
    return
    f64.const 0x1.4p+3 (;=10;)
    f64.le)
  (func $as-compare-right (type $t2) (result i32)
    f32.const 0x1.4p+3 (;=10;)
    i32.const 42
    return
    f32.ne)
  (func $as-convert-operand (type $t2) (result i32)
    i32.const 41
    return
    i32.wrap_i64)
  (func $as-memory.grow-size (type $t2) (result i32)
    i32.const 40
    return
    memory.grow)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 0))
  (export "type-i32" (func $type-i32))
  (export "type-i64" (func $type-i64))
  (export "type-f32" (func $type-f32))
  (export "type-f64" (func $type-f64))
  (export "type-i32-value" (func $type-i32-value))
  (export "type-i64-value" (func $type-i64-value))
  (export "type-f32-value" (func $type-f32-value))
  (export "type-f64-value" (func $type-f64-value))
  (export "nullary" (func $nullary))
  (export "unary" (func $unary))
  (export "as-func-first" (func $as-func-first))
  (export "as-func-mid" (func $as-func-mid))
  (export "as-func-last" (func $as-func-last))
  (export "as-func-value" (func $as-func-value))
  (export "as-block-first" (func $as-block-first))
  (export "as-block-mid" (func $as-block-mid))
  (export "as-block-last" (func $as-block-last))
  (export "as-block-value" (func $as-block-value))
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
  (export "as-call_indirect-func" (func $as-call_indirect-func))
  (export "as-call_indirect-first" (func $as-call_indirect-first))
  (export "as-call_indirect-mid" (func $as-call_indirect-mid))
  (export "as-call_indirect-last" (func $as-call_indirect-last))
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
  (elem $e0 (i32.const 0) func $f36))
