(module
  (type $t0 (func (param i32) (result i32)))
  (func $even (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.const 0
    i32.eq
    if $I0 (result i32)
      i32.const 1
    else
      local.get $p0
      i32.const 1
      i32.sub
      call $odd
    end)
  (func $odd (type $t0) (param $p0 i32) (result i32)
    local.get $p0
    i32.const 0
    i32.eq
    if $I0 (result i32)
      i32.const 0
    else
      local.get $p0
      i32.const 1
      i32.sub
      call $even
    end)
  (export "even" (func $even))
  (export "odd" (func $odd)))
