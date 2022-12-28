(module
  (type $t0 (func (param i32)))
  (type $t1 (func (param i32 i32)))
  (import "spectest" "print_i32" (func $spectest.print_i32 (type $t0)))
  (import "spectest" "print_i32" (func $spectest.print_i32_1 (type $t0)))
  (func $print32 (type $t1) (param $p0 i32) (param $p1 i32)
    local.get $p0
    call $spectest.print_i32
    local.get $p1
    call $spectest.print_i32_1)
  (export "print32" (func $print32)))
