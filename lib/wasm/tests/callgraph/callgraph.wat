(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (param i32) (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (func $main (type 2) ;;f0
    i32.const 42
    i32.const 7
    call $sub
    call $dispatcher)
  (func $dispatcher (type 3) (param $p0 i32) ;;f1
    local.get $p0
    local.get $p0
    i32.const 1
    call_indirect (type 0)
    drop
    local.get $p0
    i32.const 0
    call_indirect (type 1)
    drop)
  (func $inc (type 1) (param i32) (result i32) ;;f2
    i32.const 1
    local.get 0
    i32.add)
  (func $add (type 0) (param i32 i32) (result i32) ;;f3
    local.get 0
    local.get 1
    i32.add)
  (func $sub (type 0) (param i32 i32) (result i32) ;;f4
    local.get 0
    local.get 1
    i32.sub)
  (func $dead-by-ty (type 2) ;;f5
    nop)
  (table (;0;) 4 funcref)
  (export "_main" (func $main))
  ;; (elem (i32.const 0) func $inc $add $dead-by-ty $sub)
  (elem (i32.const 0) $inc $add $dead-by-ty $sub))
