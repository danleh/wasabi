(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func (result i32)))
  (type $t2 (func))
  (type $t3 (func (param i32 i32) (result i32)))
  (func $as-br-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.load
      br $B0
    end)
  (func $as-br_if-cond (type $t2)
    block $B0
      i32.const 0
      i32.load
      br_if $B0
    end)
  (func $as-br_if-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.load
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 0
      i32.load
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t2)
    block $B0
      i32.const 0
      i32.load
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.load
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-index (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 0
      i32.load
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-return-value (type $t1) (result i32)
    i32.const 0
    i32.load
    return)
  (func $as-if-cond (type $t1) (result i32)
    i32.const 0
    i32.load
    if $I0 (result i32)
      i32.const 0
    else
      i32.const 1
    end)
  (func $as-if-then (type $t1) (result i32)
    i32.const 1
    if $I0 (result i32)
      i32.const 0
      i32.load
    else
      i32.const 0
    end)
  (func $as-if-else (type $t1) (result i32)
    i32.const 0
    if $I0 (result i32)
      i32.const 0
    else
      i32.const 0
      i32.load
    end)
  (func $as-select-first (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    i32.const 0
    i32.load
    local.get $p0
    local.get $p1
    select)
  (func $as-select-second (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    i32.const 0
    i32.load
    local.get $p1
    select)
  (func $as-select-cond (type $t1) (result i32)
    i32.const 0
    i32.const 1
    i32.const 0
    i32.load
    select)
  (func $f14 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t1) (result i32)
    i32.const 0
    i32.load
    i32.const 2
    i32.const 3
    call $f14)
  (func $as-call-mid (type $t1) (result i32)
    i32.const 1
    i32.const 0
    i32.load
    i32.const 3
    call $f14)
  (func $as-call-last (type $t1) (result i32)
    i32.const 1
    i32.const 2
    i32.const 0
    i32.load
    call $f14)
  (func $as-call_indirect-first (type $t1) (result i32)
    i32.const 0
    i32.load
    i32.const 2
    i32.const 3
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-mid (type $t1) (result i32)
    i32.const 1
    i32.const 0
    i32.load
    i32.const 3
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-last (type $t1) (result i32)
    i32.const 1
    i32.const 2
    i32.const 0
    i32.load
    i32.const 0
    call_indirect $T0 (type $t0))
  (func $as-call_indirect-index (type $t1) (result i32)
    i32.const 1
    i32.const 2
    i32.const 3
    i32.const 0
    i32.load
    call_indirect $T0 (type $t0))
  (func $as-local.set-value (type $t2)
    (local $l0 i32)
    i32.const 0
    i32.load
    local.set $l0)
  (func $as-local.tee-value (type $t1) (result i32)
    (local $l0 i32)
    i32.const 0
    i32.load
    local.tee $l0)
  (func $as-global.set-value (type $t2)
    (local $l0 i32)
    i32.const 0
    i32.load
    global.set $g0)
  (func $as-load-address (type $t1) (result i32)
    i32.const 0
    i32.load
    i32.load)
  (func $as-loadN-address (type $t1) (result i32)
    i32.const 0
    i32.load
    i32.load8_s)
  (func $as-store-address (type $t2)
    i32.const 0
    i32.load
    i32.const 7
    i32.store)
  (func $as-store-value (type $t2)
    i32.const 2
    i32.const 0
    i32.load
    i32.store)
  (func $as-storeN-address (type $t2)
    i32.const 0
    i32.load8_s
    i32.const 7
    i32.store8)
  (func $as-storeN-value (type $t2)
    i32.const 2
    i32.const 0
    i32.load
    i32.store16)
  (func $as-unary-operand (type $t1) (result i32)
    i32.const 100
    i32.load
    i32.clz)
  (func $as-binary-left (type $t1) (result i32)
    i32.const 100
    i32.load
    i32.const 10
    i32.add)
  (func $as-binary-right (type $t1) (result i32)
    i32.const 10
    i32.const 100
    i32.load
    i32.sub)
  (func $as-test-operand (type $t1) (result i32)
    i32.const 100
    i32.load
    i32.eqz)
  (func $as-compare-left (type $t1) (result i32)
    i32.const 100
    i32.load
    i32.const 10
    i32.le_s)
  (func $as-compare-right (type $t1) (result i32)
    i32.const 10
    i32.const 100
    i32.load
    i32.ne)
  (func $as-memory.grow-size (type $t1) (result i32)
    i32.const 100
    i32.load
    memory.grow)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 0))
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
  (export "as-memory.grow-size" (func $as-memory.grow-size))
  (elem $e0 (i32.const 0) func $f14))
