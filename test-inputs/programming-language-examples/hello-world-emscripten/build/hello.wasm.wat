(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i32 i32 i32) (result i32)))
  (type $t2 (func (param i32 i32 i32 i32) (result i32)))
  (type $t3 (func (param i32 i32 i32)))
  (type $t4 (func))
  (type $t5 (func (result i32)))
  (type $t6 (func (param i32 i64 i32) (result i64)))
  (type $t7 (func (param i32 i32) (result i32)))
  (import "a" "a" (func $a.a (type $t2)))
  (import "a" "b" (func $a.b (type $t3)))
  (func $d (type $t4)
    nop)
  (func $f3 (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.eqz
    if $I0
      i32.const 0
      return
    end
    i32.const 2216
    local.get $p0
    i32.store
    i32.const -1)
  (func $f4 (type $t0) (param $p0 i32) (result i32)
    (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32)
    i32.const 1024
    local.set $l2
    block $B0
      local.get $p0
      i32.const 1056
      i32.load
      local.tee $l3
      if $I1 (result i32)
        local.get $l3
      else
        call $f5
        br_if $B0
        i32.const 1056
        i32.load
      end
      i32.const 1060
      i32.load
      local.tee $l4
      i32.sub
      i32.gt_u
      if $I2
        i32.const 1040
        i32.const 1024
        local.get $p0
        i32.const 1076
        i32.load
        call_indirect $f (type $t1)
        return
      end
      block $B3
        i32.const 1120
        i32.load
        i32.const 0
        i32.lt_s
        if $I4
          i32.const 0
          local.set $l3
          br $B3
        end
        local.get $p0
        local.set $l1
        loop $L5
          local.get $l1
          local.tee $l3
          i32.eqz
          if $I6
            i32.const 0
            local.set $l3
            br $B3
          end
          local.get $l3
          i32.const 1
          i32.sub
          local.tee $l1
          i32.const 1024
          i32.add
          i32.load8_u
          i32.const 10
          i32.ne
          br_if $L5
        end
        i32.const 1040
        i32.const 1024
        local.get $l3
        i32.const 1076
        i32.load
        call_indirect $f (type $t1)
        local.tee $l1
        local.get $l3
        i32.lt_u
        br_if $B0
        local.get $l3
        i32.const 1024
        i32.add
        local.set $l2
        local.get $p0
        local.get $l3
        i32.sub
        local.set $p0
        i32.const 1060
        i32.load
        local.set $l4
      end
      local.get $l4
      local.set $l1
      block $B7
        local.get $p0
        i32.const 512
        i32.ge_u
        if $I8
          local.get $l1
          local.get $l2
          local.get $p0
          call $a.b
          br $B7
        end
        local.get $p0
        local.get $l1
        i32.add
        local.set $l4
        block $B9
          local.get $l1
          local.get $l2
          i32.xor
          i32.const 3
          i32.and
          i32.eqz
          if $I10
            block $B11
              local.get $l1
              i32.const 3
              i32.and
              i32.eqz
              local.get $p0
              i32.eqz
              i32.or
              br_if $B11
              loop $L12
                local.get $l1
                local.get $l2
                i32.load8_u
                i32.store8
                local.get $l2
                i32.const 1
                i32.add
                local.set $l2
                local.get $l1
                i32.const 1
                i32.add
                local.tee $l1
                i32.const 3
                i32.and
                i32.eqz
                br_if $B11
                local.get $l1
                local.get $l4
                i32.lt_u
                br_if $L12
              end
            end
            block $B13
              local.get $l4
              i32.const -4
              i32.and
              local.tee $l5
              i32.const 64
              i32.lt_u
              br_if $B13
              local.get $l1
              local.get $l5
              i32.const -64
              i32.add
              local.tee $l6
              i32.gt_u
              br_if $B13
              loop $L14
                local.get $l1
                local.get $l2
                i32.load
                i32.store
                local.get $l1
                local.get $l2
                i32.load offset=4
                i32.store offset=4
                local.get $l1
                local.get $l2
                i32.load offset=8
                i32.store offset=8
                local.get $l1
                local.get $l2
                i32.load offset=12
                i32.store offset=12
                local.get $l1
                local.get $l2
                i32.load offset=16
                i32.store offset=16
                local.get $l1
                local.get $l2
                i32.load offset=20
                i32.store offset=20
                local.get $l1
                local.get $l2
                i32.load offset=24
                i32.store offset=24
                local.get $l1
                local.get $l2
                i32.load offset=28
                i32.store offset=28
                local.get $l1
                local.get $l2
                i32.load offset=32
                i32.store offset=32
                local.get $l1
                local.get $l2
                i32.load offset=36
                i32.store offset=36
                local.get $l1
                local.get $l2
                i32.load offset=40
                i32.store offset=40
                local.get $l1
                local.get $l2
                i32.load offset=44
                i32.store offset=44
                local.get $l1
                local.get $l2
                i32.load offset=48
                i32.store offset=48
                local.get $l1
                local.get $l2
                i32.load offset=52
                i32.store offset=52
                local.get $l1
                local.get $l2
                i32.load offset=56
                i32.store offset=56
                local.get $l1
                local.get $l2
                i32.load offset=60
                i32.store offset=60
                local.get $l2
                i32.const -64
                i32.sub
                local.set $l2
                local.get $l1
                i32.const -64
                i32.sub
                local.tee $l1
                local.get $l6
                i32.le_u
                br_if $L14
              end
            end
            local.get $l1
            local.get $l5
            i32.ge_u
            br_if $B9
            loop $L15
              local.get $l1
              local.get $l2
              i32.load
              i32.store
              local.get $l2
              i32.const 4
              i32.add
              local.set $l2
              local.get $l1
              i32.const 4
              i32.add
              local.tee $l1
              local.get $l5
              i32.lt_u
              br_if $L15
            end
            br $B9
          end
          local.get $l4
          i32.const 4
          i32.lt_u
          br_if $B9
          local.get $l1
          local.get $l4
          i32.const 4
          i32.sub
          local.tee $l5
          i32.gt_u
          br_if $B9
          loop $L16
            local.get $l1
            local.get $l2
            i32.load8_u
            i32.store8
            local.get $l1
            local.get $l2
            i32.load8_u offset=1
            i32.store8 offset=1
            local.get $l1
            local.get $l2
            i32.load8_u offset=2
            i32.store8 offset=2
            local.get $l1
            local.get $l2
            i32.load8_u offset=3
            i32.store8 offset=3
            local.get $l2
            i32.const 4
            i32.add
            local.set $l2
            local.get $l1
            i32.const 4
            i32.add
            local.tee $l1
            local.get $l5
            i32.le_u
            br_if $L16
          end
        end
        local.get $l1
        local.get $l4
        i32.lt_u
        if $I17
          loop $L18
            local.get $l1
            local.get $l2
            i32.load8_u
            i32.store8
            local.get $l2
            i32.const 1
            i32.add
            local.set $l2
            local.get $l1
            i32.const 1
            i32.add
            local.tee $l1
            local.get $l4
            i32.ne
            br_if $L18
          end
        end
      end
      i32.const 1060
      i32.const 1060
      i32.load
      local.get $p0
      i32.add
      i32.store
      local.get $p0
      local.get $l3
      i32.add
      local.set $l1
    end
    local.get $l1)
  (func $f5 (type $t5) (result i32)
    (local $l0 i32)
    i32.const 1112
    i32.const 1112
    i32.load
    local.tee $l0
    i32.const 1
    i32.sub
    local.get $l0
    i32.or
    i32.store
    i32.const 1040
    i32.load
    local.tee $l0
    i32.const 8
    i32.and
    if $I0
      i32.const 1040
      local.get $l0
      i32.const 32
      i32.or
      i32.store
      i32.const -1
      return
    end
    i32.const 1044
    i64.const 0
    i64.store align=4
    i32.const 1068
    i32.const 1084
    i32.load
    local.tee $l0
    i32.store
    i32.const 1060
    local.get $l0
    i32.store
    i32.const 1056
    local.get $l0
    i32.const 1088
    i32.load
    i32.add
    i32.store
    i32.const 0)
  (func $f6 (type $t6) (param $p0 i32) (param $p1 i64) (param $p2 i32) (result i64)
    i64.const 0)
  (func $f7 (type $t0) (param $p0 i32) (result i32)
    i32.const 0)
  (func $f8 (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i32) (local $l8 i32) (local $l9 i32)
    global.get $g0
    i32.const 32
    i32.sub
    local.tee $l3
    global.set $g0
    local.get $l3
    local.get $p0
    i32.load offset=28
    local.tee $l4
    i32.store offset=16
    local.get $p0
    i32.load offset=20
    local.set $l5
    local.get $l3
    local.get $p2
    i32.store offset=28
    local.get $l3
    local.get $p1
    i32.store offset=24
    local.get $l3
    local.get $l5
    local.get $l4
    i32.sub
    local.tee $p1
    i32.store offset=20
    local.get $p1
    local.get $p2
    i32.add
    local.set $l5
    local.get $l3
    i32.const 16
    i32.add
    local.set $p1
    i32.const 2
    local.set $l7
    block $B0 (result i32)
      block $B1
        block $B2
          block $B3
            local.get $p0
            i32.load offset=60
            local.get $p1
            i32.const 2
            local.get $l3
            i32.const 12
            i32.add
            call $a.a
            call $f3
            if $I4
              local.get $p1
              local.set $l4
              br $B3
            end
            loop $L5
              local.get $l5
              local.get $l3
              i32.load offset=12
              local.tee $l6
              i32.eq
              br_if $B2
              local.get $l6
              i32.const 0
              i32.lt_s
              if $I6
                local.get $p1
                local.set $l4
                br $B1
              end
              local.get $p1
              local.get $l6
              local.get $p1
              i32.load offset=4
              local.tee $l8
              i32.gt_u
              local.tee $l9
              i32.const 3
              i32.shl
              i32.add
              local.tee $l4
              local.get $l6
              local.get $l8
              i32.const 0
              local.get $l9
              select
              i32.sub
              local.tee $l8
              local.get $l4
              i32.load
              i32.add
              i32.store
              local.get $p1
              i32.const 12
              i32.const 4
              local.get $l9
              select
              i32.add
              local.tee $p1
              local.get $p1
              i32.load
              local.get $l8
              i32.sub
              i32.store
              local.get $l5
              local.get $l6
              i32.sub
              local.set $l5
              local.get $p0
              i32.load offset=60
              local.get $l4
              local.tee $p1
              local.get $l7
              local.get $l9
              i32.sub
              local.tee $l7
              local.get $l3
              i32.const 12
              i32.add
              call $a.a
              call $f3
              i32.eqz
              br_if $L5
            end
          end
          local.get $l5
          i32.const -1
          i32.ne
          br_if $B1
        end
        local.get $p0
        local.get $p0
        i32.load offset=44
        local.tee $p1
        i32.store offset=28
        local.get $p0
        local.get $p1
        i32.store offset=20
        local.get $p0
        local.get $p1
        local.get $p0
        i32.load offset=48
        i32.add
        i32.store offset=16
        local.get $p2
        br $B0
      end
      local.get $p0
      i32.const 0
      i32.store offset=28
      local.get $p0
      i64.const 0
      i64.store offset=16
      local.get $p0
      local.get $p0
      i32.load
      i32.const 32
      i32.or
      i32.store
      i32.const 0
      local.get $l7
      i32.const 2
      i32.eq
      br_if $B0
      drop
      local.get $p2
      local.get $l4
      i32.load offset=4
      i32.sub
    end
    local.set $p1
    local.get $l3
    i32.const 32
    i32.add
    global.set $g0
    local.get $p1)
  (func $g (type $t0) (param $p0 i32) (result i32)
    global.get $g0
    local.get $p0
    i32.sub
    i32.const -16
    i32.and
    local.tee $p0
    global.set $g0
    local.get $p0)
  (func $e (type $t7) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    i32.const 1116
    i32.load
    drop
    block $B0
      i32.const 1024
      local.tee $p1
      i32.const 3
      i32.and
      if $I1
        loop $L2
          local.get $p1
          i32.load8_u
          i32.eqz
          br_if $B0
          local.get $p1
          i32.const 1
          i32.add
          local.tee $p1
          i32.const 3
          i32.and
          br_if $L2
        end
      end
      loop $L3
        local.get $p1
        local.tee $p0
        i32.const 4
        i32.add
        local.set $p1
        local.get $p0
        i32.load
        local.tee $l2
        i32.const -1
        i32.xor
        local.get $l2
        i32.const 16843009
        i32.sub
        i32.and
        i32.const -2139062144
        i32.and
        i32.eqz
        br_if $L3
      end
      loop $L4
        local.get $p0
        local.tee $p1
        i32.const 1
        i32.add
        local.set $p0
        local.get $p1
        i32.load8_u
        br_if $L4
      end
    end
    block $B5
      i32.const -1
      i32.const 0
      block $B6 (result i32)
        local.get $p1
        i32.const 1024
        i32.sub
        local.tee $p0
        block $B7 (result i32)
          i32.const 1116
          i32.load
          i32.const 0
          i32.lt_s
          if $I8
            local.get $p0
            call $f4
            br $B7
          end
          local.get $p0
          call $f4
        end
        local.tee $p1
        local.get $p0
        i32.eq
        br_if $B6
        drop
        local.get $p1
      end
      local.get $p0
      i32.ne
      select
      i32.const 0
      i32.lt_s
      br_if $B5
      block $B9
        i32.const 1120
        i32.load
        i32.const 10
        i32.eq
        br_if $B9
        i32.const 1060
        i32.load
        local.tee $p0
        i32.const 1056
        i32.load
        i32.eq
        br_if $B9
        i32.const 1060
        local.get $p0
        i32.const 1
        i32.add
        i32.store
        local.get $p0
        i32.const 10
        i32.store8
        br $B5
      end
      global.get $g0
      i32.const 16
      i32.sub
      local.tee $p0
      global.set $g0
      local.get $p0
      i32.const 10
      i32.store8 offset=15
      block $B10
        block $B11
          i32.const 1056
          i32.load
          local.tee $p1
          if $I12 (result i32)
            local.get $p1
          else
            call $f5
            br_if $B10
            i32.const 1056
            i32.load
          end
          i32.const 1060
          i32.load
          local.tee $p1
          i32.eq
          br_if $B11
          i32.const 1120
          i32.load
          i32.const 10
          i32.eq
          br_if $B11
          i32.const 1060
          local.get $p1
          i32.const 1
          i32.add
          i32.store
          local.get $p1
          i32.const 10
          i32.store8
          br $B10
        end
        i32.const 1040
        local.get $p0
        i32.const 15
        i32.add
        i32.const 1
        i32.const 1076
        i32.load
        call_indirect $f (type $t1)
        i32.const 1
        i32.ne
        br_if $B10
        local.get $p0
        i32.load8_u offset=15
        drop
      end
      local.get $p0
      i32.const 16
      i32.add
      global.set $g0
    end
    i32.const 0)
  (table $f 4 4 funcref)
  (memory $c 256 256)
  (global $g0 (mut i32) (i32.const 67760))
  (export "c" (memory 0))
  (export "d" (func $d))
  (export "e" (func $e))
  (export "f" (table 0))
  (export "g" (func $g))
  (elem $e0 (i32.const 1) func $f7 $f8 $f6)
  (data $d0 (i32.const 1024) "Hello, world!")
  (data $d1 (i32.const 1040) "\05")
  (data $d2 (i32.const 1052) "\01")
  (data $d3 (i32.const 1076) "\02\00\00\00\03\00\00\00\a8\04\00\00\00\04")
  (data $d4 (i32.const 1100) "\01")
  (data $d5 (i32.const 1116) "\ff\ff\ff\ff\0a"))
