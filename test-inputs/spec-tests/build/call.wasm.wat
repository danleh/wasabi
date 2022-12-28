(module
  (type $t0 (func (param i32 i32) (result i32)))
  (type $t1 (func (result i32)))
  (type $t2 (func (result i64)))
  (type $t3 (func (result f32)))
  (type $t4 (func (result f64)))
  (type $t5 (func (result i32 i64)))
  (type $t6 (func (param i32) (result i32)))
  (type $t7 (func (param i64) (result i64)))
  (type $t8 (func (param f32) (result f32)))
  (type $t9 (func (param f64) (result f64)))
  (type $t10 (func (param i32 f64) (result i32 f64)))
  (type $t11 (func (param i32 i32) (result i32 i32)))
  (type $t12 (func (param f32 f64) (result f64 f32)))
  (type $t13 (func (param f64 i32) (result i32 f64)))
  (type $t14 (func (param f32 i32) (result i32)))
  (type $t15 (func (param i32 i64) (result i64)))
  (type $t16 (func (param f64 f32) (result f32)))
  (type $t17 (func (param i64 f64) (result f64)))
  (type $t18 (func (result i32 f64)))
  (type $t19 (func (result i32 i32)))
  (type $t20 (func (result f64 f32)))
  (type $t21 (func (param i64 i64) (result i64)))
  (type $t22 (func (param i64) (result i32)))
  (type $t23 (func))
  (type $t24 (func (param f32 i32 i32 f64 f32 f32 f32 f64 f32 i32 i32 f32 f64 i64 i64 i32 i64 i64 f32 i64 i64 i64 i32 f32 f32 f32 f64 f32 i32 i64 f32 f64 f64 f32 i32 f32 f32 f64 i64 f64 i32 i64 f32 f64 i32 i32 i32 i64 f64 i32 i64 i64 f64 f64 f64 f64 f64 f64 i32 f32 f64 f64 i32 i64 f32 f32 f32 i32 f64 f64 f64 f64 f64 f32 i64 i64 i32 i32 i32 f32 f64 i32 i64 f32 f32 f32 i32 i32 f32 f64 i64 f32 f64 f32 f32 f32 i32 f32 i64 i32) (result i32)))
  (func $f0 (type $t1) (result i32)
    i32.const 306)
  (func $f1 (type $t2) (result i64)
    i64.const 356)
  (func $f2 (type $t3) (result f32)
    f32.const 0x1.e64p+11 (;=3890;))
  (func $f3 (type $t4) (result f64)
    f64.const 0x1.ec8p+11 (;=3940;))
  (func $f4 (type $t5) (result i32 i64)
    i32.const 306
    i64.const 356)
  (func $f5 (type $t6) (param $p0 i32) (result i32)
    local.get $p0)
  (func $f6 (type $t7) (param $p0 i64) (result i64)
    local.get $p0)
  (func $f7 (type $t8) (param $p0 f32) (result f32)
    local.get $p0)
  (func $f8 (type $t9) (param $p0 f64) (result f64)
    local.get $p0)
  (func $f9 (type $t10) (param $p0 i32) (param $p1 f64) (result i32 f64)
    local.get $p0
    local.get $p1)
  (func $f10 (type $t11) (param $p0 i32) (param $p1 i32) (result i32 i32)
    local.get $p1
    local.get $p0)
  (func $f11 (type $t12) (param $p0 f32) (param $p1 f64) (result f64 f32)
    local.get $p1
    local.get $p0)
  (func $f12 (type $t13) (param $p0 f64) (param $p1 i32) (result i32 f64)
    local.get $p1
    local.get $p0)
  (func $f13 (type $t14) (param $p0 f32) (param $p1 i32) (result i32)
    local.get $p1)
  (func $f14 (type $t15) (param $p0 i32) (param $p1 i64) (result i64)
    local.get $p1)
  (func $f15 (type $t16) (param $p0 f64) (param $p1 f32) (result f32)
    local.get $p1)
  (func $f16 (type $t17) (param $p0 i64) (param $p1 f64) (result f64)
    local.get $p1)
  (func $type-i32 (type $t1) (result i32)
    call $f0)
  (func $type-i64 (type $t2) (result i64)
    call $f1)
  (func $type-f32 (type $t3) (result f32)
    call $f2)
  (func $type-f64 (type $t4) (result f64)
    call $f3)
  (func $type-i32-i64 (type $t5) (result i32 i64)
    call $f4)
  (func $type-first-i32 (type $t1) (result i32)
    i32.const 32
    call $f5)
  (func $type-first-i64 (type $t2) (result i64)
    i64.const 64
    call $f6)
  (func $type-first-f32 (type $t3) (result f32)
    f32.const 0x1.51eb86p+0 (;=1.32;)
    call $f7)
  (func $type-first-f64 (type $t4) (result f64)
    f64.const 0x1.a3d70a3d70a3dp+0 (;=1.64;)
    call $f8)
  (func $type-second-i32 (type $t1) (result i32)
    f32.const 0x1.00ccccp+5 (;=32.1;)
    i32.const 32
    call $f13)
  (func $type-second-i64 (type $t2) (result i64)
    i32.const 32
    i64.const 64
    call $f14)
  (func $type-second-f32 (type $t3) (result f32)
    f64.const 0x1p+6 (;=64;)
    f32.const 0x1p+5 (;=32;)
    call $f15)
  (func $type-second-f64 (type $t4) (result f64)
    i64.const 64
    f64.const 0x1.0066666666666p+6 (;=64.1;)
    call $f16)
  (func $type-all-i32-f64 (type $t18) (result i32 f64)
    i32.const 32
    f64.const 0x1.a3d70a3d70a3dp+0 (;=1.64;)
    call $f9)
  (func $type-all-i32-i32 (type $t19) (result i32 i32)
    i32.const 1
    i32.const 2
    call $f10)
  (func $type-all-f32-f64 (type $t20) (result f64 f32)
    f32.const 0x1p+0 (;=1;)
    f64.const 0x1p+1 (;=2;)
    call $f11)
  (func $type-all-f64-i32 (type $t18) (result i32 f64)
    f64.const 0x1p+0 (;=1;)
    i32.const 2
    call $f12)
  (func $as-binary-all-operands (type $t1) (result i32)
    i32.const 3
    i32.const 4
    call $f10
    i32.add)
  (func $as-mixed-operands (type $t1) (result i32)
    i32.const 3
    i32.const 4
    call $f10
    i32.const 5
    i32.add
    i32.mul)
  (func $as-call-all-operands (type $t19) (result i32 i32)
    i32.const 3
    i32.const 4
    call $f10
    call $f10)
  (func $fac (type $t7) (param $p0 i64) (result i64)
    local.get $p0
    i64.eqz
    if $I0 (result i64)
      i64.const 1
    else
      local.get $p0
      local.get $p0
      i64.const 1
      i64.sub
      call $fac
      i64.mul
    end)
  (func $fac-acc (type $t21) (param $p0 i64) (param $p1 i64) (result i64)
    local.get $p0
    i64.eqz
    if $I0 (result i64)
      local.get $p1
    else
      local.get $p0
      i64.const 1
      i64.sub
      local.get $p0
      local.get $p1
      i64.mul
      call $fac-acc
    end)
  (func $fib (type $t7) (param $p0 i64) (result i64)
    local.get $p0
    i64.const 1
    i64.le_u
    if $I0 (result i64)
      i64.const 1
    else
      local.get $p0
      i64.const 2
      i64.sub
      call $fib
      local.get $p0
      i64.const 1
      i64.sub
      call $fib
      i64.add
    end)
  (func $even (type $t22) (param $p0 i64) (result i32)
    local.get $p0
    i64.eqz
    if $I0 (result i32)
      i32.const 44
    else
      local.get $p0
      i64.const 1
      i64.sub
      call $odd
    end)
  (func $odd (type $t22) (param $p0 i64) (result i32)
    local.get $p0
    i64.eqz
    if $I0 (result i32)
      i32.const 99
    else
      local.get $p0
      i64.const 1
      i64.sub
      call $even
    end)
  (func $runaway (type $t23)
    call $runaway)
  (func $mutual-runaway (type $t23)
    call $f44)
  (func $f44 (type $t23)
    call $mutual-runaway)
  (func $as-select-first (type $t1) (result i32)
    call $f0
    i32.const 2
    i32.const 3
    select)
  (func $as-select-mid (type $t1) (result i32)
    i32.const 2
    call $f0
    i32.const 3
    select)
  (func $as-select-last (type $t1) (result i32)
    i32.const 2
    i32.const 3
    call $f0
    select)
  (func $as-if-condition (type $t1) (result i32)
    call $f0
    if $I0 (result i32)
      i32.const 1
    else
      i32.const 2
    end)
  (func $as-br_if-first (type $t1) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 2
      br_if $B0
    end)
  (func $as-br_if-last (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 2
      call $f0
      br_if $B0
    end)
  (func $as-br_table-first (type $t1) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 2
      br_table $B0 $B0
    end)
  (func $as-br_table-last (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 2
      call $f0
      br_table $B0 $B0
    end)
  (func $f53 (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    local.get $p0)
  (func $as-call_indirect-first (type $t1) (result i32)
    block $B0 (result i32)
      call $f0
      i32.const 2
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-mid (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 2
      call $f0
      i32.const 0
      call_indirect $T0 (type $t0)
    end)
  (func $as-call_indirect-last (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 1
      i32.const 2
      call $f0
      call_indirect $T0 (type $t0)
    end)
  (func $as-store-first (type $t23)
    call $f0
    i32.const 1
    i32.store)
  (func $as-store-last (type $t23)
    i32.const 10
    call $f0
    i32.store)
  (func $as-memory.grow-value (type $t1) (result i32)
    call $f0
    memory.grow)
  (func $as-return-value (type $t1) (result i32)
    call $f0
    return)
  (func $as-drop-operand (type $t23)
    call $f0
    drop)
  (func $as-br-value (type $t1) (result i32)
    block $B0 (result i32)
      call $f0
      br $B0
    end)
  (func $as-local.set-value (type $t1) (result i32)
    (local $l0 i32)
    call $f0
    local.set $l0
    local.get $l0)
  (func $as-local.tee-value (type $t1) (result i32)
    (local $l0 i32)
    call $f0
    local.tee $l0)
  (func $as-global.set-value (type $t1) (result i32)
    call $f0
    global.set $g0
    global.get $g0)
  (func $as-load-operand (type $t1) (result i32)
    call $f0
    i32.load)
  (func $f67 (type $t6) (param $p0 i32) (result i32)
    local.get $p0)
  (func $f68 (type $t8) (param $p0 f32) (result f32)
    local.get $p0)
  (func $as-unary-operand (type $t3) (result f32)
    block $B0 (result f32)
      f32.const 0x0p+0 (;=0;)
      call $f68
      f32.sqrt
    end)
  (func $as-binary-left (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 1
      call $f67
      i32.const 10
      i32.add
    end)
  (func $as-binary-right (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 10
      i32.const 1
      call $f67
      i32.sub
    end)
  (func $as-test-operand (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 1
      call $f67
      i32.eqz
    end)
  (func $as-compare-left (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 1
      call $f67
      i32.const 10
      i32.le_u
    end)
  (func $as-compare-right (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 10
      i32.const 1
      call $f67
      i32.ne
    end)
  (func $as-convert-operand (type $t2) (result i64)
    block $B0 (result i64)
      i32.const 1
      call $f67
      i64.extend_i32_s
    end)
  (func $f76 (type $t24) (param $p0 f32) (param $p1 i32) (param $p2 i32) (param $p3 f64) (param $p4 f32) (param $p5 f32) (param $p6 f32) (param $p7 f64) (param $p8 f32) (param $p9 i32) (param $p10 i32) (param $p11 f32) (param $p12 f64) (param $p13 i64) (param $p14 i64) (param $p15 i32) (param $p16 i64) (param $p17 i64) (param $p18 f32) (param $p19 i64) (param $p20 i64) (param $p21 i64) (param $p22 i32) (param $p23 f32) (param $p24 f32) (param $p25 f32) (param $p26 f64) (param $p27 f32) (param $p28 i32) (param $p29 i64) (param $p30 f32) (param $p31 f64) (param $p32 f64) (param $p33 f32) (param $p34 i32) (param $p35 f32) (param $p36 f32) (param $p37 f64) (param $p38 i64) (param $p39 f64) (param $p40 i32) (param $p41 i64) (param $p42 f32) (param $p43 f64) (param $p44 i32) (param $p45 i32) (param $p46 i32) (param $p47 i64) (param $p48 f64) (param $p49 i32) (param $p50 i64) (param $p51 i64) (param $p52 f64) (param $p53 f64) (param $p54 f64) (param $p55 f64) (param $p56 f64) (param $p57 f64) (param $p58 i32) (param $p59 f32) (param $p60 f64) (param $p61 f64) (param $p62 i32) (param $p63 i64) (param $p64 f32) (param $p65 f32) (param $p66 f32) (param $p67 i32) (param $p68 f64) (param $p69 f64) (param $p70 f64) (param $p71 f64) (param $p72 f64) (param $p73 f32) (param $p74 i64) (param $p75 i64) (param $p76 i32) (param $p77 i32) (param $p78 i32) (param $p79 f32) (param $p80 f64) (param $p81 i32) (param $p82 i64) (param $p83 f32) (param $p84 f32) (param $p85 f32) (param $p86 i32) (param $p87 i32) (param $p88 f32) (param $p89 f64) (param $p90 i64) (param $p91 f32) (param $p92 f64) (param $p93 f32) (param $p94 f32) (param $p95 f32) (param $p96 i32) (param $p97 f32) (param $p98 i64) (param $p99 i32) (result i32)
    local.get $p99)
  (func $return-from-long-argument-list (type $t6) (param $p0 i32) (result i32)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    i64.const 0
    i32.const 0
    i64.const 0
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    i64.const 0
    i64.const 0
    i64.const 0
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 0
    i32.const 0
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i64.const 0
    i64.const 0
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i64.const 0
    i64.const 0
    i32.const 0
    i32.const 0
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i32.const 0
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    i64.const 0
    f32.const 0x0p+0 (;=0;)
    f64.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    f32.const 0x0p+0 (;=0;)
    i32.const 0
    f32.const 0x0p+0 (;=0;)
    i64.const 0
    local.get $p0
    call $f76)
  (table $T0 1 1 funcref)
  (memory $M0 1)
  (global $g0 (mut i32) (i32.const 10))
  (export "type-i32" (func $type-i32))
  (export "type-i64" (func $type-i64))
  (export "type-f32" (func $type-f32))
  (export "type-f64" (func $type-f64))
  (export "type-i32-i64" (func $type-i32-i64))
  (export "type-first-i32" (func $type-first-i32))
  (export "type-first-i64" (func $type-first-i64))
  (export "type-first-f32" (func $type-first-f32))
  (export "type-first-f64" (func $type-first-f64))
  (export "type-second-i32" (func $type-second-i32))
  (export "type-second-i64" (func $type-second-i64))
  (export "type-second-f32" (func $type-second-f32))
  (export "type-second-f64" (func $type-second-f64))
  (export "type-all-i32-f64" (func $type-all-i32-f64))
  (export "type-all-i32-i32" (func $type-all-i32-i32))
  (export "type-all-f32-f64" (func $type-all-f32-f64))
  (export "type-all-f64-i32" (func $type-all-f64-i32))
  (export "as-binary-all-operands" (func $as-binary-all-operands))
  (export "as-mixed-operands" (func $as-mixed-operands))
  (export "as-call-all-operands" (func $as-call-all-operands))
  (export "fac" (func $fac))
  (export "fac-acc" (func $fac-acc))
  (export "fib" (func $fib))
  (export "even" (func $even))
  (export "odd" (func $odd))
  (export "runaway" (func $runaway))
  (export "mutual-runaway" (func $mutual-runaway))
  (export "as-select-first" (func $as-select-first))
  (export "as-select-mid" (func $as-select-mid))
  (export "as-select-last" (func $as-select-last))
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
  (export "as-return-value" (func $as-return-value))
  (export "as-drop-operand" (func $as-drop-operand))
  (export "as-br-value" (func $as-br-value))
  (export "as-local.set-value" (func $as-local.set-value))
  (export "as-local.tee-value" (func $as-local.tee-value))
  (export "as-global.set-value" (func $as-global.set-value))
  (export "as-load-operand" (func $as-load-operand))
  (export "as-unary-operand" (func $as-unary-operand))
  (export "as-binary-left" (func $as-binary-left))
  (export "as-binary-right" (func $as-binary-right))
  (export "as-test-operand" (func $as-test-operand))
  (export "as-compare-left" (func $as-compare-left))
  (export "as-compare-right" (func $as-compare-right))
  (export "as-convert-operand" (func $as-convert-operand))
  (export "return-from-long-argument-list" (func $return-from-long-argument-list))
  (elem $e0 (i32.const 0) func $f53))
