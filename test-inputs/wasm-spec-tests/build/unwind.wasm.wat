(module
  (type $t0 (func))
  (type $t1 (func (result i32)))
  (func $func-unwind-by-unreachable (type $t0)
    i32.const 3
    i64.const 1
    unreachable)
  (func $func-unwind-by-br (type $t0)
    i32.const 3
    i64.const 1
    br 0 (;@0;))
  (func $func-unwind-by-br-value (type $t1) (result i32)
    i32.const 3
    i64.const 1
    i32.const 9
    br 0 (;@0;))
  (func $func-unwind-by-br_if (type $t0)
    i32.const 3
    i64.const 1
    i32.const 1
    br_if 0 (;@0;)
    drop
    drop)
  (func $func-unwind-by-br_if-value (type $t1) (result i32)
    i32.const 3
    i64.const 1
    i32.const 9
    i32.const 1
    br_if 0 (;@0;)
    drop
    drop)
  (func $func-unwind-by-br_table (type $t0)
    i32.const 3
    i64.const 1
    i32.const 0
    br_table 0 (;@0;))
  (func $func-unwind-by-br_table-value (type $t1) (result i32)
    i32.const 3
    i64.const 1
    i32.const 9
    i32.const 0
    br_table 0 (;@0;))
  (func $func-unwind-by-return (type $t1) (result i32)
    i32.const 3
    i64.const 1
    i32.const 9
    return)
  (func $block-unwind-by-unreachable (type $t0)
    block $B0
      i32.const 3
      i64.const 1
      unreachable
    end)
  (func $block-unwind-by-br (type $t1) (result i32)
    block $B0
      i32.const 3
      i64.const 1
      br $B0
    end
    i32.const 9)
  (func $block-unwind-by-br-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      i64.const 1
      i32.const 9
      br $B0
    end)
  (func $block-unwind-by-br_if (type $t1) (result i32)
    block $B0
      i32.const 3
      i64.const 1
      i32.const 1
      br_if $B0
      drop
      drop
    end
    i32.const 9)
  (func $block-unwind-by-br_if-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      i64.const 1
      i32.const 9
      i32.const 1
      br_if $B0
      drop
      drop
    end)
  (func $block-unwind-by-br_table (type $t1) (result i32)
    block $B0
      i32.const 3
      i64.const 1
      i32.const 0
      br_table $B0
    end
    i32.const 9)
  (func $block-unwind-by-br_table-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      i64.const 1
      i32.const 9
      i32.const 0
      br_table $B0
    end)
  (func $block-unwind-by-return (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      i64.const 1
      i32.const 9
      return
    end)
  (func $block-nested-unwind-by-unreachable (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      block $B1
        i64.const 1
        unreachable
      end
    end)
  (func $block-nested-unwind-by-br (type $t1) (result i32)
    block $B0
      i32.const 3
      block $B1
        i64.const 1
        br $B0
      end
      drop
    end
    i32.const 9)
  (func $block-nested-unwind-by-br-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      block $B1
        i64.const 1
        i32.const 9
        br $B0
      end
    end)
  (func $block-nested-unwind-by-br_if (type $t1) (result i32)
    block $B0
      i32.const 3
      block $B1
        i64.const 1
        i32.const 1
        br_if $B0
        drop
      end
      drop
    end
    i32.const 9)
  (func $block-nested-unwind-by-br_if-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      block $B1
        i64.const 1
        i32.const 9
        i32.const 1
        br_if $B0
        drop
        drop
      end
    end)
  (func $block-nested-unwind-by-br_table (type $t1) (result i32)
    block $B0
      i32.const 3
      block $B1
        i64.const 1
        i32.const 1
        br_table $B0
      end
      drop
    end
    i32.const 9)
  (func $block-nested-unwind-by-br_table-value (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      block $B1
        i64.const 1
        i32.const 9
        i32.const 1
        br_table $B0
      end
    end)
  (func $block-nested-unwind-by-return (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 3
      block $B1
        i64.const 1
        i32.const 9
        return
      end
    end)
  (func $unary-after-unreachable (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    unreachable
    i64.eqz)
  (func $unary-after-br (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      br $B0
      i64.eqz
    end)
  (func $unary-after-br_if (type $t1) (result i32)
    block $B0 (result i32)
      i64.const 0
      i32.const 9
      i32.const 1
      br_if $B0
      drop
      i64.eqz
    end)
  (func $unary-after-br_table (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      i32.const 0
      br_table $B0 $B0
      i64.eqz
    end)
  (func $unary-after-return (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    i32.const 9
    return
    i64.eqz)
  (func $binary-after-unreachable (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x1p+0 (;=1;)
    unreachable
    i64.eq)
  (func $binary-after-br (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      f64.const 0x1p+0 (;=1;)
      i32.const 9
      br $B0
      i64.eq
    end)
  (func $binary-after-br_if (type $t1) (result i32)
    block $B0 (result i32)
      i64.const 0
      i64.const 1
      i32.const 9
      i32.const 1
      br_if $B0
      drop
      i64.eq
    end)
  (func $binary-after-br_table (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      f64.const 0x1p+0 (;=1;)
      i32.const 9
      i32.const 0
      br_table $B0
      i64.eq
    end)
  (func $binary-after-return (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x1p+0 (;=1;)
    i32.const 9
    return
    i64.eq)
  (func $select-after-unreachable (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x1p+0 (;=1;)
    i64.const 0
    unreachable
    select)
  (func $select-after-br (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      f64.const 0x1p+0 (;=1;)
      i64.const 0
      i32.const 9
      br $B0
      select
    end)
  (func $select-after-br_if (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 1
      i32.const 0
      i32.const 9
      i32.const 1
      br_if $B0
      drop
      select
    end)
  (func $select-after-br_table (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      f64.const 0x1p+0 (;=1;)
      i64.const 0
      i32.const 9
      i32.const 0
      br_table $B0
      select
    end)
  (func $select-after-return (type $t1) (result i32)
    f32.const 0x0p+0 (;=0;)
    f64.const 0x1p+0 (;=1;)
    i64.const 1
    i32.const 9
    return
    select)
  (func $block-value-after-unreachable (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      unreachable
    end)
  (func $block-value-after-br (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      br $B0
    end)
  (func $block-value-after-br_if (type $t1) (result i32)
    block $B0 (result i32)
      i32.const 0
      i32.const 9
      i32.const 1
      br_if $B0
      drop
    end)
  (func $block-value-after-br_table (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      i32.const 0
      br_table $B0 $B0
    end)
  (func $block-value-after-return (type $t1) (result i32)
    block $B0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      return
    end)
  (func $loop-value-after-unreachable (type $t1) (result i32)
    loop $L0 (result i32)
      f32.const 0x0p+0 (;=0;)
      unreachable
    end)
  (func $loop-value-after-br (type $t1) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        f32.const 0x0p+0 (;=0;)
        i32.const 9
        br $B0
      end
    end)
  (func $loop-value-after-br_if (type $t1) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        i32.const 0
        i32.const 9
        i32.const 1
        br_if $B0
        drop
      end
    end)
  (func $loop-value-after-br_table (type $t1) (result i32)
    block $B0 (result i32)
      loop $L1 (result i32)
        f32.const 0x0p+0 (;=0;)
        i32.const 9
        i32.const 0
        br_table $B0 $B0
      end
    end)
  (func $loop-value-after-return (type $t1) (result i32)
    loop $L0 (result i32)
      f32.const 0x0p+0 (;=0;)
      i32.const 9
      return
    end)
  (export "func-unwind-by-unreachable" (func $func-unwind-by-unreachable))
  (export "func-unwind-by-br" (func $func-unwind-by-br))
  (export "func-unwind-by-br-value" (func $func-unwind-by-br-value))
  (export "func-unwind-by-br_if" (func $func-unwind-by-br_if))
  (export "func-unwind-by-br_if-value" (func $func-unwind-by-br_if-value))
  (export "func-unwind-by-br_table" (func $func-unwind-by-br_table))
  (export "func-unwind-by-br_table-value" (func $func-unwind-by-br_table-value))
  (export "func-unwind-by-return" (func $func-unwind-by-return))
  (export "block-unwind-by-unreachable" (func $block-unwind-by-unreachable))
  (export "block-unwind-by-br" (func $block-unwind-by-br))
  (export "block-unwind-by-br-value" (func $block-unwind-by-br-value))
  (export "block-unwind-by-br_if" (func $block-unwind-by-br_if))
  (export "block-unwind-by-br_if-value" (func $block-unwind-by-br_if-value))
  (export "block-unwind-by-br_table" (func $block-unwind-by-br_table))
  (export "block-unwind-by-br_table-value" (func $block-unwind-by-br_table-value))
  (export "block-unwind-by-return" (func $block-unwind-by-return))
  (export "block-nested-unwind-by-unreachable" (func $block-nested-unwind-by-unreachable))
  (export "block-nested-unwind-by-br" (func $block-nested-unwind-by-br))
  (export "block-nested-unwind-by-br-value" (func $block-nested-unwind-by-br-value))
  (export "block-nested-unwind-by-br_if" (func $block-nested-unwind-by-br_if))
  (export "block-nested-unwind-by-br_if-value" (func $block-nested-unwind-by-br_if-value))
  (export "block-nested-unwind-by-br_table" (func $block-nested-unwind-by-br_table))
  (export "block-nested-unwind-by-br_table-value" (func $block-nested-unwind-by-br_table-value))
  (export "block-nested-unwind-by-return" (func $block-nested-unwind-by-return))
  (export "unary-after-unreachable" (func $unary-after-unreachable))
  (export "unary-after-br" (func $unary-after-br))
  (export "unary-after-br_if" (func $unary-after-br_if))
  (export "unary-after-br_table" (func $unary-after-br_table))
  (export "unary-after-return" (func $unary-after-return))
  (export "binary-after-unreachable" (func $binary-after-unreachable))
  (export "binary-after-br" (func $binary-after-br))
  (export "binary-after-br_if" (func $binary-after-br_if))
  (export "binary-after-br_table" (func $binary-after-br_table))
  (export "binary-after-return" (func $binary-after-return))
  (export "select-after-unreachable" (func $select-after-unreachable))
  (export "select-after-br" (func $select-after-br))
  (export "select-after-br_if" (func $select-after-br_if))
  (export "select-after-br_table" (func $select-after-br_table))
  (export "select-after-return" (func $select-after-return))
  (export "block-value-after-unreachable" (func $block-value-after-unreachable))
  (export "block-value-after-br" (func $block-value-after-br))
  (export "block-value-after-br_if" (func $block-value-after-br_if))
  (export "block-value-after-br_table" (func $block-value-after-br_table))
  (export "block-value-after-return" (func $block-value-after-return))
  (export "loop-value-after-unreachable" (func $loop-value-after-unreachable))
  (export "loop-value-after-br" (func $loop-value-after-br))
  (export "loop-value-after-br_if" (func $loop-value-after-br_if))
  (export "loop-value-after-br_table" (func $loop-value-after-br_table))
  (export "loop-value-after-return" (func $loop-value-after-return)))
