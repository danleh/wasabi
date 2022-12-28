(module
  (type $t0 (func (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func))
  (type $t3 (func (param i32 i32) (result i32)))
  (import "env" "__original_main" (func $__original_main (type $t0)))
  (import "env" "exit" (func $exit (type $t1)))
  (func $_start (type $t2)
    (local $l0 i32)
    block $B0
      call $__original_main
      local.tee $l0
      i32.eqz
      br_if $B0
      local.get $l0
      call $exit
      unreachable
    end)
  (func $ackermann (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
    block $B0
      local.get $p0
      i32.eqz
      br_if $B0
      loop $L1
        block $B2
          block $B3
            local.get $p1
            br_if $B3
            i32.const 1
            local.set $p1
            br $B2
          end
          local.get $p0
          local.get $p1
          i32.const -1
          i32.add
          call $ackermann
          local.set $p1
        end
        local.get $p0
        i32.const -1
        i32.add
        local.tee $p0
        br_if $L1
      end
    end
    local.get $p1
    i32.const 1
    i32.add)
  (table $T0 1 1 funcref)
  (memory $memory 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (export "memory" (memory 0))
  (export "_start" (func $_start))
  (export "ackermann" (func $ackermann)))
