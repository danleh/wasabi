(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i32 i32) (result i32)))
  (type $t2 (func (param i32)))
  (type $t3 (func))
  (type $t4 (func (result i32)))
  (import "wasi_snapshot_preview1" "args_sizes_get" (func $__wasi_args_sizes_get (type $t1)))
  (import "wasi_snapshot_preview1" "args_get" (func $__wasi_args_get (type $t1)))
  (import "wasi_snapshot_preview1" "proc_exit" (func $__wasi_proc_exit (type $t2)))
  (func $__wasm_call_ctors (type $t3)
    (nop))
  (func $module1_bool_ (type $t0) (param $p0 i32) (result i32)
    (local $l1 i32)
    (global.set $__stack_pointer
      (local.tee $l1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=8
      (local.get $l1)
      (i32.const 1032))
    (i32.store
      (local.get $l1)
      (i32.const 1044))
    (local.set $p0
      ;; This 
      (call_indirect $__indirect_function_table (type $t0)
        (local.tee $p0
          (select
            (i32.add
              (local.get $l1)
              (i32.const 8))
            (local.get $l1)
            (local.get $p0)))
        (i32.load
          (i32.load
            (local.get $p0)))))
    (global.set $__stack_pointer
      (i32.add
        (local.get $l1)
        (i32.const 16)))
    (local.get $p0))
  (func $module2_bool_ (type $t0) (param $p0 i32) (result i32)
    (local $l1 i32)
    (global.set $__stack_pointer
      (local.tee $l1
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (i32.store offset=8
      (local.get $l1)
      (i32.const 1056))
    (i32.store
      (local.get $l1)
      (i32.const 1068))
    (local.set $p0
      (call_indirect $__indirect_function_table (type $t0)
        (local.tee $p0
          (select
            (i32.add
              (local.get $l1)
              (i32.const 8))
            (local.get $l1)
            (local.get $p0)))
        (i32.load
          (i32.load
            (local.get $p0)))))
    (global.set $__stack_pointer
      (i32.add
        (local.get $l1)
        (i32.const 16)))
    (local.get $p0))
  (func $main (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
    (i32.add
      (call $module1_bool_
        (local.tee $p0
          (i32.gt_s
            (local.get $p0)
            (i32.const 1))))
      (call $module2_bool_
        (local.get $p0))))
  (func $A::method__ (type $t0) (param $p0 i32) (result i32)
    (i32.const 23))
  (func $B::method__ (type $t0) (param $p0 i32) (result i32)
    (i32.const 42))
  (func $C::method__ (type $t0) (param $p0 i32) (result i32)
    (i32.const 1))
  (func $D::method__ (type $t0) (param $p0 i32) (result i32)
    (i32.const 2))
  (func $_start (type $t3)
    (call $__wasm_call_ctors)
    (call $exit
      (call $__original_main))
    (unreachable))
  (func $__original_main (type $t4) (result i32)
    (call $__main_void))
  (func $__main_void (type $t4) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    (global.set $__stack_pointer
      (local.tee $l0
        (i32.sub
          (global.get $__stack_pointer)
          (i32.const 16))))
    (block $B0
      (if $I1
        (i32.eqz
          (call $__wasi_args_sizes_get
            (i32.add
              (local.tee $l1
                (local.get $l0))
              (i32.const 12))
            (i32.add
              (local.get $l0)
              (i32.const 8))))
        (then
          (local.set $l0
            (call $main.1
              (local.tee $l2
                (if $I2 (result i32)
                  (local.tee $l2
                    (i32.load offset=12
                      (local.get $l1)))
                  (then
                    (global.set $__stack_pointer
                      (local.tee $l3
                        (local.tee $l0
                          (i32.sub
                            (local.get $l0)
                            (i32.and
                              (i32.add
                                (local.tee $l2
                                  (i32.shl
                                    (local.get $l2)
                                    (i32.const 2)))
                                (i32.const 19))
                              (i32.const -16))))))
                    (global.set $__stack_pointer
                      (local.tee $l3
                        (i32.sub
                          (local.get $l3)
                          (i32.and
                            (i32.add
                              (i32.load offset=8
                                (local.get $l1))
                              (i32.const 15))
                            (i32.const -16)))))
                    (i32.store
                      (i32.add
                        (local.get $l0)
                        (local.get $l2))
                      (i32.const 0))
                    (br_if $B0
                      (call $__wasi_args_get
                        (local.get $l0)
                        (local.get $l3)))
                    (i32.load offset=12
                      (local.get $l1)))
                  (else
                    (i32.const 0))))
              (local.get $l0)))
          (global.set $__stack_pointer
            (i32.add
              (local.get $l1)
              (i32.const 16)))
          (return
            (local.get $l0))))
      (call $__wasi_proc_exit
        (i32.const 71))
      (unreachable))
    (call $__wasi_proc_exit
      (i32.const 71))
    (unreachable))
  (func $main.1 (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
    (call $main
      (local.get $p0)
      (local.get $p1)))
  (func $_Exit (type $t2) (param $p0 i32)
    (call $__wasi_proc_exit
      (local.get $p0))
    (unreachable))
  (func $libc_exit_fini (type $t3)
    (call $_fini))
  (func $exit (type $t2) (param $p0 i32)
    (call $_fini)
    (call $libc_exit_fini)
    (call $_fini)
    (call $_Exit
      (local.get $p0))
    (unreachable))
  (func $_fini (type $t3)
    (nop))
  (func $stackSave (type $t4) (result i32)
    (global.get $__stack_pointer))
  (func $stackRestore (type $t2) (param $p0 i32)
    (global.set $__stack_pointer
      (local.get $p0)))
  (func $stackAlloc (type $t0) (param $p0 i32) (result i32)
    (local $l1 i32)
    (global.set $__stack_pointer
      (local.tee $l1
        (i32.and
          (i32.sub
            (global.get $__stack_pointer)
            (local.get $p0))
          (i32.const -16))))
    (local.get $l1))
  (func $__errno_location (type $t4) (result i32)
    (i32.const 1072))
  (table $__indirect_function_table 6 6 funcref)
  (memory $memory 256 256)
  (global $__stack_pointer (mut i32) (i32.const 5243968))
  (export "memory" (memory 0))
  (export "__indirect_function_table" (table 0))
  (export "_start" (func $_start))
  (export "__errno_location" (func $__errno_location))
  (export "stackSave" (func $stackSave))
  (export "stackRestore" (func $stackRestore))
  (export "stackAlloc" (func $stackAlloc))
  ;; Offset = 1 -> index 0 is nullptr, so crashes if called.
  (elem $e0 (i32.const 1) func $A::method__ $B::method__ $C::method__ $D::method__ $__wasm_call_ctors)
  ;; vtables for the four classes.
  (data $.rodata (i32.const 1032) "\01")
  (data $.rodata.1 (i32.const 1044) "\02")
  (data $.rodata.2 (i32.const 1056) "\03")
  (data $.rodata.3 (i32.const 1068) "\04"))
