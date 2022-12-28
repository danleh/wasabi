(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func (param i32 i32 i32)))
  (func $checkRange (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    loop $L0
      local.get $p0
      local.get $p1
      i32.eq
      if $I1
        i32.const -1
        return
      end
      local.get $p0
      i32.load8_u
      local.get $p2
      i32.eq
      if $I2
        local.get $p0
        i32.const 1
        i32.add
        local.set $p0
        br $L0
      end
    end
    local.get $p0
    return)
  (func $run (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32)
    local.get $p0
    local.get $p1
    local.get $p2
    memory.fill)
  (memory $M0 1 1)
  (export "checkRange" (func $checkRange))
  (export "run" (func $run)))
