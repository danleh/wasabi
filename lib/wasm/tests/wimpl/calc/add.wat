(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (result i32)))
  (type (;4;) (func (param i32 i32 i32) (result i32)))
  (type (;5;) (func (param i32)))
  (func $__wasm_call_ctors (type 1)
    (nop))
  (func $no_use (type 0) (param i32 i32) (result i32)
    (i32.const 0))
  (func $add (type 0) (param i32 i32) (result i32)
    (i32.add
      (local.get 0)
      (local.get 1)))
  (func $sub (type 0) (param i32 i32) (result i32)
    (i32.sub
      (local.get 0)
      (local.get 1)))
  (func $mul (type 0) (param i32 i32) (result i32)
    (i32.mul
      (local.get 0)
      (local.get 1)))
  (func $temp (type 4) (param i32 i32 i32) (result i32)
    (local i32)
    (local.set 3
      (i32.const 1))
    (call_indirect (type 0)
      (local.get 0)
      (local.get 1)
      (block (result i32)  ;; label = @1
        (if  ;; label = @2
          (i32.le_u
            (local.tee 2
              (i32.sub
                (local.get 2)
                (i32.const 1)))
            (i32.const 2))
          (then
            (local.set 3
              (i32.load
                (i32.add
                  (i32.shl
                    (local.get 2)
                    (i32.const 2))
                  (i32.const 1024))))))
        (local.get 3))))
  (func $inc (type 2) (param i32) (result i32)
    (i32.add
      (local.get 0)
      (i32.const 1)))
  (func $_initialize (type 1)
    (call $__wasm_call_ctors))
  (func $stackSave (type 3) (result i32)
    (global.get $__stack_pointer))
  (func $stackRestore (type 5) (param i32)
    (global.set $__stack_pointer
      (local.get 0)))
  (func $stackAlloc (type 2) (param i32) (result i32)
    (local i32)
    (global.set $__stack_pointer
      (local.tee 1
        (i32.and
          (i32.sub
            (global.get $__stack_pointer)
            (local.get 0))
          (i32.const -16))))
    (local.get 1))
  (func $__errno_location (type 3) (result i32)
    (i32.const 1036))
  (table (;0;) 6 6 funcref)
  (memory (;0;) 256 256)
  (global $__stack_pointer (mut i32) (i32.const 5243920))
  (export "memory" (memory 0))
  (export "temp" (func $temp))
  (export "inc" (func $inc))
  (export "__indirect_function_table" (table 0))
  (export "_initialize" (func $_initialize))
  (export "__errno_location" (func $__errno_location))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  (elem (;0;) (i32.const 1) func $no_use $add $sub $mul $__wasm_call_ctors)
  (data $.rodata (i32.const 1024) "\02\00\00\00\03\00\00\00\04"))
