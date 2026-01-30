(module
  (type $t0 (func (param i32 i32 i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (result i64)))
  (type $t4 (func (result f32)))
  (type $t5 (func (result f64)))
  (type $t6 (func (param i32 i32) (result i32)))
  (func $f0 (type $t1))
  (func $f1 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32))
  (func $type-i32 (type $t2) (result i32)
    unreachable)
  (func $type-i64 (type $t3) (result i64)
    unreachable)
  (func $type-f32 (type $t4) (result f32)
    unreachable)
  (func $type-f64 (type $t5) (result f64)
    unreachable)
  (func $as-func-first (type $t2) (result i32)
    unreachable
    i32.const -1)
  (func $as-func-mid (type $t2) (result i32)
    call $f0
    unreachable
    i32.const -1)
  (func $as-func-last (type $t1)
    call $f0
    unreachable)
  (func $as-func-value (type $t2) (result i32)
    call $f0
    unreachable)
  (func $as-block-first (type $t2) (result i32)
    block $B0 (result i32)
      unreachable
      i32.const 2
    end)
  (func $as-block-mid (type $t2) (result i32)
    block $B0 (result i32)
      call $f0
      unreachable
      i32.const 2
    end)
  (func $as-block-last (type $t1)
    block $B0
      nop
      call $f0
      unreachable
    end)
  (func $as-block-value (type $t2) (result i32)
    block $B0 (result i32)
      nop
      call $f0
      unreachable
    end)
  (func $as-block-broke (type $t2) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 1
      br $B0
      unreachable
    end)
  (func $as-loop-first (type $t2) (result i32)
    loop $L0 (result i32)
      unreachable
      i32.const 2
    end)
  (func $as-loop-mid (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      unreachable
      i32.const 2
    end)
  (func $as-loop-last (type $t1)
    loop $L0
      nop
      call $f0
      unreachable
    end)
  (func $as-loop-broke (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        call $f0
        i32.const 1
        br $B0
        unreachable
      end
    end)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      unreachable
      br $B0
    end)
  (func $as-br_if-cond (type $t1)
    block $B0
      unreachable
      br_if $B0
    end)
  (func $as-br_if-value (type $t2) (result i32)
    block $B0 (result i32)
      unreachable
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      unreachable
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t1)
    block $B0
      unreachable
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t2) (result i32)
    block $B0 (result i32)
      unreachable
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-2 (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        unreachable
        i32.const 1
        br_table $B1 $B0
      end
    end)
  (func $as-br_table-value-index (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      unreachable
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-and-index (type $t2) (result i32)
    block $B0 (result i32)
      unreachable
      br_table $B0 $B0
      i32.const 8
    end)
  (func $as-return-value (type $t3) (result i64)
    unreachable
    return)
  (func $as-if-cond (type $t2) (result i32)
    unreachable
    if $I0 (result i32)
      i32.const 0
    else
      i32.const 1
    end)
  (func $as-if-then (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      unreachable
    else
      local.get $p1
    end)
  (func $as-if-else (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      local.get $p1
    else
      unreachable
    end)
  (func $as-if-then-no-else (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0
      unreachable
    end
    local.get $p1)
  (func $as-select-first (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    unreachable
    local.get $p0
    local.get $p1
    select)
  (func $as-select-second (type $t6) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    unreachable
    local.get $p1
    select)
  (func $as-select-cond (type $t2) (result i32)
    i32.const 0
    i32.const 1
    unreachable
    select)
  (func $as-call-first (type $t1)
    unreachable
    i32.const 2
    i32.const 3
    call $f1)
  (func $as-call-mid (type $t1)
    i32.const 1
    unreachable
    i32.const 3
    call $f1)
  (func $as-call-last (type $t1)
    i32.const 1
    i32.const 2
    unreachable
    call $f1)
  (func $as-call_indirect-func (type $t1)
    unreachable
    i32.const 1
    i32.const 2
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-first (type $t1)
    i32.const 0
    unreachable
    i32.const 2
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-mid (type $t1)
    i32.const 0
    i32.const 1
    unreachable
    i32.const 3
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-last (type $t1)
    i32.const 0
    i32.const 1
    i32.const 2
    unreachable
    call_indirect $T0 (type $t0))
  (func $as-local.set-value (type $t1)
    (local $l0 f32)
    unreachable
    local.set $l0)
  (func $as-local.tee-value (type $t4) (result f32)
    (local $l0 f32)
    unreachable
    local.tee $l0)
  (func $as-global.set-value (type $t4) (result f32)
    unreachable
    global.set $g0)
  (func $as-load-address (type $t4) (result f32)
    unreachable
    f32.load)
  (func $as-loadN-address (type $t3) (result i64)
    unreachable
    i64.load8_s)
  (func $as-store-address (type $t1)
    unreachable
    f64.const 0x1.cp+2 (;=7;)
    f64.store)
  (func $as-store-value (type $t1)
    i32.const 2
    unreachable
    i64.store)
  (func $as-storeN-address (type $t1)
    unreachable
    i32.const 7
    i32.store8)
  (func $as-storeN-value (type $t1)
    i32.const 2
    unreachable
    i64.store16)
  (func $as-unary-operand (type $t4) (result f32)
    unreachable
    f32.neg)
  (func $as-binary-left (type $t2) (result i32)
    unreachable
    i32.const 10
    i32.add)
  (func $as-binary-right (type $t3) (result i64)
    i64.const 10
    unreachable
    i64.sub)
  (func $as-test-operand (type $t2) (result i32)
    unreachable
    i32.eqz)
  (func $as-compare-left (type $t2) (result i32)
    unreachable
    f64.const 0x1.4p+3 (;=10;)
    f64.le)
  (func $as-compare-right (type $t2) (result i32)
    f32.const 0x1.4p+3 (;=10;)
    unreachable
    f32.ne)
  (func $as-convert-operand (type $t2) (result i32)
    unreachable
    i32.wrap_i64)
  (func $as-memory.grow-size (type $t2) (result i32)
    unreachable
    memory.grow)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut f32) (f32.const 0x0p+0 (;=0;)))
  (export "type-i32" (func $type-i32))
  (export "type-i64" (func $type-i64))
  (export "type-f32" (func $type-f32))
  (export "type-f64" (func $type-f64))
  (export "as-func-first" (func $as-func-first))
  (export "as-func-mid" (func $as-func-mid))
  (export "as-func-last" (func $as-func-last))
  (export "as-func-value" (func $as-func-value))
  (export "as-block-first" (func $as-block-first))
  (export "as-block-mid" (func $as-block-mid))
  (export "as-block-last" (func $as-block-last))
  (export "as-block-value" (func $as-block-value))
  (export "as-block-broke" (func $as-block-broke))
  (export "as-loop-first" (func $as-loop-first))
  (export "as-loop-mid" (func $as-loop-mid))
  (export "as-loop-last" (func $as-loop-last))
  (export "as-loop-broke" (func $as-loop-broke))
  (export "as-br-value" (func $as-br-value))
  (export "as-br_if-cond" (func $as-br_if-cond))
  (export "as-br_if-value" (func $as-br_if-value))
  (export "as-br_if-value-cond" (func $as-br_if-value-cond))
  (export "as-br_table-index" (func $as-br_table-index))
  (export "as-br_table-value" (func $as-br_table-value))
  (export "as-br_table-value-2" (func $as-br_table-value-2))
  (export "as-br_table-value-index" (func $as-br_table-value-index))
  (export "as-br_table-value-and-index" (func $as-br_table-value-and-index))
  (export "as-return-value" (func $as-return-value))
  (export "as-if-cond" (func $as-if-cond))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else))
  (export "as-if-then-no-else" (func $as-if-then-no-else))
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
  (elem $e0 (i32.const 0) func $f1))
