(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (result i64)))
  (type $t4 (func (result f32)))
  (type $t5 (func (result f64)))
  (type $t6 (func (result f64 f64)))
  (type $t7 (func (result i32 i64)))
  (type $t8 (func (param i32 i32) (result i32)))
  (func $f0 (type $t1))
  (func $type-i32 (type $t1)
    block $B0
      br $B0
      i32.ctz
      drop
    end)
  (func $type-i64 (type $t1)
    block $B0
      br $B0
      i64.ctz
      drop
    end)
  (func $type-f32 (type $t1)
    block $B0
      br $B0
      f32.neg
      drop
    end)
  (func $type-f64 (type $t1)
    block $B0
      br $B0
      f64.neg
      drop
    end)
  (func $type-i32-i32 (type $t1)
    block $B0
      br $B0
      i32.add
      drop
    end)
  (func $type-i64-i64 (type $t1)
    block $B0
      br $B0
      i64.add
      drop
    end)
  (func $type-f32-f32 (type $t1)
    block $B0
      br $B0
      f32.add
      drop
    end)
  (func $type-f64-f64 (type $t1)
    block $B0
      br $B0
      f64.add
      drop
    end)
  (func $type-i32-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      br $B0
      i32.ctz
    end)
  (func $type-i64-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 2
      br $B0
      i64.ctz
    end)
  (func $type-f32-value (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.8p+1 (;=3;)
      br $B0
      f32.neg
    end)
  (func $type-f64-value (type $t5) (result f64)
    block $B0 (result f64)
      f64.const 0x1p+2 (;=4;)
      br $B0
      f64.neg
    end)
  (func $type-f64-f64-value (type $t6) (result f64 f64)
    block $B0 (result f64 f64)
      f64.const 0x1p+2 (;=4;)
      f64.const 0x1.4p+2 (;=5;)
      br $B0
      f64.add
      f64.const 0x1.8p+2 (;=6;)
    end)
  (func $as-block-first (type $t1)
    block $B0
      br $B0
      call $f0
    end)
  (func $as-block-mid (type $t1)
    block $B0
      call $f0
      br $B0
      call $f0
    end)
  (func $as-block-last (type $t1)
    block $B0
      nop
      call $f0
      br $B0
    end)
  (func $as-block-value (type $t2) (result i32)
    block $B0 (result i32)
      nop
      call $f0
      i32.const 2
      br $B0
    end)
  (func $as-loop-first (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 3
        br $B0
        i32.const 2
      end
    end)
  (func $as-loop-mid (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        call $f0
        i32.const 4
        br $B0
        i32.const 2
      end
    end)
  (func $as-loop-last (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        nop
        call $f0
        i32.const 5
        br $B0
      end
    end)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 9
      br $B0
      br $B0
    end)
  (func $as-br_if-cond (type $t1)
    block $B0
      br $B0
      br_if $B0
    end)
  (func $as-br_if-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 8
      br $B0
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 9
      br $B0
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t1)
    block $B0
      br $B0
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 10
      br $B0
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-index (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 11
      br $B0
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-return-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 7
      br $B0
      return
    end)
  (func $as-return-values (type $t7) (result i32 i64)
    i32.const 2
    block $B0 (result i64)
      i32.const 1
      i64.const 7
      br $B0
      return
    end)
  (func $as-if-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      br $B0
      if $I1 (result i32)
        i32.const 0
      else
        i32.const 1
      end
    end)
  (func $as-if-then (type $t8) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        i32.const 3
        br $B0
      else
        local.get $p1
      end
    end)
  (func $as-if-else (type $t8) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        local.get $p1
      else
        i32.const 4
        br $B0
      end
    end)
  (func $as-select-first (type $t8) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      i32.const 5
      br $B0
      local.get $p0
      local.get $p1
      select
    end)
  (func $as-select-second (type $t8) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      i32.const 6
      br $B0
      local.get $p1
      select
    end)
  (func $as-select-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 7
      br $B0
      select
    end)
  (func $as-select-all (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 8
      br $B0
      select
    end)
  (func $f37 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 12
      br $B0
      i32.const 2
      i32.const 3
      call $f37
    end)
  (func $as-call-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 13
      br $B0
      i32.const 3
      call $f37
    end)
  (func $as-call-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 14
      br $B0
      call $f37
    end)
  (func $as-call-all (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 15
      br $B0
      call $f37
    end)
  (func $as-call_indirect-func (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 20
      br $B0
      i32.const 1
      i32.const 2
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 21
      br $B0
      i32.const 2
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 22
      br $B0
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 2
      i32.const 23
      br $B0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-all (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 24
      br $B0
      call_indirect $T0 (type $t0)
    end)
  (func $as-local.set-value (type $t2) (result i32)
    (local $l0 f32)
    block $B0 (result i32)
      i32.const 17
      br $B0
      local.set $l0
      i32.const -1
    end)
  (func $as-local.tee-value (type $t2) (result i32)
    (local $l0 i32)
    block $B0 (result i32)
      i32.const 1
      br $B0
      local.tee $l0
    end)
  (func $as-global.set-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      br $B0
      global.set $g0
    end)
  (func $as-load-address (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.b33334p+0 (;=1.7;)
      br $B0
      f32.load
    end)
  (func $as-loadN-address (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 30
      br $B0
      i64.load8_s
    end)
  (func $as-store-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 30
      br $B0
      f64.const 0x1.cp+2 (;=7;)
      f64.store
      i32.const -1
    end)
  (func $as-store-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 31
      br $B0
      i64.store
      i32.const -1
    end)
  (func $as-store-both (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 32
      br $B0
      i64.store
      i32.const -1
    end)
  (func $as-storeN-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 32
      br $B0
      i32.const 7
      i32.store8
      i32.const -1
    end)
  (func $as-storeN-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 33
      br $B0
      i64.store16
      i32.const -1
    end)
  (func $as-storeN-both (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 34
      br $B0
      i64.store16
      i32.const -1
    end)
  (func $as-unary-operand (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.b33334p+1 (;=3.4;)
      br $B0
      f32.neg
    end)
  (func $as-binary-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 3
      br $B0
      i32.const 10
      i32.add
    end)
  (func $as-binary-right (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 10
      i64.const 45
      br $B0
      i64.sub
    end)
  (func $as-binary-both (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 46
      br $B0
      i32.add
    end)
  (func $as-test-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 44
      br $B0
      i32.eqz
    end)
  (func $as-compare-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 43
      br $B0
      f64.const 0x1.4p+3 (;=10;)
      f64.le
    end)
  (func $as-compare-right (type $t2) (result i32)
    block $B0 (result i32)
      f32.const 0x1.4p+3 (;=10;)
      i32.const 42
      br $B0
      f32.ne
    end)
  (func $as-compare-both (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 44
      br $B0
      f64.le
    end)
  (func $as-convert-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 41
      br $B0
      i32.wrap_i64
    end)
  (func $as-memory.grow-size (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 40
      br $B0
      memory.grow
    end)
  (func $nested-block-value (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      call $f0
      i32.const 4
      i32.const 8
      br $B0
      i32.add
    end
    i32.add)
  (func $nested-br-value (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 4
        drop
        i32.const 8
        br $B0
        br $B1
      end
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_if-value (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 4
        drop
        i32.const 8
        br $B0
        i32.const 1
        br_if $B1
        drop
        i32.const 32
      end
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_if-value-cond (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      i32.const 4
      i32.const 8
      br $B0
      br_if $B0
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_table-value (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      block $B1 (result i32)
        i32.const 4
        drop
        i32.const 8
        br $B0
        i32.const 1
        br_table $B1
      end
      drop
      i32.const 16
    end
    i32.add)
  (func $nested-br_table-value-index (type $t2) (result i32)
    i32.const 1
    block $B0 (result i32)
      i32.const 2
      drop
      i32.const 4
      i32.const 8
      br $B0
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
  (export "type-i32-i32" (func $type-i32-i32))
  (export "type-i64-i64" (func $type-i64-i64))
  (export "type-f32-f32" (func $type-f32-f32))
  (export "type-f64-f64" (func $type-f64-f64))
  (export "type-i32-value" (func $type-i32-value))
  (export "type-i64-value" (func $type-i64-value))
  (export "type-f32-value" (func $type-f32-value))
  (export "type-f64-value" (func $type-f64-value))
  (export "type-f64-f64-value" (func $type-f64-f64-value))
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
  (export "as-return-values" (func $as-return-values))
  (export "as-if-cond" (func $as-if-cond))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-second" (func $as-select-second))
  (export "as-select-cond" (func $as-select-cond))
  (export "as-select-all" (func $as-select-all))
  (export "as-call-first" (func $as-call-first))
  (export "as-call-mid" (func $as-call-mid))
  (export "as-call-last" (func $as-call-last))
  (export "as-call-all" (func $as-call-all))
  (export "as-call_indirect-func" (func $as-call_indirect-func))
  (export "as-call_indirect-first" (func $as-call_indirect-first))
  (export "as-call_indirect-mid" (func $as-call_indirect-mid))
  (export "as-call_indirect-last" (func $as-call_indirect-last))
  (export "as-call_indirect-all" (func $as-call_indirect-all))
  (export "as-local.set-value" (func $as-local.set-value))
  (export "as-local.tee-value" (func $as-local.tee-value))
  (export "as-global.set-value" (func $as-global.set-value))
  (export "as-load-address" (func $as-load-address))
  (export "as-loadN-address" (func $as-loadN-address))
  (export "as-store-address" (func $as-store-address))
  (export "as-store-value" (func $as-store-value))
  (export "as-store-both" (func $as-store-both))
  (export "as-storeN-address" (func $as-storeN-address))
  (export "as-storeN-value" (func $as-storeN-value))
  (export "as-storeN-both" (func $as-storeN-both))
  (export "as-unary-operand" (func $as-unary-operand))
  (export "as-binary-left" (func $as-binary-left))
  (export "as-binary-right" (func $as-binary-right))
  (export "as-binary-both" (func $as-binary-both))
  (export "as-test-operand" (func $as-test-operand))
  (export "as-compare-left" (func $as-compare-left))
  (export "as-compare-right" (func $as-compare-right))
  (export "as-compare-both" (func $as-compare-both))
  (export "as-convert-operand" (func $as-convert-operand))
  (export "as-memory.grow-size" (func $as-memory.grow-size))
  (export "nested-block-value" (func $nested-block-value))
  (export "nested-br-value" (func $nested-br-value))
  (export "nested-br_if-value" (func $nested-br_if-value))
  (export "nested-br_if-value-cond" (func $nested-br_if-value-cond))
  (export "nested-br_table-value" (func $nested-br_table-value))
  (export "nested-br_table-value-index" (func $nested-br_table-value-index))
  (elem $e0 (i32.const 0) func $f37))
