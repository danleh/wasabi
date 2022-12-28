(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (param i32)))
  (type $t4 (func (param i32 f64 i32) (result i32 f64 i32)))
  (type $t5 (func (result i32 i64 i32)))
  (type $t6 (func (param i32) (result i32)))
  (type $t7 (func (result i32 i32)))
  (type $t8 (func (result f32 f32)))
  (type $t9 (func (result i32 i32 i64)))
  (type $t10 (func (param i32 i32 i64)))
  (type $t11 (func (param i32 i32) (result i32 i32)))
  (type $t12 (func (param i64) (result i64)))
  (type $t13 (func (param f32 f32) (result f32)))
  (func $f0 (type $t1))
  (func $empty (type $t1)
    loop $L0
    end
    loop $L1
    end)
  (func $singular (type $t2) (result i32)
    loop $L0
      nop
    end
    loop $L1 (result i32)
      i32.const 7
    end)
  (func $multi (type $t2) (result i32)
    loop $L0
      call $f0
      call $f0
      call $f0
      call $f0
    end
    loop $L1 (result i32)
      call $f0
      call $f0
      i32.const 8
      call $f0
    end
    drop
    loop $L2 (result i32 i64 i32)
      call $f0
      call $f0
      call $f0
      i32.const 8
      call $f0
      call $f0
      call $f0
      call $f0
      i64.const 7
      call $f0
      call $f0
      call $f0
      call $f0
      i32.const 9
      call $f0
    end
    drop
    drop)
  (func $nested (type $t2) (result i32)
    loop $L0 (result i32)
      loop $L1
        call $f0
        block $B2
        end
        nop
      end
      loop $L3 (result i32)
        call $f0
        i32.const 9
      end
    end)
  (func $deep (type $t2) (result i32)
    loop $L0 (result i32)
      block $B1 (result i32)
        loop $L2 (result i32)
          block $B3 (result i32)
            loop $L4 (result i32)
              block $B5 (result i32)
                loop $L6 (result i32)
                  block $B7 (result i32)
                    loop $L8 (result i32)
                      block $B9 (result i32)
                        loop $L10 (result i32)
                          block $B11 (result i32)
                            loop $L12 (result i32)
                              block $B13 (result i32)
                                loop $L14 (result i32)
                                  block $B15 (result i32)
                                    loop $L16 (result i32)
                                      block $B17 (result i32)
                                        loop $L18 (result i32)
                                          block $B19 (result i32)
                                            loop $L20 (result i32)
                                              block $B21 (result i32)
                                                loop $L22 (result i32)
                                                  block $B23 (result i32)
                                                    loop $L24 (result i32)
                                                      block $B25 (result i32)
                                                        loop $L26 (result i32)
                                                          block $B27 (result i32)
                                                            loop $L28 (result i32)
                                                              block $B29 (result i32)
                                                                loop $L30 (result i32)
                                                                  block $B31 (result i32)
                                                                    loop $L32 (result i32)
                                                                      block $B33 (result i32)
                                                                        loop $L34 (result i32)
                                                                          block $B35 (result i32)
                                                                            loop $L36 (result i32)
                                                                              block $B37 (result i32)
                                                                                loop $L38 (result i32)
                                                                                  block $B39 (result i32)
                                                                                    call $f0
                                                                                    i32.const 150
                                                                                  end
                                                                                end
                                                                              end
                                                                            end
                                                                          end
                                                                        end
                                                                      end
                                                                    end
                                                                  end
                                                                end
                                                              end
                                                            end
                                                          end
                                                        end
                                                      end
                                                    end
                                                  end
                                                end
                                              end
                                            end
                                          end
                                        end
                                      end
                                    end
                                  end
                                end
                              end
                            end
                          end
                        end
                      end
                    end
                  end
                end
              end
            end
          end
        end
      end
    end)
  (func $as-select-first (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    i32.const 2
    i32.const 3
    select)
  (func $as-select-mid (type $t2) (result i32)
    i32.const 2
    loop $L0 (result i32)
      i32.const 1
    end
    i32.const 3
    select)
  (func $as-select-last (type $t2) (result i32)
    i32.const 2
    i32.const 3
    loop $L0 (result i32)
      i32.const 1
    end
    select)
  (func $as-if-condition (type $t1)
    loop $L0 (result i32)
      i32.const 1
    end
    if $I1
      call $f0
    end)
  (func $as-if-then (type $t2) (result i32)
    i32.const 1
    if $I0 (result i32)
      loop $L1 (result i32)
        i32.const 1
      end
    else
      i32.const 2
    end)
  (func $as-if-else (type $t2) (result i32)
    i32.const 1
    if $I0 (result i32)
      i32.const 2
    else
      loop $L1 (result i32)
        i32.const 1
      end
    end)
  (func $as-br_if-first (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 1
      end
      i32.const 2
      br_if $B0
    end)
  (func $as-br_if-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      loop $L1 (result i32)
        i32.const 1
      end
      br_if $B0
    end)
  (func $as-br_table-first (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 1
      end
      i32.const 2
      br_table $B0 $B0
    end)
  (func $as-br_table-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      loop $L1 (result i32)
        i32.const 1
      end
      br_table $B0 $B0
    end)
  (func $f16 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-first (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 1
      end
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      loop $L1 (result i32)
        i32.const 1
      end
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      loop $L1 (result i32)
        i32.const 0
      end
      call_indirect $T0 (type $t0)
    end)
  (func $as-store-first (type $t1)
    loop $L0 (result i32)
      i32.const 1
    end
    i32.const 1
    i32.store)
  (func $as-store-last (type $t1)
    i32.const 10
    loop $L0 (result i32)
      i32.const 1
    end
    i32.store)
  (func $as-memory.grow-value (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    memory.grow)
  (func $f23 (type $t6) (param $p0 i32) (result i32)
    local.get $p0)
  (func $as-call-value (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    call $f23)
  (func $as-return-value (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    return)
  (func $as-drop-operand (type $t1)
    loop $L0 (result i32)
      i32.const 1
    end
    drop)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 1
      end
      br $B0
    end)
  (func $as-local.set-value (type $t2) (result i32)
    (local $l0 i32)
    loop $L0 (result i32)
      i32.const 1
    end
    local.set $l0
    local.get $l0)
  (func $as-local.tee-value (type $t2) (result i32)
    (local $l0 i32)
    loop $L0 (result i32)
      i32.const 1
    end
    local.tee $l0)
  (func $as-global.set-value (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    global.set $g0
    global.get $g0)
  (func $as-load-operand (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    i32.load)
  (func $as-unary-operand (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 13
    end
    i32.ctz)
  (func $as-binary-operand (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 3
    end
    loop $L1 (result i32)
      call $f0
      i32.const 4
    end
    i32.mul)
  (func $as-test-operand (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 13
    end
    i32.eqz)
  (func $as-compare-operand (type $t2) (result i32)
    loop $L0 (result f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    loop $L1 (result f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    f32.gt)
  (func $as-binary-operands (type $t2) (result i32)
    loop $L0 (result i32 i32)
      call $f0
      i32.const 3
      call $f0
      i32.const 4
    end
    i32.mul)
  (func $as-compare-operands (type $t2) (result i32)
    loop $L0 (result f32 f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    f32.gt)
  (func $as-mixed-operands (type $t2) (result i32)
    loop $L0 (result i32 i32)
      call $f0
      i32.const 3
      call $f0
      i32.const 4
    end
    i32.const 5
    i32.add
    i32.mul)
  (func $break-bare (type $t2) (result i32)
    block $B0
      loop $L1
        br $B0
        br $L1
        unreachable
      end
    end
    block $B2
      loop $L3
        i32.const 1
        br_if $B2
        unreachable
      end
    end
    block $B4
      loop $L5
        i32.const 0
        br_table $B4
        unreachable
      end
    end
    block $B6
      loop $L7
        i32.const 1
        br_table $B6 $B6 $B6
        unreachable
      end
    end
    i32.const 19)
  (func $break-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      loop $L1 (param i32)
        block $B2
          i32.const 18
          br $B0
        end
        i32.const 20
        br $L1
      end
      i32.const 19
    end)
  (func $break-multi-value (type $t9) (result i32 i32 i64)
    block $B0 (result i32 i32 i64)
      i32.const 0
      i32.const 0
      i64.const 0
      loop $L1 (param i32 i32 i64)
        block $B2
          i32.const 18
          i32.const -18
          i64.const 18
          br $B0
        end
        i32.const 20
        i32.const -20
        i64.const 20
        br $L1
      end
      i32.const 19
      i32.const -19
      i64.const 19
    end)
  (func $break-repeated (type $t2) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 18
        br $B0
        i32.const 19
        br $B0
        i32.const 20
        i32.const 0
        br_if $B0
        drop
        i32.const 20
        i32.const 1
        br_if $B0
        drop
        i32.const 21
        br $B0
        i32.const 22
        i32.const 0
        br_table $B0
        i32.const 23
        i32.const 1
        br_table $B0 $B0 $B0
        i32.const 21
      end
    end)
  (func $break-inner (type $t2) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    local.get $l0
    block $B0 (result i32)
      loop $L1 (result i32)
        block $B2 (result i32)
          i32.const 1
          br $B0
        end
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B3 (result i32)
      loop $L4 (result i32)
        loop $L5 (result i32)
          i32.const 2
          br $B3
        end
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B6 (result i32)
      loop $L7 (result i32)
        block $B8 (result i32)
          loop $L9 (result i32)
            i32.const 4
            br $B8
          end
        end
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B10 (result i32)
      loop $L11 (result i32)
        i32.const 8
        br $B10
        i32.ctz
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B12 (result i32)
      loop $L13 (result i32)
        loop $L14 (result i32)
          i32.const 16
          br $B12
        end
        i32.ctz
      end
    end
    i32.add
    local.set $l0
    local.get $l0)
  (func $cont-inner (type $t2) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    local.get $l0
    loop $L0 (result i32)
      loop $L1 (result i32)
        br $L0
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    loop $L2 (result i32)
      br $L2
      i32.ctz
    end
    i32.add
    local.set $l0
    local.get $l0
    loop $L3 (result i32)
      loop $L4 (result i32)
        br $L3
      end
      i32.ctz
    end
    i32.add
    local.set $l0
    local.get $l0)
  (func $param (type $t2) (result i32)
    i32.const 1
    loop $L0 (param i32) (result i32)
      i32.const 2
      i32.add
    end)
  (func $params (type $t2) (result i32)
    i32.const 1
    i32.const 2
    loop $L0 (param i32 i32) (result i32)
      i32.add
    end)
  (func $params-id (type $t2) (result i32)
    i32.const 1
    i32.const 2
    loop $L0 (param i32 i32) (result i32 i32)
    end
    i32.add)
  (func $param-break (type $t2) (result i32)
    (local $l0 i32)
    i32.const 1
    loop $L0 (param i32) (result i32)
      i32.const 4
      i32.add
      local.tee $l0
      local.get $l0
      i32.const 10
      i32.lt_u
      br_if $L0
    end)
  (func $params-break (type $t2) (result i32)
    (local $l0 i32)
    i32.const 1
    i32.const 2
    loop $L0 (param i32 i32) (result i32)
      i32.add
      local.tee $l0
      i32.const 3
      local.get $l0
      i32.const 10
      i32.lt_u
      br_if $L0
      drop
    end)
  (func $params-id-break (type $t2) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    i32.const 1
    i32.const 2
    loop $L0 (param i32 i32) (result i32 i32)
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      local.get $l0
      i32.const 10
      i32.lt_u
      br_if $L0
    end
    i32.add)
  (func $effects (type $t2) (result i32)
    (local $l0 i32)
    block $B0
      loop $L1
        i32.const 1
        local.set $l0
        local.get $l0
        i32.const 3
        i32.mul
        local.set $l0
        local.get $l0
        i32.const 5
        i32.sub
        local.set $l0
        local.get $l0
        i32.const 7
        i32.mul
        local.set $l0
        br $B0
        local.get $l0
        i32.const 100
        i32.mul
        local.set $l0
      end
    end
    local.get $l0
    i32.const -14
    i32.eq)
  (func $while (type $t12) (param $p0 i64) (result i64)
    (local $l1 i64)
    i64.const 1
    local.set $l1
    block $B0
      loop $L1
        local.get $p0
        i64.eqz
        br_if $B0
        local.get $p0
        local.get $l1
        i64.mul
        local.set $l1
        local.get $p0
        i64.const 1
        i64.sub
        local.set $p0
        br $L1
      end
    end
    local.get $l1)
  (func $for (type $t12) (param $p0 i64) (result i64)
    (local $l1 i64) (local $l2 i64)
    i64.const 1
    local.set $l1
    i64.const 2
    local.set $l2
    block $B0
      loop $L1
        local.get $l2
        local.get $p0
        i64.gt_u
        br_if $B0
        local.get $l1
        local.get $l2
        i64.mul
        local.set $l1
        local.get $l2
        i64.const 1
        i64.add
        local.set $l2
        br $L1
      end
    end
    local.get $l1)
  (func $nesting (type $t13) (param $p0 f32) (param $p1 f32) (result f32)
    (local $l2 f32) (local $l3 f32)
    block $B0
      loop $L1
        local.get $p0
        f32.const 0x0p+0 (;=0;)
        f32.eq
        br_if $B0
        local.get $p1
        local.set $l2
        block $B2
          loop $L3
            local.get $l2
            f32.const 0x0p+0 (;=0;)
            f32.eq
            br_if $B2
            local.get $l2
            f32.const 0x0p+0 (;=0;)
            f32.lt
            br_if $B0
            local.get $l3
            local.get $l2
            f32.add
            local.set $l3
            local.get $l2
            f32.const 0x1p+1 (;=2;)
            f32.sub
            local.set $l2
            br $L3
          end
        end
        local.get $l3
        local.get $p0
        f32.div
        local.set $l3
        local.get $p0
        f32.const 0x1p+0 (;=1;)
        f32.sub
        local.set $p0
        br $L1
      end
    end
    local.get $l3)
  (func $type-use (type $t1)
    loop $L0
    end
    loop $L1 (result i32)
      i32.const 0
    end
    loop $L2 (param i32)
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    loop $L3 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop
    loop $L4 (result i32)
      i32.const 0
    end
    loop $L5 (param i32)
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    loop $L6 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 0))
  (export "empty" (func $empty))
  (export "singular" (func $singular))
  (export "multi" (func $multi))
  (export "nested" (func $nested))
  (export "deep" (func $deep))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-mid" (func $as-select-mid))
  (export "as-select-last" (func $as-select-last))
  (export "as-if-condition" (func $as-if-condition))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else))
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
  (export "break-repeated" (func $break-repeated))
  (export "break-inner" (func $break-inner))
  (export "cont-inner" (func $cont-inner))
  (export "param" (func $param))
  (export "params" (func $params))
  (export "params-id" (func $params-id))
  (export "param-break" (func $param-break))
  (export "params-break" (func $params-break))
  (export "params-id-break" (func $params-id-break))
  (export "effects" (func $effects))
  (export "while" (func $while))
  (export "for" (func $for))
  (export "nesting" (func $nesting))
  (export "type-use" (func $type-use))
  (elem $e0 (i32.const 0) func $f16))
