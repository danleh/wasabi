(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32)))
  (type $t4 (func (param i32 f64 i32) (result i32 f64 i32)))
  (type $t5 (func (param i32) (result i32)))
  (type $t6 (func (param i32) (result i32 i32)))
  (type $t7 (func (result i32 i64 i32)))
  (type $t8 (func (result i32 i32)))
  (type $t9 (func (result f32 f32)))
  (type $t10 (func (param i32) (result i32 i32 i64)))
  (type $t11 (func (result i32 i32 i64)))
  (type $t12 (func (param i32 i32) (result i32 i32)))
  (type $t13 (func (param i64 i64 i32) (result i64 i32)))
  (type $t14 (func (param i64 i64) (result i64)))
  (type $t15 (func (param i64) (result i64)))
  (func $f0 (type $t1))
  (func $empty (type $t3) (param $p0 i32)
    local.get $p0
    if $I0
    end
    local.get $p0
    if $I1
    end
    local.get $p0
    if $I2
    end
    local.get $p0
    if $I3
    end)
  (func $singular (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0
      nop
    end
    local.get $p0
    if $I1
      nop
    else
      nop
    end
    local.get $p0
    if $I2 (result i32)
      i32.const 7
    else
      i32.const 8
    end)
  (func $multi (type $t6) (param $p0 i32) (result i32 i32)
    local.get $p0
    if $I0
      call $f0
      call $f0
      call $f0
    end
    local.get $p0
    if $I1
    else
      call $f0
      call $f0
      call $f0
    end
    local.get $p0
    if $I2 (result i32)
      call $f0
      call $f0
      i32.const 8
      call $f0
    else
      call $f0
      call $f0
      i32.const 9
      call $f0
    end
    local.get $p0
    if $I3 (result i32 i64 i32)
      call $f0
      call $f0
      i32.const 1
      call $f0
      call $f0
      call $f0
      i64.const 2
      call $f0
      call $f0
      call $f0
      i32.const 3
      call $f0
    else
      call $f0
      call $f0
      i32.const -1
      call $f0
      call $f0
      call $f0
      i64.const -2
      call $f0
      call $f0
      call $f0
      i32.const -3
      call $f0
    end
    drop
    drop)
  (func $nested (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      local.get $p1
      if $I1
        call $f0
        block $B2
        end
        nop
      end
      local.get $p1
      if $I3
      else
        call $f0
        block $B4
        end
        nop
      end
      local.get $p1
      if $I5 (result i32)
        call $f0
        i32.const 9
      else
        call $f0
        i32.const 10
      end
    else
      local.get $p1
      if $I6
        call $f0
        block $B7
        end
        nop
      end
      local.get $p1
      if $I8
      else
        call $f0
        block $B9
        end
        nop
      end
      local.get $p1
      if $I10 (result i32)
        call $f0
        i32.const 10
      else
        call $f0
        i32.const 11
      end
    end)
  (func $as-select-first (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 1
    else
      call $f0
      i32.const 0
    end
    i32.const 2
    i32.const 3
    select)
  (func $as-select-mid (type $t5) (param $p0 i32) (result i32)
    i32.const 2
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 1
    else
      call $f0
      i32.const 0
    end
    i32.const 3
    select)
  (func $as-select-last (type $t5) (param $p0 i32) (result i32)
    i32.const 2
    i32.const 3
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 1
    else
      call $f0
      i32.const 0
    end
    select)
  (func $as-loop-first (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      call $f0
      call $f0
    end)
  (func $as-loop-mid (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      call $f0
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      call $f0
    end)
  (func $as-loop-last (type $t5) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      call $f0
      call $f0
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
    end)
  (func $as-if-condition (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    if $I1 (result i32)
      call $f0
      i32.const 2
    else
      call $f0
      i32.const 3
    end)
  (func $as-br_if-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      i32.const 2
      br_if $B0
      i32.const 3
      return
    end)
  (func $as-br_if-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 2
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      br_if $B0
      i32.const 3
      return
    end)
  (func $as-br_table-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      i32.const 2
      br_table $B0 $B0
    end)
  (func $as-br_table-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 2
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      br_table $B0 $B0
    end)
  (func $f16 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-first (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 2
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 0
      local.get $p0
      if $I1 (result i32)
        call $f0
        i32.const 1
      else
        call $f0
        i32.const 0
      end
      call_indirect $T0 (type $t0)
    end)
  (func $as-store-first (type $t3) (param $p0 i32)
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 1
    else
      call $f0
      i32.const 0
    end
    i32.const 2
    i32.store)
  (func $as-store-last (type $t3) (param $p0 i32)
    i32.const 2
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 1
    else
      call $f0
      i32.const 0
    end
    i32.store)
  (func $as-memory.grow-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    memory.grow)
  (func $f23 (type $t5) (param $p0 i32) (result i32)
    local.get $p0)
  (func $as-call-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    call $f23)
  (func $as-return-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    return)
  (func $as-drop-operand (type $t3) (param $p0 i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    drop)
  (func $as-br-value (type $t5) (param $p0 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        i32.const 1
      else
        i32.const 0
      end
      br $B0
    end)
  (func $as-local.set-value (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    local.set $p0
    local.get $p0)
  (func $as-local.tee-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    local.tee $p0)
  (func $as-global.set-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 0
    end
    global.set $g0
    global.get $g0)
  (func $as-load-operand (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 11
    else
      i32.const 10
    end
    i32.load)
  (func $as-unary-operand (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 13
    else
      call $f0
      i32.const -13
    end
    i32.ctz)
  (func $as-binary-operand (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 3
    else
      call $f0
      i32.const -3
    end
    local.get $p1
    if $I1 (result i32)
      call $f0
      i32.const 4
    else
      call $f0
      i32.const -5
    end
    i32.mul)
  (func $as-test-operand (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      call $f0
      i32.const 13
    else
      call $f0
      i32.const 0
    end
    i32.eqz)
  (func $as-compare-operand (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0
    if $I0 (result f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    else
      call $f0
      f32.const -0x1.8p+1 (;=-3;)
    end
    local.get $p1
    if $I1 (result f32)
      call $f0
      f32.const 0x1p+2 (;=4;)
    else
      call $f0
      f32.const -0x1p+2 (;=-4;)
    end
    f32.gt)
  (func $as-binary-operands (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32 i32)
      call $f0
      i32.const 3
      call $f0
      i32.const 4
    else
      call $f0
      i32.const 3
      call $f0
      i32.const -4
    end
    i32.mul)
  (func $as-compare-operands (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result f32 f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    else
      call $f0
      f32.const -0x1p+1 (;=-2;)
      call $f0
      f32.const -0x1.8p+1 (;=-3;)
    end
    f32.gt)
  (func $as-mixed-operands (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32 i32)
      call $f0
      i32.const 3
      call $f0
      i32.const 4
    else
      call $f0
      i32.const -3
      call $f0
      i32.const -4
    end
    i32.const 5
    i32.add
    i32.mul)
  (func $break-bare (type $t2) (result i32)
    i32.const 1
    if $I0
      br $I0
      unreachable
    end
    i32.const 1
    if $I1
      br $I1
      unreachable
    else
      unreachable
    end
    i32.const 0
    if $I2
      unreachable
    else
      br $I2
      unreachable
    end
    i32.const 1
    if $I3
      i32.const 1
      br_if $I3
      unreachable
    end
    i32.const 1
    if $I4
      i32.const 1
      br_if $I4
      unreachable
    else
      unreachable
    end
    i32.const 0
    if $I5
      unreachable
    else
      i32.const 1
      br_if $I5
      unreachable
    end
    i32.const 1
    if $I6
      i32.const 0
      br_table $I6
      unreachable
    end
    i32.const 1
    if $I7
      i32.const 0
      br_table $I7
      unreachable
    else
      unreachable
    end
    i32.const 0
    if $I8
      unreachable
    else
      i32.const 0
      br_table $I8
      unreachable
    end
    i32.const 19)
  (func $break-value (type $t5) (param $p0 i32) (result i32)
    local.get $p0
    if $I0 (result i32)
      i32.const 18
      br $I0
      i32.const 19
    else
      i32.const 21
      br $I0
      i32.const 20
    end)
  (func $break-multi-value (type $t10) (param $p0 i32) (result i32 i32 i64)
    local.get $p0
    if $I0 (result i32 i32 i64)
      i32.const 18
      i32.const -18
      i64.const 18
      br $I0
      i32.const 19
      i32.const -19
      i64.const 19
    else
      i32.const -18
      i32.const 18
      i64.const -18
      br $I0
      i32.const -19
      i32.const 19
      i64.const -19
    end)
  (func $param (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    local.get $p0
    if $I0 (param i32) (result i32)
      i32.const 2
      i32.add
    else
      i32.const -2
      i32.add
    end)
  (func $params (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    local.get $p0
    if $I0 (param i32 i32) (result i32)
      i32.add
    else
      i32.sub
    end)
  (func $params-id (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    local.get $p0
    if $I0 (param i32 i32) (result i32 i32)
    end
    i32.add)
  (func $param-break (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    local.get $p0
    if $I0 (param i32) (result i32)
      i32.const 2
      i32.add
      br $I0
    else
      i32.const -2
      i32.add
      br $I0
    end)
  (func $params-break (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    local.get $p0
    if $I0 (param i32 i32) (result i32)
      i32.add
      br $I0
    else
      i32.sub
      br $I0
    end)
  (func $params-id-break (type $t5) (param $p0 i32) (result i32)
    i32.const 1
    i32.const 2
    local.get $p0
    if $I0 (param i32 i32) (result i32 i32)
      br $I0
    end
    i32.add)
  (func $effects (type $t5) (param $p0 i32) (result i32)
    (local $l1 i32)
    block $B0 (result i32)
      i32.const 1
      local.set $l1
      local.get $p0
    end
    if $I1
      local.get $l1
      i32.const 3
      i32.mul
      local.set $l1
      local.get $l1
      i32.const 5
      i32.sub
      local.set $l1
      local.get $l1
      i32.const 7
      i32.mul
      local.set $l1
      br $I1
      local.get $l1
      i32.const 100
      i32.mul
      local.set $l1
    else
      local.get $l1
      i32.const 5
      i32.mul
      local.set $l1
      local.get $l1
      i32.const 7
      i32.sub
      local.set $l1
      local.get $l1
      i32.const 3
      i32.mul
      local.set $l1
      br $I1
      local.get $l1
      i32.const 1000
      i32.mul
      local.set $l1
    end
    local.get $l1)
  (func $add64_u_with_carry (type $t13) (param $p0 i64) (param $p1 i64) (param $p2 i32) (result i64 i32)
    (local $l3 i64)
    local.get $p0
    local.get $p1
    i64.add
    local.get $p2
    i64.extend_i32_u
    i64.add
    local.set $l3
    local.get $l3
    local.get $l3
    local.get $p0
    i64.lt_u
    return)
  (func $add64_u_saturated (type $t14) (param $p0 i64) (param $p1 i64) (result i64)
    local.get $p0
    local.get $p1
    i32.const 0
    call $add64_u_with_carry
    if $I0 (param i64) (result i64)
      drop
      i64.const -1
    end)
  (func $type-use (type $t1)
    i32.const 1
    if $I0
    end
    i32.const 1
    if $I1 (result i32)
      i32.const 0
    else
      i32.const 2
    end
    i32.const 1
    if $I2 (param i32)
      drop
    else
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 1
    if $I3 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop
    i32.const 1
    if $I4 (result i32)
      i32.const 0
    else
      i32.const 2
    end
    i32.const 1
    if $I5 (param i32)
      drop
    else
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 1
    if $I6 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop)
  (func $atypical-condition (type $t1)
    i32.const 0
    if $I0
    end
    i32.const 1
    i32.eqz
    if $I1
    end)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 10))
  (export "empty" (func $empty))
  (export "singular" (func $singular))
  (export "multi" (func $multi))
  (export "nested" (func $nested))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-mid" (func $as-select-mid))
  (export "as-select-last" (func $as-select-last))
  (export "as-loop-first" (func $as-loop-first))
  (export "as-loop-mid" (func $as-loop-mid))
  (export "as-loop-last" (func $as-loop-last))
  (export "as-if-condition" (func $as-if-condition))
  (export "as-br_if-first" (func $as-br_if-first))
  (export "as-br_if-last" (func $as-br_if-last))
  (export "as-br_table-first" (func $as-br_table-first))
  (export "as-br_table-last" (func $as-br_table-last))
  (export "as-call_indirect-first" (func $as-call_indirect-first))
  (export "as-call_indirect-mid" (func $as-call_indirect-mid))
  (export "as-call_indirect-last" (func $as-call_indirect-last))
  (export "as-store-first" (func $as-store-first))
  (export "as-store-last" (func $as-store-last))
  (export "as-memory.grow-value" (func $as-memory.grow-value))
  (export "as-call-value" (func $as-call-value))
  (export "as-return-value" (func $as-return-value))
  (export "as-drop-operand" (func $as-drop-operand))
  (export "as-br-value" (func $as-br-value))
  (export "as-local.set-value" (func $as-local.set-value))
  (export "as-local.tee-value" (func $as-local.tee-value))
  (export "as-global.set-value" (func $as-global.set-value))
  (export "as-load-operand" (func $as-load-operand))
  (export "as-unary-operand" (func $as-unary-operand))
  (export "as-binary-operand" (func $as-binary-operand))
  (export "as-test-operand" (func $as-test-operand))
  (export "as-compare-operand" (func $as-compare-operand))
  (export "as-binary-operands" (func $as-binary-operands))
  (export "as-compare-operands" (func $as-compare-operands))
  (export "as-mixed-operands" (func $as-mixed-operands))
  (export "break-bare" (func $break-bare))
  (export "break-value" (func $break-value))
  (export "break-multi-value" (func $break-multi-value))
  (export "param" (func $param))
  (export "params" (func $params))
  (export "params-id" (func $params-id))
  (export "param-break" (func $param-break))
  (export "params-break" (func $params-break))
  (export "params-id-break" (func $params-id-break))
  (export "effects" (func $effects))
  (export "add64_u_with_carry" (func $add64_u_with_carry))
  (export "add64_u_saturated" (func $add64_u_saturated))
  (export "type-use" (func $type-use))
  (export "atypical-condition" (func $atypical-condition))
  (elem $e0 (i32.const 0) func $f16))
