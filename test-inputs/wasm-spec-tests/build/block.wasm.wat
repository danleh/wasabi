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
  (type $t10 (func (param i32 i32) (result i32 i32)))
  (func $f0 (type $t1))
  (func $empty (type $t1)
    block $B0
    end
    block $B1
    end)
  (func $singular (type $t2) (result i32)
    block $B0
      nop
    end
    block $B1 (result i32)
      i32.const 7
    end)
  (func $multi (type $t2) (result i32)
    block $B0
      call $f0
      call $f0
      call $f0
      call $f0
    end
    block $B1 (result i32)
      call $f0
      call $f0
      call $f0
      i32.const 7
      call $f0
    end
    drop
    block $B2 (result i32 i64 i32)
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
    block $B0 (result i32)
      block $B1
        call $f0
        block $B2
        end
        nop
      end
      block $B3 (result i32)
        call $f0
        i32.const 9
      end
    end)
  (func $deep (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        block $B2 (result i32)
          block $B3 (result i32)
            block $B4 (result i32)
              block $B5 (result i32)
                block $B6 (result i32)
                  block $B7 (result i32)
                    block $B8 (result i32)
                      block $B9 (result i32)
                        block $B10 (result i32)
                          block $B11 (result i32)
                            block $B12 (result i32)
                              block $B13 (result i32)
                                block $B14 (result i32)
                                  block $B15 (result i32)
                                    block $B16 (result i32)
                                      block $B17 (result i32)
                                        block $B18 (result i32)
                                          block $B19 (result i32)
                                            block $B20 (result i32)
                                              block $B21 (result i32)
                                                block $B22 (result i32)
                                                  block $B23 (result i32)
                                                    block $B24 (result i32)
                                                      block $B25 (result i32)
                                                        block $B26 (result i32)
                                                          block $B27 (result i32)
                                                            block $B28 (result i32)
                                                              block $B29 (result i32)
                                                                block $B30 (result i32)
                                                                  block $B31 (result i32)
                                                                    block $B32 (result i32)
                                                                      block $B33 (result i32)
                                                                        block $B34 (result i32)
                                                                          block $B35 (result i32)
                                                                            block $B36 (result i32)
                                                                              block $B37 (result i32)
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
    end)
  (func $as-select-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    i32.const 2
    i32.const 3
    select)
  (func $as-select-mid (type $t2) (result i32)
    i32.const 2
    block $B0 (result i32)
      i32.const 1
    end
    i32.const 3
    select)
  (func $as-select-last (type $t2) (result i32)
    i32.const 2
    i32.const 3
    block $B0 (result i32)
      i32.const 1
    end
    select)
  (func $as-loop-first (type $t2) (result i32)
    loop $L0 (result i32)
      block $B1 (result i32)
        i32.const 1
      end
      call $f0
      call $f0
    end)
  (func $as-loop-mid (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      block $B1 (result i32)
        i32.const 1
      end
      call $f0
    end)
  (func $as-loop-last (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      call $f0
      block $B1 (result i32)
        i32.const 1
      end
    end)
  (func $as-if-condition (type $t1)
    block $B0 (result i32)
      i32.const 1
    end
    if $I1
      call $f0
    end)
  (func $as-if-then (type $t2) (result i32)
    i32.const 1
    if $I0 (result i32)
      block $B1 (result i32)
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
      block $B1 (result i32)
        i32.const 1
      end
    end)
  (func $as-br_if-first (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
      end
      i32.const 2
      br_if $B0
    end)
  (func $as-br_if-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      block $B1 (result i32)
        i32.const 1
      end
      br_if $B0
    end)
  (func $as-br_table-first (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
      end
      i32.const 2
      br_table $B0 $B0
    end)
  (func $as-br_table-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      block $B1 (result i32)
        i32.const 1
      end
      br_table $B0 $B0
    end)
  (func $f19 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-first (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
      end
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      block $B1 (result i32)
        i32.const 1
      end
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      block $B1 (result i32)
        i32.const 0
      end
      call_indirect $T0 (type $t0)
    end)
  (func $as-store-first (type $t1)
    block $B0 (result i32)
      i32.const 1
    end
    i32.const 1
    i32.store)
  (func $as-store-last (type $t1)
    i32.const 10
    block $B0 (result i32)
      i32.const 1
    end
    i32.store)
  (func $as-memory.grow-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    memory.grow)
  (func $f26 (type $t6) (param $p0 i32) (result i32)
    local.get $p0)
  (func $as-call-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    call $f26)
  (func $as-return-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    return)
  (func $as-drop-operand (type $t1)
    block $B0 (result i32)
      i32.const 1
    end
    drop)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
      end
      br $B0
    end)
  (func $as-local.set-value (type $t2) (result i32)
    (local $l0 i32)
    block $B0 (result i32)
      i32.const 1
    end
    local.set $l0
    local.get $l0)
  (func $as-local.tee-value (type $t2) (result i32)
    (local $l0 i32)
    block $B0 (result i32)
      i32.const 1
    end
    local.tee $l0)
  (func $as-global.set-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    global.set $g0
    global.get $g0)
  (func $as-load-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
    end
    i32.load)
  (func $as-unary-operand (type $t2) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 13
    end
    i32.ctz)
  (func $as-binary-operand (type $t2) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 3
    end
    block $B1 (result i32)
      call $f0
      i32.const 4
    end
    i32.mul)
  (func $as-test-operand (type $t2) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 13
    end
    i32.eqz)
  (func $as-compare-operand (type $t2) (result i32)
    block $B0 (result f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    block $B1 (result f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    f32.gt)
  (func $as-binary-operands (type $t2) (result i32)
    block $B0 (result i32 i32)
      call $f0
      i32.const 3
      call $f0
      i32.const 4
    end
    i32.mul)
  (func $as-compare-operands (type $t2) (result i32)
    block $B0 (result f32 f32)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
      call $f0
      f32.const 0x1.8p+1 (;=3;)
    end
    f32.gt)
  (func $as-mixed-operands (type $t2) (result i32)
    block $B0 (result i32 i32)
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
      br $B0
      unreachable
    end
    block $B1
      i32.const 1
      br_if $B1
      unreachable
    end
    block $B2
      i32.const 0
      br_table $B2
      unreachable
    end
    block $B3
      i32.const 1
      br_table $B3 $B3 $B3
      unreachable
    end
    i32.const 19)
  (func $break-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 18
      br $B0
      i32.const 19
    end)
  (func $break-multi-value (type $t9) (result i32 i32 i64)
    block $B0 (result i32 i32 i64)
      i32.const 18
      i32.const -18
      i64.const 18
      br $B0
      i32.const 19
      i32.const -19
      i64.const 19
    end)
  (func $break-repeated (type $t2) (result i32)
    block $B0 (result i32)
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
      i32.const 4
      br_table $B0
      i32.const 23
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 21
    end)
  (func $break-inner (type $t2) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    local.get $l0
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
        br $B0
      end
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B2 (result i32)
      block $B3
        br $B3
      end
      i32.const 2
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B4 (result i32)
      i32.const 4
      br $B4
      i32.ctz
    end
    i32.add
    local.set $l0
    local.get $l0
    block $B5 (result i32)
      block $B6 (result i32)
        i32.const 8
        br $B5
      end
      i32.ctz
    end
    i32.add
    local.set $l0
    local.get $l0)
  (func $param (type $t2) (result i32)
    i32.const 1
    block $B0 (param i32) (result i32)
      i32.const 2
      i32.add
    end)
  (func $params (type $t2) (result i32)
    i32.const 1
    i32.const 2
    block $B0 (param i32 i32) (result i32)
      i32.add
    end)
  (func $params-id (type $t2) (result i32)
    i32.const 1
    i32.const 2
    block $B0 (param i32 i32) (result i32 i32)
    end
    i32.add)
  (func $param-break (type $t2) (result i32)
    i32.const 1
    block $B0 (param i32) (result i32)
      i32.const 2
      i32.add
      br $B0
    end)
  (func $params-break (type $t2) (result i32)
    i32.const 1
    i32.const 2
    block $B0 (param i32 i32) (result i32)
      i32.add
      br $B0
    end)
  (func $params-id-break (type $t2) (result i32)
    i32.const 1
    i32.const 2
    block $B0 (param i32 i32) (result i32 i32)
      br $B0
    end
    i32.add)
  (func $effects (type $t2) (result i32)
    (local $l0 i32)
    block $B0
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
    local.get $l0
    i32.const -14
    i32.eq)
  (func $type-use (type $t1)
    block $B0
    end
    block $B1 (result i32)
      i32.const 0
    end
    block $B2 (param i32)
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    block $B3 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop
    block $B4 (result i32)
      i32.const 0
    end
    block $B5 (param i32)
      drop
    end
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    block $B6 (param i32 f64 i32) (result i32 f64 i32)
    end
    drop
    drop
    drop)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 10))
  (export "empty" (func $empty))
  (export "singular" (func $singular))
  (export "multi" (func $multi))
  (export "nested" (func $nested))
  (export "deep" (func $deep))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-mid" (func $as-select-mid))
  (export "as-select-last" (func $as-select-last))
  (export "as-loop-first" (func $as-loop-first))
  (export "as-loop-mid" (func $as-loop-mid))
  (export "as-loop-last" (func $as-loop-last))
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
  (export "param" (func $param))
  (export "params" (func $params))
  (export "params-id" (func $params-id))
  (export "param-break" (func $param-break))
  (export "params-break" (func $params-break))
  (export "params-id-break" (func $params-id-break))
  (export "effects" (func $effects))
  (export "type-use" (func $type-use))
  (elem $e0 (i32.const 0) func $f19))
