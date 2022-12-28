(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32)))
  (func $size (type $t0) (result i32)
    memory.size)
  (func $grow (type $t1) (param $p0 i32)
    local.get $p0
    memory.grow
    drop)
  (memory $M0 3 8)
  (export "size" (func $size))
  (export "grow" (func $grow)))
