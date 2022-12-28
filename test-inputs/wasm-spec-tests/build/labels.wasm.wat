(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32) (result i32)))
  (func $block (type $t0) (result i32)
    block $B0 (result i32)
      i32.const 1
      br $B0
      i32.const 0
    end)
  (func $loop1 (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0 (result i32)
      loop $L1 (result i32)
        local.get $l0
        i32.const 1
        i32.add
        local.set $l0
        local.get $l0
        i32.const 5
        i32.eq
        if $I2
          local.get $l0
          br $B0
        end
        br $L1
      end
    end)
  (func $loop2 (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0 (result i32)
      loop $L1 (result i32)
        local.get $l0
        i32.const 1
        i32.add
        local.set $l0
        local.get $l0
        i32.const 5
        i32.eq
        if $I2
          br $L1
        end
        local.get $l0
        i32.const 8
        i32.eq
        if $I3
          local.get $l0
          br $B0
        end
        local.get $l0
        i32.const 1
        i32.add
        local.set $l0
        br $L1
      end
    end)
  (func $loop3 (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0 (result i32)
      loop $L1 (result i32)
        local.get $l0
        i32.const 1
        i32.add
        local.set $l0
        local.get $l0
        i32.const 5
        i32.eq
        if $I2
          local.get $l0
          br $B0
        end
        local.get $l0
      end
    end)
  (func $loop4 (type $t1) (param $p0 i32) (result i32)
    (local $l1 i32)
    i32.const 1
    local.set $l1
    block $B0 (result i32)
      loop $L1 (result i32)
        local.get $l1
        local.get $l1
        i32.add
        local.set $l1
        local.get $l1
        local.get $p0
        i32.gt_u
        if $I2
          local.get $l1
          br $B0
        end
        br $L1
      end
    end)
  (func $loop5 (type $t0) (result i32)
    loop $L0 (result i32)
      i32.const 1
    end
    i32.const 1
    i32.add)
  (func $loop6 (type $t0) (result i32)
    loop $L0 (result i32)
      i32.const 0
      br_if $L0
      i32.const 3
    end)
  (func $if (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0
      i32.const 1
      if $I1
        br $I1
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 1
      if $I2
        br $I2
        i32.const 666
        local.set $l0
      else
        i32.const 888
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 1
      if $I3
        br $I3
        i32.const 666
        local.set $l0
      else
        i32.const 888
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 0
      if $I4
        i32.const 888
        local.set $l0
      else
        br $I4
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 0
      if $I5
        i32.const 888
        local.set $l0
      else
        br $I5
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
    end
    local.get $l0)
  (func $if2 (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0
      i32.const 1
      if $I1
        br $I1
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 1
      if $I2
        br $I2
        i32.const 666
        local.set $l0
      else
        i32.const 888
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 1
      if $I3
        br $I3
        i32.const 666
        local.set $l0
      else
        i32.const 888
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 0
      if $I4
        i32.const 888
        local.set $l0
      else
        br $I4
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
      i32.const 0
      if $I5
        i32.const 888
        local.set $l0
      else
        br $I5
        i32.const 666
        local.set $l0
      end
      local.get $l0
      i32.const 1
      i32.add
      local.set $l0
    end
    local.get $l0)
  (func $switch (type $t1) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 10
      block $B1 (result i32)
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  local.get $p0
                  br_table $B2 $B6 $B5 $B4 $B3
                end
              end
              i32.const 2
              br $B1
            end
            i32.const 3
            br $B0
          end
        end
        i32.const 5
      end
      i32.mul
    end)
  (func $return (type $t1) (param $p0 i32) (result i32)
    block $B0
      block $B1
        block $B2
          local.get $p0
          br_table $B2 $B1
          br $B0
        end
        i32.const 0
        return
      end
    end
    i32.const 2)
  (func $br_if0 (type $t0) (result i32)
    (local $l0 i32)
    i32.const 0
    local.set $l0
    block $B0 (result i32)
      block $B1
        i32.const 0
        br_if $B1
        local.get $l0
        i32.const 1
        i32.or
        local.set $l0
        i32.const 1
        br_if $B1
        local.get $l0
        i32.const 2
        i32.or
        local.set $l0
      end
      block $B2 (result i32)
        local.get $l0
        i32.const 4
        i32.or
        local.set $l0
        local.get $l0
      end
      i32.const 0
      br_if $B0
      drop
      local.get $l0
      i32.const 8
      i32.or
      local.set $l0
      block $B3 (result i32)
        local.get $l0
        i32.const 16
        i32.or
        local.set $l0
        local.get $l0
      end
      i32.const 1
      br_if $B0
      drop
      local.get $l0
      i32.const 32
      i32.or
      local.set $l0
      local.get $l0
    end)
  (func $br_if1 (type $t0) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
        br $B1
      end
      i32.const 1
      br_if $B0
      drop
      i32.const 0
    end)
  (func $br_if2 (type $t0) (result i32)
    block $B0 (result i32)
      i32.const 1
      if $I1
        block $B2 (result i32)
          i32.const 1
          br $B2
        end
        i32.const 1
        br_if $B0
        drop
      end
      i32.const 0
    end)
  (func $br_if3 (type $t0) (result i32)
    (local $l0 i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 1
        local.set $l0
        local.get $l0
      end
      block $B2 (result i32)
        i32.const 2
        local.set $l0
        local.get $l0
      end
      br_if $B0
      drop
      i32.const 0
    end
    i32.const 0
    i32.add
    drop
    local.get $l0)
  (func $br (type $t0) (result i32)
    block $B0 (result i32)
      i32.const 1
      if $I1
        block $B2 (result i32)
          i32.const 1
          br $B2
        end
        br $B0
      else
        block $B3
          block $B4 (result i32)
            i32.const 1
            br $B4
          end
          drop
        end
      end
      i32.const 1
    end)
  (func $shadowing (type $t0) (result i32)
    block $B0 (result i32)
      i32.const 1
      br $B0
      i32.const 2
      i32.xor
    end)
  (func $redefinition (type $t0) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 2
      end
      block $B2 (result i32)
        i32.const 3
        br $B2
      end
      i32.add
    end)
  (export "block" (func $block))
  (export "loop1" (func $loop1))
  (export "loop2" (func $loop2))
  (export "loop3" (func $loop3))
  (export "loop4" (func $loop4))
  (export "loop5" (func $loop5))
  (export "loop6" (func $loop6))
  (export "if" (func $if))
  (export "if2" (func $if2))
  (export "switch" (func $switch))
  (export "return" (func $return))
  (export "br_if0" (func $br_if0))
  (export "br_if1" (func $br_if1))
  (export "br_if2" (func $br_if2))
  (export "br_if3" (func $br_if3))
  (export "br" (func $br))
  (export "shadowing" (func $shadowing))
  (export "redefinition" (func $redefinition)))
