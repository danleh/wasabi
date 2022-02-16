(module
  (type $binary (func (param i32 i32) (result i32)))
  (type $unary (func (param i32) (result i32)))
  (type $void (func))
  (table $table 1 anyfunc)
  (func $main
    i32.const 42
    call $dispatcher
  )
  (func $dispatcher (param $p0 i32)
    ;; indirect call with fixed index
    local.get $p0
    local.get $p0
    i32.const 0 ;; table index
    call_indirect (type $binary)
    drop

    ;; indirect call with a different type
    local.get $p0
    i32.const 0 ;; table index
    call_indirect (type $unary)
    drop

  )
  (func $inc (type $unary)
    i32.const 1
    local.get 0
    i32.add
  )
  (func $add (type $binary)
    local.get 0
    local.get 1
    i32.add
  )
  (func $sub (type $binary)
    local.get 0
    local.get 1
    i32.sub
  )
  (func $dead-by-ty (type $void)
    nop
  )
  (elem (i32.const 0) $inc $add $dead-by-ty)
  (export "main" (func $main))
)
