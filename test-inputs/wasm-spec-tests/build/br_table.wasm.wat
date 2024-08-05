(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func))
  (type $t2 (func (result i32)))
  (type $t3 (func (result i64)))
  (type $t4 (func (result f32)))
  (type $t5 (func (result f64)))
  (type $t6 (func (param i32) (result i32)))
  (type $t7 (func (param i32 i32) (result i32)))
  (type $t8 (func (param i32 externref) (result externref)))
  (func $f0 (type $t1))
  (func $type-i32 (type $t1)
    block $B0
      i32.const 0
      br_table $B0 $B0
      i32.ctz
      drop
    end)
  (func $type-i64 (type $t1)
    block $B0
      i32.const 0
      br_table $B0 $B0
      i64.ctz
      drop
    end)
  (func $type-f32 (type $t1)
    block $B0
      i32.const 0
      br_table $B0 $B0
      f32.neg
      drop
    end)
  (func $type-f64 (type $t1)
    block $B0
      i32.const 0
      br_table $B0 $B0
      f64.neg
      drop
    end)
  (func $type-i32-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 0
      br_table $B0 $B0
      i32.ctz
    end)
  (func $type-i64-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 2
      i32.const 0
      br_table $B0 $B0
      i64.ctz
    end)
  (func $type-f32-value (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.8p+1 (;=3;)
      i32.const 0
      br_table $B0 $B0
      f32.neg
    end)
  (func $type-f64-value (type $t5) (result f64)
    block $B0 (result f64)
      f64.const 0x1p+2 (;=4;)
      i32.const 0
      br_table $B0 $B0
      f64.neg
    end)
  (func $empty (type $t6) (param $p0 i32) (result i32)
    block $B0
      local.get $p0
      br_table $B0
      i32.const 21
      return
    end
    i32.const 22)
  (func $empty-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 33
      local.get $p0
      br_table $B0
      i32.const 31
    end)
  (func $singleton (type $t6) (param $p0 i32) (result i32)
    block $B0
      block $B1
        local.get $p0
        br_table $B0 $B1
        i32.const 21
        return
      end
      i32.const 20
      return
    end
    i32.const 22)
  (func $singleton-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      block $B1 (result i32)
        i32.const 33
        local.get $p0
        br_table $B1 $B0
        i32.const 31
        return
      end
      drop
      i32.const 32
    end)
  (func $multiple (type $t6) (param $p0 i32) (result i32)
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              local.get $p0
              br_table $B1 $B2 $B3 $B4 $B0
              i32.const 99
              return
            end
            i32.const 100
            return
          end
          i32.const 101
          return
        end
        i32.const 102
        return
      end
      i32.const 103
      return
    end
    i32.const 104)
  (func $multiple-value (type $t6) (param $p0 i32) (result i32)
    (local $l1 i32)
    block $B0 (result i32)
      block $B1 (result i32)
        block $B2 (result i32)
          block $B3 (result i32)
            block $B4 (result i32)
              i32.const 200
              local.get $p0
              br_table $B1 $B2 $B3 $B4 $B0
              local.get $l1
              i32.const 99
              i32.add
              return
            end
            local.set $l1
            local.get $l1
            i32.const 10
            i32.add
            return
          end
          local.set $l1
          local.get $l1
          i32.const 11
          i32.add
          return
        end
        local.set $l1
        local.get $l1
        i32.const 12
        i32.add
        return
      end
      local.set $l1
      local.get $l1
      i32.const 13
      i32.add
      return
    end
    local.set $l1
    local.get $l1
    i32.const 14
    i32.add)
  (func $large (type $t6) (param $p0 i32) (result i32)
    block $B0
      block $B1
        local.get $p0
        br_table $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0 $B1 $B0
        i32.const -1
        return
      end
      i32.const 0
      return
    end
    i32.const 1
    return)
  (func $as-block-first (type $t1)
    block $B0
      i32.const 0
      br_table $B0 $B0 $B0
      call $f0
    end)
  (func $as-block-mid (type $t1)
    block $B0
      call $f0
      i32.const 0
      br_table $B0 $B0 $B0
      call $f0
    end)
  (func $as-block-last (type $t1)
    block $B0
      nop
      call $f0
      i32.const 0
      br_table $B0 $B0 $B0
    end)
  (func $as-block-value (type $t2) (result i32)
    block $B0 (result i32)
      nop
      call $f0
      i32.const 2
      i32.const 0
      br_table $B0 $B0 $B0
    end)
  (func $as-loop-first (type $t2) (result i32)
    loop $L0 (result i32)
      i32.const 3
      i32.const 0
      br_table 1 (;@0;) 1 (;@0;)
      i32.const 1
    end)
  (func $as-loop-mid (type $t2) (result i32)
    loop $L0 (result i32)
      call $f0
      i32.const 4
      i32.const -1
      br_table 1 (;@0;) 1 (;@0;) 1 (;@0;)
      i32.const 2
    end)
  (func $as-loop-last (type $t2) (result i32)
    loop $L0 (result i32)
      nop
      call $f0
      i32.const 5
      i32.const 1
      br_table 1 (;@0;) 1 (;@0;) 1 (;@0;)
    end)
  (func $as-br-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 9
      i32.const 0
      br_table $B0
      br $B0
    end)
  (func $as-br_if-cond (type $t1)
    block $B0
      i32.const 1
      br_table $B0 $B0 $B0
      br_if $B0
    end)
  (func $as-br_if-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 8
      i32.const 0
      br_table $B0
      i32.const 1
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_if-value-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 9
      i32.const 0
      br_table $B0 $B0
      br_if $B0
      drop
      i32.const 7
    end)
  (func $as-br_table-index (type $t1)
    block $B0
      i32.const 1
      br_table $B0
      br_table $B0 $B0 $B0
    end)
  (func $as-br_table-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 10
      i32.const 0
      br_table $B0
      i32.const 1
      br_table $B0 $B0 $B0
      i32.const 7
    end)
  (func $as-br_table-value-index (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 6
      i32.const 11
      i32.const 1
      br_table $B0
      br_table $B0 $B0
      i32.const 7
    end)
  (func $as-return-value (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 7
      i32.const 0
      br_table $B0
      return
    end)
  (func $as-if-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 0
      br_table $B0
      if $I1 (result i32)
        i32.const 0
      else
        i32.const 1
      end
    end)
  (func $as-if-then (type $t7) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        i32.const 3
        i32.const 0
        br_table $B0
      else
        local.get $p1
      end
    end)
  (func $as-if-else (type $t7) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      if $I1 (result i32)
        local.get $p1
      else
        i32.const 4
        i32.const 0
        br_table $B0 $I1
      end
    end)
  (func $as-select-first (type $t7) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      i32.const 5
      i32.const 0
      br_table $B0
      local.get $p0
      local.get $p1
      select
    end)
  (func $as-select-second (type $t7) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0 (result i32)
      local.get $p0
      i32.const 6
      i32.const 1
      br_table $B0
      local.get $p1
      select
    end)
  (func $as-select-cond (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 7
      i32.const 1
      br_table $B0
      select
    end)
  (func $f37 (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    i32.const -1)
  (func $as-call-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 12
      i32.const 1
      br_table $B0
      i32.const 2
      i32.const 3
      call $f37
    end)
  (func $as-call-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 13
      i32.const 1
      br_table $B0
      i32.const 3
      call $f37
    end)
  (func $as-call-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      i32.const 14
      i32.const 1
      br_table $B0
      call $f37
    end)
  (func $as-call_indirect-first (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 20
      i32.const 1
      br_table $B0
      i32.const 1
      i32.const 2
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 21
      i32.const 1
      br_table $B0
      i32.const 2
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 22
      i32.const 1
      br_table $B0
      i32.const 3
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-func (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 2
      i32.const 23
      i32.const 1
      br_table $B0
      call_indirect $T0 (type $t0)
    end)
  (func $as-local.set-value (type $t2) (result i32)
    (local $l0 f32)
    block $B0 (result i32)
      i32.const 17
      i32.const 1
      br_table $B0
      local.set $l0
      i32.const -1
    end)
  (func $as-local.tee-value (type $t2) (result i32)
    (local $l0 i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_table $B0
      local.set $l0
      i32.const -1
    end)
  (func $as-global.set-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 1
      br_table $B0
      global.set $g0
      i32.const -1
    end)
  (func $as-load-address (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.b33334p+0 (;=1.7;)
      i32.const 1
      br_table $B0
      f32.load
    end)
  (func $as-loadN-address (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 30
      i32.const 1
      br_table $B0
      i64.load8_s
    end)
  (func $as-store-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 30
      i32.const 1
      br_table $B0
      f64.const 0x1.cp+2 (;=7;)
      f64.store
      i32.const -1
    end)
  (func $as-store-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 31
      i32.const 1
      br_table $B0
      i64.store
      i32.const -1
    end)
  (func $as-storeN-address (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 32
      i32.const 0
      br_table $B0
      i32.const 7
      i32.store8
      i32.const -1
    end)
  (func $as-storeN-value (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 2
      i32.const 33
      i32.const 0
      br_table $B0
      i64.store16
      i32.const -1
    end)
  (func $as-unary-operand (type $t4) (result f32)
    block $B0 (result f32)
      f32.const 0x1.b33334p+1 (;=3.4;)
      i32.const 0
      br_table $B0
      f32.neg
    end)
  (func $as-binary-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 3
      i32.const 0
      br_table $B0 $B0
      i32.const 10
      i32.add
    end)
  (func $as-binary-right (type $t3) (result i64)
    block $B0 (result i64)
      i64.const 10
      i64.const 45
      i32.const 0
      br_table $B0
      i64.sub
    end)
  (func $as-test-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 44
      i32.const 0
      br_table $B0
      i32.eqz
    end)
  (func $as-compare-left (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 43
      i32.const 0
      br_table $B0 $B0
      f64.const 0x1.4p+3 (;=10;)
      f64.le
    end)
  (func $as-compare-right (type $t2) (result i32)
    block $B0 (result i32)
      f32.const 0x1.4p+3 (;=10;)
      i32.const 42
      i32.const 0
      br_table $B0
      f32.ne
    end)
  (func $as-convert-operand (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 41
      i32.const 0
      br_table $B0
      i32.wrap_i64
    end)
  (func $as-memory.grow-size (type $t2) (result i32)
    block $B0 (result i32)
      i32.const 40
      i32.const 0
      br_table $B0
      memory.grow
    end)
  (func $nested-block-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const -1
      drop
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        block $B2 (result i32)
          i32.const 4
          drop
          i32.const 8
          i32.const 16
          local.get $p0
          br_table $B2 $B1 $B0
          i32.add
        end
        i32.add
      end
      i32.add
    end)
  (func $nested-br-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        drop
        block $B2 (result i32)
          i32.const 4
          drop
          i32.const 8
          local.get $p0
          br_table $B0 $B1 $B2
          br $B2
        end
        drop
        i32.const 16
      end
      i32.add
    end)
  (func $nested-br_if-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        drop
        block $B2 (result i32)
          i32.const 4
          drop
          i32.const 8
          local.get $p0
          br_table $B2 $B1 $B0
          i32.const 1
          br_if $B2
          drop
          i32.const 32
        end
        drop
        i32.const 16
      end
      i32.add
    end)
  (func $nested-br_if-value-cond (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        drop
        i32.const 4
        i32.const 8
        local.get $p0
        br_table $B1 $B0 $B1
        br_if $B1
        drop
        i32.const 16
      end
      i32.add
    end)
  (func $nested-br_table-value (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        drop
        block $B2 (result i32)
          i32.const 4
          drop
          i32.const 8
          local.get $p0
          br_table $B2 $B1 $B0
          i32.const 1
          br_table $B2
          i32.const 32
        end
        drop
        i32.const 16
      end
      i32.add
    end)
  (func $nested-br_table-value-index (type $t6) (param $p0 i32) (result i32)
    block $B0 (result i32)
      i32.const 1
      block $B1 (result i32)
        i32.const 2
        drop
        i32.const 4
        i32.const 8
        local.get $p0
        br_table $B1 $B0 $B1
        br_table $B1
        i32.const 16
      end
      i32.add
    end)
  (func $nested-br_table-loop-block (type $t6) (param $p0 i32) (result i32)
    loop $L0 (result i32)
      block $B1
        local.get $p0
        br_table $L0 $B1 $B1
      end
      i32.const 0
    end
    local.set $p0
    loop $L2 (result i32)
      block $B3
        local.get $p0
        br_table $B3 $L2 $L2
      end
      i32.const 3
    end)
  (func $meet-externref (type $t8) (param $p0 i32) (param $p1 externref) (result externref)
    block $B0 (result externref)
      block $B1 (result externref)
        local.get $p1
        local.get $p0
        br_table $B0 $B1 $B0
      end
    end)
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
  (export "empty" (func $empty))
  (export "empty-value" (func $empty-value))
  (export "singleton" (func $singleton))
  (export "singleton-value" (func $singleton-value))
  (export "multiple" (func $multiple))
  (export "multiple-value" (func $multiple-value))
  (export "large" (func $large))
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
  (export "as-call_indirect-func" (func $as-call_indirect-func))
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
  (export "nested-block-value" (func $nested-block-value))
  (export "nested-br-value" (func $nested-br-value))
  (export "nested-br_if-value" (func $nested-br_if-value))
  (export "nested-br_if-value-cond" (func $nested-br_if-value-cond))
  (export "nested-br_table-value" (func $nested-br_table-value))
  (export "nested-br_table-value-index" (func $nested-br_table-value-index))
  (export "nested-br_table-loop-block" (func $nested-br_table-loop-block))
  (export "meet-externref" (func $meet-externref))
  (elem $e0 (i32.const 0) func $f37))
