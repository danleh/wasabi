(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (result i64)))
  (type $t4 (func (result f32)))
  (type $t5 (func (result f64)))
  (type $t6 (func (param i32) (result i32)))
  (type $t7 (func (param i32)))
  (type $t8 (func (param i32 i32)))
  (func $f0 (type $t1))
  (func $type-i32 (type $t1)
    block $B0
      i32.const 0
      i32.const 1
      br_if $B0
      i32.ctz
      drop
    end)
  (func $type-i64 (type $t1)
    block $B0
      i64.const 0
      i32.const 1
      br_if $B0
      i64.ctz
      drop
    end)
  (func $type-f32 (type $t1)
    block $B0
      f32.const 0x0p+0 (;=0;)
      i32.const 1
      br_if $B0
      f32.neg
      drop
    end)
  (func $type-f64 (type $t1)
    block $B0
      f64.const 0x0p+0 (;=0;)
      i32.const 1
      br_if $B0
      f64.neg
      drop
    end)
  (func $type-i32-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_if $B0
      i32.ctz
    end)
  (func $type-i64-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 2
      i32.const 1
      br_if $B0
      i64.ctz
    end)
  (func $type-f32-value (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.8p+1 (;=3;)
      i32.const 1
      br_if $B0
      f32.neg
    end)
  (func $type-f64-value (type $t5) (result f64)
    block $B0 (result f64)
      f64.const 0x1p+2 (;=4;)
      i32.const 1
      br_if $B0
      f64.neg
    end)
  (func $as-block-first (type $t6) (param $p0 i32) (result i32)
    block $B0
      local.get $p0
      br_if $B0
      i32.const 2
      return
    end
    i32.const 3)
  (func $as-block-mid (type $t6) (param $p0 i32) (result i32)
    block $B0
      call $f0
      local.get $p0
      br_if $B0
      i32.const 2
      return
    end
    i32.const 3)
  (func $as-block-last (type $t7) (param $p0 i32)
    block $B0
      call $f0
      call $f0
      local.get $p0
      br_if $B0
    end)
  (func $as-block-first-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 10
      local.get $p0
      br_if $B0
      drop
      i32.const 11
      return
    end)
  (func $as-block-mid-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 20
      local.get $p0
      br_if $B0
      drop
      i32.const 21
      return
    end)
  (func $as-block-last-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      call $f0
      call $f0
      i32.const 11
      local.get $p0
      br_if $B0
    end)
  (func $as-loop-first (type $t6) (param $p0 i32) (result i32)
    block $B0
      loop $L1
        local.get $p0
        br_if $B0
        i32.const 2
        return
      end
    end
    i32.const 3)
  (func $as-loop-mid (type $t6) (param $p0 i32) (result i32)
    block $B0
      loop $L1
        call $f0
        local.get $p0
        br_if $B0
        i32.const 2
        return
      end
    end
    i32.const 4)
  (func $as-loop-last (type $t7) (param $p0 i32)
    loop $L0
      call $f0
      local.get $p0
      br_if 1 (;@0;)
    end)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      br_if $B0
      br $B0
    end)
  (func $as-br_if-cond (type $t1)
    block $B0
      i32.const 1
      i32.const 1
      br_if $B0
      br_if $B0
    end)
  (func $as-br_if-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      br_if $B0
      i32.const 3
      br_if $B0
      drop
      i32.const 4
    end)
  (func $as-br_if-value-cond (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 1
      local.get $p0
      br_if $B0
      br_if $B0
      drop
      i32.const 4
    end)
  (func $as-br_table-index (type $t1)
    block $B0
      i32.const 1
      i32.const 2
      br_if $B0
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      br_if $B0
      i32.const 3
      br_table $B0 $B0 $B0
      i32.const 4
    end)
  (func $as-br_table-value-index (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 1
      i32.const 3
      br_if $B0
      br_table $B0 $B0
      i32.const 4
    end)
  (func $as-return-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 1
      i32.const 2
      br_if $B0
      return
    end)
  (func $as-if-cond (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      local.get $p0
      br_if $B0
      if $I1 (result i32)
        i32.const 2
      else
        i32.const 3
      end
    end)
  (func $as-if-then (type $t8) (param $p0 i32) (param $p1 i32)
    block $B0
      local.get $p0
      if $I1
        local.get $p1
        br_if $B0
      else
        call $f0
      end
    end)
  (func $as-if-else (type $t8) (param $p0 i32) (param $p1 i32)
    block $B0
      local.get $p0
      if $I1
        call $f0
      else
        local.get $p1
        br_if $B0
      end
    end)
  (func $as-select-first (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 3
      i32.const 10
      br_if $B0
      i32.const 2
      local.get $p0
      select
    end)
  (func $as-select-second (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 3
      i32.const 10
      br_if $B0
      local.get $p0
      select
    end)
  (func $as-select-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 3
      i32.const 10
      br_if $B0
      select
    end)
  (func $f32 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 12
      i32.const 1
      br_if $B0
      i32.const 2
      i32.const 3
      call $f32
    end)
  (func $as-call-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 13
      i32.const 1
      br_if $B0
      i32.const 3
      call $f32
    end)
  (func $as-call-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 14
      i32.const 1
      br_if $B0
      call $f32
    end)
  (func $f36 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-func (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 4
      i32.const 10
      br_if $B0
      i32.const 1
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 4
      i32.const 10
      br_if $B0
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 4
      i32.const 10
      br_if $B0
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 3
      i32.const 4
      i32.const 10
      br_if $B0
      call_indirect $T0 (type $t0)
    end)
  (func $as-local.set-value (type $t6) (param $p0 i32) (result i32)
    (local $l1 i32)
    block $B0 (result i32)
      i32.const 17
      local.get $p0
      br_if $B0
      local.set $p0
      i32.const -1
    end)
  (func $as-local.tee-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      local.get $p0
      br_if $B0
      local.tee $p0
      i32.const -1
      return
    end)
  (func $as-global.set-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      local.get $p0
      br_if $B0
      global.set $g0
      i32.const -1
      return
    end)
  (func $as-load-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_if $B0
      i32.load
    end)
  (func $as-loadN-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 30
      i32.const 1
      br_if $B0
      i32.load8_s
    end)
  (func $as-store-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 30
      i32.const 1
      br_if $B0
      i32.const 7
      i32.store
      i32.const -1
    end)
  (func $as-store-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 31
      i32.const 1
      br_if $B0
      i32.store
      i32.const -1
    end)
  (func $as-storeN-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 32
      i32.const 1
      br_if $B0
      i32.const 7
      i32.store8
      i32.const -1
    end)
  (func $as-storeN-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 33
      i32.const 1
      br_if $B0
      i32.store16
      i32.const -1
    end)
  (func $as-unary-operand (type $t5) (result f64)
    block $B0 (result f64)
      f64.const 0x1p+0 (;=1;)
      i32.const 1
      br_if $B0
      f64.neg
    end)
  (func $as-binary-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_if $B0
      i32.const 10
      i32.add
    end)
  (func $as-binary-right (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 10
      i32.const 1
      i32.const 1
      br_if $B0
      i32.sub
    end)
  (func $as-test-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      br_if $B0
      i32.eqz
    end)
  (func $as-compare-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_if $B0
      i32.const 10
      i32.le_u
    end)
  (func $as-compare-right (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 10
      i32.const 1
      i32.const 42
      br_if $B0
      i32.ne
    end)
  (func $as-memory.grow-size (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_if $B0
      memory.grow
    end)
  (func $nested-block-value (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      i32.const 4
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 16
      end
      i32.add
    end
    i32.add)
  (func $nested-br-value (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 4
      end
      br $B0
      i32.const 16
    end
    i32.add)
  (func $nested-br_if-value (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 4
      end
      i32.const 1
      br_if $B0
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_if-value-cond (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      i32.const 4
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 1
      end
      br_if $B0
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_table-value (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 4
      end
      i32.const 1
      br_table $B0
      i32.const 16
    end
    i32.add)
  (func $nested-br_table-value-index (type $t6) (param $p0 i32) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      i32.const 4
      block $B1 (result i32)
        i32.const 8
        local.get $p0
        br_if $B0
        drop
        i32.const 1
      end
      br_table $B0
      i32.const 16
    end
    i32.add)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 10))
  (export "type-i32" (func $type-i32))
  (export "type-i64" (func $type-i64))
  (export "type-f32" (func $type-f32))
  (export "type-f64" (func $type-f64))
  (export "type-i32-value" (func $type-i32-value))
  (export "type-i64-value" (func $type-i64-value))
  (export "type-f32-value" (func $type-f32-value))
  (export "type-f64-value" (func $type-f64-value))
  (export "as-block-first" (func $as-block-first))
  (export "as-block-mid" (func $as-block-mid))
  (export "as-block-last" (func $as-block-last))
  (export "as-block-first-value" (func $as-block-first-value))
  (export "as-block-mid-value" (func $as-block-mid-value))
  (export "as-block-last-value" (func $as-block-last-value))
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
  (export "as-memory.grow-size" (func $as-memory.grow-size))
  (export "nested-block-value" (func $nested-block-value))
  (export "nested-br-value" (func $nested-br-value))
  (export "nested-br_if-value" (func $nested-br_if-value))
  (export "nested-br_if-value-cond" (func $nested-br_if-value-cond))
  (export "nested-br_table-value" (func $nested-br_table-value))
  (export "nested-br_table-value-index" (func $nested-br_table-value-index))
  (elem $e0 (i32.const 0) func $f36))
