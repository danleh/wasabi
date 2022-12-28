(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (param i32 i32 i32) (result i32)))
  (type $t3 (func (result i32)))
  (type $t4 (func (param i32)))
  (type $t5 (func (param i32) (result i32)))
  (type $t6 (func (param i32 i32)))
  (func $f0 (type $t1))
  (func $f1 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    local.get $p1
    local.get $p2
    i32.sub
    i32.add)
  (func $as-func-first (type $t3) (result i32)
    nop
    i32.const 1)
  (func $as-func-mid (type $t3) (result i32)
    call $f0
    nop
    i32.const 2)
  (func $as-func-last (type $t3) (result i32)
    call $f0
    i32.const 3
    nop)
  (func $as-func-everywhere (type $t3) (result i32)
    nop
    nop
    call $f0
    nop
    i32.const 4
    nop
    nop)
  (func $as-drop-first (type $t4) (param $p0 i32)
    nop
    local.get $p0
    drop)
  (func $as-drop-last (type $t4) (param $p0 i32)
    local.get $p0
    nop
    drop)
  (func $as-drop-everywhere (type $t4) (param $p0 i32)
    nop
    nop
    local.get $p0
    nop
    nop
    drop)
  (func $as-select-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    local.get $p0
    local.get $p0
    select)
  (func $as-select-mid1 (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    local.get $p0
    local.get $p0
    select)
  (func $as-select-mid2 (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    local.get $p0
    nop
    local.get $p0
    select)
  (func $as-select-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    local.get $p0
    local.get $p0
    nop
    select)
  (func $as-select-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    nop
    nop
    local.get $p0
    nop
    nop
    local.get $p0
    nop
    nop
    select)
  (func $as-block-first (type $t3) (result i32)
    block $B0 (result i32)
      nop
      i32.const 2
    end)
  (func $as-block-mid (type $t3) (result i32)
    block $B0 (result i32)
      call $f0
      nop
      i32.const 2
    end)
  (func $as-block-last (type $t3) (result i32)
    block $B0 (result i32)
      nop
      call $f0
      i32.const 3
      nop
    end)
  (func $as-block-everywhere (type $t3) (result i32)
    block $B0 (result i32)
      nop
      nop
      call $f0
      nop
      i32.const 4
      nop
      nop
    end)
  (func $as-loop-first (type $t3) (result i32)
    loop $L0 (result i32)
      nop
      i32.const 2
    end)
  (func $as-loop-mid (type $t3) (result i32)
    loop $L0 (result i32)
      call $f0
      nop
      i32.const 2
    end)
  (func $as-loop-last (type $t3) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 3
      nop
    end)
  (func $as-loop-everywhere (type $t3) (result i32)
    loop $L0 (result i32)
      nop
      nop
      call $f0
      nop
      i32.const 4
      nop
      nop
    end)
  (func $as-if-condition (type $t4) (param $p0 i32)
    local.get $p0
    nop
    if $I0
      call $f0
    end)
  (func $as-if-then (type $t4) (param $p0 i32)
    local.get $p0
    if $I0
      nop
    else
      call $f0
    end)
  (func $as-if-else (type $t4) (param $p0 i32)
    local.get $p0
    if $I0
      call $f0
    else
      nop
    end)
  (func $as-br-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      local.get $p0
      br $B0
    end)
  (func $as-br-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      nop
      br $B0
    end)
  (func $as-br-everywhere (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      nop
      local.get $p0
      nop
      nop
      br $B0
    end)
  (func $as-br_if-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      local.get $p0
      local.get $p0
      br_if $B0
    end)
  (func $as-br_if-mid (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      nop
      local.get $p0
      br_if $B0
    end)
  (func $as-br_if-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      local.get $p0
      nop
      br_if $B0
    end)
  (func $as-br_if-everywhere (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      nop
      local.get $p0
      nop
      nop
      local.get $p0
      nop
      nop
      br_if $B0
    end)
  (func $as-br_table-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      local.get $p0
      local.get $p0
      br_table $B0 $B0
    end)
  (func $as-br_table-mid (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      nop
      local.get $p0
      br_table $B0 $B0
    end)
  (func $as-br_table-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      local.get $p0
      nop
      br_table $B0 $B0
    end)
  (func $as-br_table-everywhere (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      nop
      nop
      local.get $p0
      nop
      nop
      local.get $p0
      nop
      nop
      br_table $B0 $B0
    end)
  (func $as-return-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    return)
  (func $as-return-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    return)
  (func $as-return-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    return)
  (func $as-call-first (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    nop
    local.get $p0
    local.get $p1
    local.get $p2
    call $f1)
  (func $as-call-mid1 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    nop
    local.get $p1
    local.get $p2
    call $f1)
  (func $as-call-mid2 (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    local.get $p1
    nop
    local.get $p2
    call $f1)
  (func $as-call-last (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    local.get $p0
    local.get $p1
    local.get $p2
    nop
    call $f1)
  (func $as-call-everywhere (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    local.get $p1
    nop
    nop
    local.get $p2
    nop
    nop
    call $f1)
  (func $as-unary-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    i32.ctz)
  (func $as-unary-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    i32.ctz)
  (func $as-unary-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    i32.ctz)
  (func $as-binary-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    local.get $p0
    i32.add)
  (func $as-binary-mid (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    local.get $p0
    i32.add)
  (func $as-binary-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    local.get $p0
    nop
    i32.add)
  (func $as-binary-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    nop
    nop
    local.get $p0
    nop
    nop
    i32.add)
  (func $as-test-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    i32.eqz)
  (func $as-test-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    i32.eqz)
  (func $as-test-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    i32.eqz)
  (func $as-compare-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    local.get $p0
    i32.ne)
  (func $as-compare-mid (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    local.get $p0
    i32.ne)
  (func $as-compare-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    local.get $p0
    nop
    i32.lt_u)
  (func $as-compare-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    nop
    nop
    local.get $p0
    nop
    nop
    i32.le_s)
  (func $as-memory.grow-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    memory.grow)
  (func $as-memory.grow-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    memory.grow)
  (func $as-memory.grow-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    memory.grow)
  (func $f61 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-first (type $t3) (result i32)
    block $B0 (result i32)
      nop
      i32.const 1
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid1 (type $t3) (result i32)
    block $B0 (result i32)
      i32.const 1
      nop
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid2 (type $t3) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      nop
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t3) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 0
      nop
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-everywhere (type $t3) (result i32)
    block $B0 (result i32)
      nop
      nop
      i32.const 1
      nop
      nop
      i32.const 2
      nop
      nop
      i32.const 0
      nop
      nop
      call_indirect $T0 (type $t0)
    end)
  (func $as-local.set-first (type $t5) (param $p0 i32) (result i32)
    nop
    i32.const 2
    local.set $p0
    local.get $p0)
  (func $as-local.set-last (type $t5) (param $p0 i32) (result i32)
    i32.const 2
    nop
    local.set $p0
    local.get $p0)
  (func $as-local.set-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    i32.const 2
    nop
    nop
    local.set $p0
    local.get $p0)
  (func $as-local.tee-first (type $t5) (param $p0 i32) (result i32)
    nop
    i32.const 2
    local.tee $p0)
  (func $as-local.tee-last (type $t5) (param $p0 i32) (result i32)
    i32.const 2
    nop
    local.tee $p0)
  (func $as-local.tee-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    i32.const 2
    nop
    nop
    local.tee $p0)
  (func $as-global.set-first (type $t3) (result i32)
    nop
    i32.const 2
    global.set $g0
    global.get $g0)
  (func $as-global.set-last (type $t3) (result i32)
    i32.const 2
    nop
    global.set $g0
    global.get $g0)
  (func $as-global.set-everywhere (type $t3) (result i32)
    nop
    nop
    i32.const 2
    nop
    nop
    global.set $g0
    global.get $g0)
  (func $as-load-first (type $t5) (param $p0 i32) (result i32)
    nop
    local.get $p0
    i32.load)
  (func $as-load-last (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    nop
    i32.load)
  (func $as-load-everywhere (type $t5) (param $p0 i32) (result i32)
    nop
    nop
    local.get $p0
    nop
    nop
    i32.load)
  (func $as-store-first (type $t6) (param $p0 i32) (param $p1 i32)
    nop
    local.get $p0
    local.get $p1
    i32.store)
  (func $as-store-mid (type $t6) (param $p0 i32) (param $p1 i32)
    local.get $p0
    nop
    local.get $p1
    i32.store)
  (func $as-store-last (type $t6) (param $p0 i32) (param $p1 i32)
    local.get $p0
    local.get $p1
    nop
    i32.store)
  (func $as-store-everywhere (type $t6) (param $p0 i32) (param $p1 i32)
    nop
    nop
    local.get $p0
    nop
    nop
    local.get $p1
    nop
    nop
    i32.store)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 0))
  (export "as-func-first" (func $as-func-first))
  (export "as-func-mid" (func $as-func-mid))
  (export "as-func-last" (func $as-func-last))
  (export "as-func-everywhere" (func $as-func-everywhere))
  (export "as-drop-first" (func $as-drop-first))
  (export "as-drop-last" (func $as-drop-last))
  (export "as-drop-everywhere" (func $as-drop-everywhere))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-mid1" (func $as-select-mid1))
  (export "as-select-mid2" (func $as-select-mid2))
  (export "as-select-last" (func $as-select-last))
  (export "as-select-everywhere" (func $as-select-everywhere))
  (export "as-block-first" (func $as-block-first))
  (export "as-block-mid" (func $as-block-mid))
  (export "as-block-last" (func $as-block-last))
  (export "as-block-everywhere" (func $as-block-everywhere))
  (export "as-loop-first" (func $as-loop-first))
  (export "as-loop-mid" (func $as-loop-mid))
  (export "as-loop-last" (func $as-loop-last))
  (export "as-loop-everywhere" (func $as-loop-everywhere))
  (export "as-if-condition" (func $as-if-condition))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else))
  (export "as-br-first" (func $as-br-first))
  (export "as-br-last" (func $as-br-last))
  (export "as-br-everywhere" (func $as-br-everywhere))
  (export "as-br_if-first" (func $as-br_if-first))
  (export "as-br_if-mid" (func $as-br_if-mid))
  (export "as-br_if-last" (func $as-br_if-last))
  (export "as-br_if-everywhere" (func $as-br_if-everywhere))
  (export "as-br_table-first" (func $as-br_table-first))
  (export "as-br_table-mid" (func $as-br_table-mid))
  (export "as-br_table-last" (func $as-br_table-last))
  (export "as-br_table-everywhere" (func $as-br_table-everywhere))
  (export "as-return-first" (func $as-return-first))
  (export "as-return-last" (func $as-return-last))
  (export "as-return-everywhere" (func $as-return-everywhere))
  (export "as-call-first" (func $as-call-first))
  (export "as-call-mid1" (func $as-call-mid1))
  (export "as-call-mid2" (func $as-call-mid2))
  (export "as-call-last" (func $as-call-last))
  (export "as-call-everywhere" (func $as-call-everywhere))
  (export "as-unary-first" (func $as-unary-first))
  (export "as-unary-last" (func $as-unary-last))
  (export "as-unary-everywhere" (func $as-unary-everywhere))
  (export "as-binary-first" (func $as-binary-first))
  (export "as-binary-mid" (func $as-binary-mid))
  (export "as-binary-last" (func $as-binary-last))
  (export "as-binary-everywhere" (func $as-binary-everywhere))
  (export "as-test-first" (func $as-test-first))
  (export "as-test-last" (func $as-test-last))
  (export "as-test-everywhere" (func $as-test-everywhere))
  (export "as-compare-first" (func $as-compare-first))
  (export "as-compare-mid" (func $as-compare-mid))
  (export "as-compare-last" (func $as-compare-last))
  (export "as-compare-everywhere" (func $as-compare-everywhere))
  (export "as-memory.grow-first" (func $as-memory.grow-first))
  (export "as-memory.grow-last" (func $as-memory.grow-last))
  (export "as-memory.grow-everywhere" (func $as-memory.grow-everywhere))
  (export "as-call_indirect-first" (func $as-call_indirect-first))
  (export "as-call_indirect-mid1" (func $as-call_indirect-mid1))
  (export "as-call_indirect-mid2" (func $as-call_indirect-mid2))
  (export "as-call_indirect-last" (func $as-call_indirect-last))
  (export "as-call_indirect-everywhere" (func $as-call_indirect-everywhere))
  (export "as-local.set-first" (func $as-local.set-first))
  (export "as-local.set-last" (func $as-local.set-last))
  (export "as-local.set-everywhere" (func $as-local.set-everywhere))
  (export "as-local.tee-first" (func $as-local.tee-first))
  (export "as-local.tee-last" (func $as-local.tee-last))
  (export "as-local.tee-everywhere" (func $as-local.tee-everywhere))
  (export "as-global.set-first" (func $as-global.set-first))
  (export "as-global.set-last" (func $as-global.set-last))
  (export "as-global.set-everywhere" (func $as-global.set-everywhere))
  (export "as-load-first" (func $as-load-first))
  (export "as-load-last" (func $as-load-last))
  (export "as-load-everywhere" (func $as-load-everywhere))
  (export "as-store-first" (func $as-store-first))
  (export "as-store-mid" (func $as-store-mid))
  (export "as-store-last" (func $as-store-last))
  (export "as-store-everywhere" (func $as-store-everywhere))
  (elem $e0 (i32.const 0) func $f61))
