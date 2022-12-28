(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i64) (result i64)))
  (type $t2 (func (result i32)))
  (func $stmt (type $t0) (param $p0 i32) (result i32)
    (local $l1 i32)
    i32.const 100
    local.set $l1
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    block $B8
                      block $B9
                        local.get $p0
                        br_table $B9 $B8 $B7 $B6 $B5 $B4 $B3 $B1 $B2
                      end
                      local.get $p0
                      return
                    end
                    nop
                  end
                end
                i32.const 0
                local.get $p0
                i32.sub
                local.set $l1
                br $B0
              end
              br $B0
            end
            i32.const 101
            local.set $l1
            br $B0
          end
          i32.const 101
          local.set $l1
        end
        i32.const 102
        local.set $l1
      end
    end
    local.get $l1
    return)
  (func $expr (type $t1) (param $p0 i64) (result i64)
    (local $l1 i64)
    i64.const 100
    local.set $l1
    block $B0 (result i64)
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    block $B8
                      block $B9
                        local.get $p0
                        i32.wrap_i64
                        br_table $B9 $B8 $B7 $B6 $B3 $B4 $B5 $B1 $B2
                      end
                      local.get $p0
                      return
                    end
                    nop
                  end
                end
                i64.const 0
                local.get $p0
                i64.sub
                br $B0
              end
              i64.const 101
              local.set $l1
            end
          end
        end
        local.get $l1
        br $B0
      end
      i64.const -5
    end
    return)
  (func $arg (type $t0) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 10
      block $B1 (result i32)
        i32.const 100
        block $B2 (result i32)
          i32.const 1000
          block $B3 (result i32)
            i32.const 2
            local.get $p0
            i32.mul
            i32.const 3
            local.get $p0
            i32.and
            br_table $B2 $B1 $B0 $B3
          end
          i32.add
        end
        i32.add
      end
      i32.add
    end
    return)
  (func $corner (type $t2) (result i32)
    block $B0
      i32.const 0
      br_table $B0
    end
    i32.const 1)
  (export "stmt" (func $stmt))
  (export "expr" (func $expr))
  (export "arg" (func $arg))
  (export "corner" (func $corner)))
