(module
  (type $t0 (func))
  (func $as-block-value (type $t0)
    block $B0
      i32.const 0
      i32.const 1
      i32.store
    end)
  (func $as-loop-value (type $t0)
    loop $L0
      i32.const 0
      i32.const 1
      i32.store
    end)
  (func $as-br-value (type $t0)
    block $B0
      i32.const 0
      i32.const 1
      i32.store
      br $B0
    end)
  (func $as-br_if-value (type $t0)
    block $B0
      i32.const 0
      i32.const 1
      i32.store
      i32.const 1
      br_if $B0
    end)
  (func $as-br_if-value-cond (type $t0)
    block $B0
      i32.const 6
      i32.const 0
      i32.const 1
      i32.store
      br_if $B0
    end)
  (func $as-br_table-value (type $t0)
    block $B0
      i32.const 0
      i32.const 1
      i32.store
      i32.const 1
      br_table $B0
    end)
  (func $as-return-value (type $t0)
    i32.const 0
    i32.const 1
    i32.store
    return)
  (func $as-if-then (type $t0)
    i32.const 1
    if $I0
      i32.const 0
      i32.const 1
      i32.store
    end)
  (func $as-if-else (type $t0)
    i32.const 0
    if $I0
    else
      i32.const 0
      i32.const 1
      i32.store
    end)
  (memory $M0 1)
  (export "as-block-value" (func $as-block-value))
  (export "as-loop-value" (func $as-loop-value))
  (export "as-br-value" (func $as-br-value))
  (export "as-br_if-value" (func $as-br_if-value))
  (export "as-br_if-value-cond" (func $as-br_if-value-cond))
  (export "as-br_table-value" (func $as-br_table-value))
  (export "as-return-value" (func $as-return-value))
  (export "as-if-then" (func $as-if-then))
  (export "as-if-else" (func $as-if-else)))
