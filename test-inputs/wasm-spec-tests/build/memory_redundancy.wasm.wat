(module
  (type $t0 (func))
  (type $t1 (func (result i32)))
  (type $t2 (func (result f32)))
  (type $t3 (func (param i32) (result i32)))
  (func $zero_everything (type $t0)
    i32.const 0
    i32.const 0
    i32.store
    i32.const 4
    i32.const 0
    i32.store
    i32.const 8
    i32.const 0
    i32.store
    i32.const 12
    i32.const 0
    i32.store)
  (func $test_store_to_load (type $t1) (result i32)
    i32.const 8
    i32.const 0
    i32.store
    i32.const 5
    f32.const -0x0p+0 (;=-0;)
    f32.store
    i32.const 8
    i32.load)
  (func $test_redundant_load (type $t1) (result i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 8
    i32.load
    local.set $l0
    i32.const 5
    i32.const -2147483648
    i32.store
    i32.const 8
    i32.load
    local.set $l1
    local.get $l0
    local.get $l1
    i32.add)
  (func $test_dead_store (type $t2) (result f32)
    (local $l0 f32)
    i32.const 8
    i32.const 589505315
    i32.store
    i32.const 11
    f32.load
    local.set $l0
    i32.const 8
    i32.const 0
    i32.store
    local.get $l0)
  (func $malloc (type $t3) (param $p0 i32) (result i32)
    i32.const 16)
  (func $malloc_aliasing (type $t1) (result i32)
    (local $l0 i32) (local $l1 i32)
    i32.const 4
    call $malloc
    local.set $l0
    i32.const 4
    call $malloc
    local.set $l1
    local.get $l0
    i32.const 42
    i32.store
    local.get $l1
    i32.const 43
    i32.store
    local.get $l0
    i32.load)
  (memory $M0 1 1)
  (export "zero_everything" (func $zero_everything))
  (export "test_store_to_load" (func $test_store_to_load))
  (export "test_redundant_load" (func $test_redundant_load))
  (export "test_dead_store" (func $test_dead_store))
  (export "malloc" (func $malloc))
  (export "malloc_aliasing" (func $malloc_aliasing)))
