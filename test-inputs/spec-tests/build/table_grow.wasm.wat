(module
  (type $t0 (func (param i32) (result i32)))
  (type $t1 (func (param i32 i32) (result funcref)))
  (func $grow (type $t0) (param $p0 i32) (result i32)
    ref.null func
    local.get $p0
    table.grow $T0)
  (func $check-table-null (type $t1) (param $p0 i32) (param $p1 i32) (result funcref)
    (local $l2 funcref)
    ref.func $check-table-null
    local.set $l2
    block $B0
      loop $L1
        local.get $p0
        table.get $T0
        local.set $l2
        local.get $l2
        ref.is_null
        i32.eqz
        br_if $B0
        local.get $p0
        local.get $p1
        i32.ge_u
        br_if $B0
        local.get $p0
        i32.const 1
        i32.add
        local.set $p0
        local.get $p0
        local.get $p1
        i32.le_u
        br_if $L1
      end
    end
    local.get $l2)
  (table $T0 10 funcref)
  (export "grow" (func $grow))
  (export "check-table-null" (func $check-table-null))
  (elem $e0 declare func $check-table-null))
