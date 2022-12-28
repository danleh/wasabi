(module
  (type $t0 (func (param i64) (result i64)))
  (type $t1 (func (param i64) (result i64 i64)))
  (type $t2 (func (param i64 i64) (result i64 i64 i64)))
  (type $t3 (func (param i64 i64) (result i64)))
  (func $fac-rec (type $t0) (param $p0 i64) (result i64)
    local.get $p0
    i64.const 0
    i64.eq
    if $I0 (result i64)
      i64.const 1
    else
      local.get $p0
      local.get $p0
      i64.const 1
      i64.sub
      call $fac-rec
      i64.mul
    end)
  (func $fac-rec-named (type $t0) (param $p0 i64) (result i64)
    local.get $p0
    i64.const 0
    i64.eq
    if $I0 (result i64)
      i64.const 1
    else
      local.get $p0
      local.get $p0
      i64.const 1
      i64.sub
      call $fac-rec-named
      i64.mul
    end)
  (func $fac-iter (type $t0) (param $p0 i64) (result i64)
    (local $l1 i64) (local $l2 i64)
    local.get $p0
    local.set $l1
    i64.const 1
    local.set $l2
    block $B0
      loop $L1
        local.get $l1
        i64.const 0
        i64.eq
        if $I2
          br $B0
        else
          local.get $l1
          local.get $l2
          i64.mul
          local.set $l2
          local.get $l1
          i64.const 1
          i64.sub
          local.set $l1
        end
        br $L1
      end
    end
    local.get $l2)
  (func $fac-iter-named (type $t0) (param $p0 i64) (result i64)
    (local $l1 i64) (local $l2 i64)
    local.get $p0
    local.set $l1
    i64.const 1
    local.set $l2
    block $B0
      loop $L1
        local.get $l1
        i64.const 0
        i64.eq
        if $I2
          br $B0
        else
          local.get $l1
          local.get $l2
          i64.mul
          local.set $l2
          local.get $l1
          i64.const 1
          i64.sub
          local.set $l1
        end
        br $L1
      end
    end
    local.get $l2)
  (func $fac-opt (type $t0) (param $p0 i64) (result i64)
    (local $l1 i64)
    i64.const 1
    local.set $l1
    block $B0
      local.get $p0
      i64.const 2
      i64.lt_s
      br_if $B0
      loop $L1
        local.get $l1
        local.get $p0
        i64.mul
        local.set $l1
        local.get $p0
        i64.const -1
        i64.add
        local.set $p0
        local.get $p0
        i64.const 1
        i64.gt_s
        br_if $L1
      end
    end
    local.get $l1)
  (func $f5 (type $t1) (param $p0 i64) (result i64 i64)
    local.get $p0
    local.get $p0)
  (func $f6 (type $t2) (param $p0 i64) (param $p1 i64) (result i64 i64 i64)
    local.get $p0
    local.get $p1
    local.get $p0)
  (func $fac-ssa (type $t0) (param $p0 i64) (result i64)
    i64.const 1
    local.get $p0
    loop $L0 (param i64 i64) (result i64)
      call $f6
      call $f6
      i64.mul
      call $f6
      i64.const 1
      i64.sub
      call $f5
      i64.const 0
      i64.gt_u
      br_if $L0
      drop
      return
    end)
  (export "fac-rec" (func $fac-rec))
  (export "fac-rec-named" (func $fac-rec-named))
  (export "fac-iter" (func $fac-iter))
  (export "fac-iter-named" (func $fac-iter-named))
  (export "fac-opt" (func $fac-opt))
  (export "fac-ssa" (func $fac-ssa)))
