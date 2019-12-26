(module 
  (type $t1 (func (param i32) (param i32)))
  (import "a" "a" (func $exec (type $t1)))
  ;; offset=448 tape
  ;; offset=512 out
  (memory $mem (export "b") 1)
  (data $mem (i32.const 0) "<<redacted>>")
  (table $interp 94 anyfunc)
  (elem $interp (i32.const 43) $inc)
  (elem $interp (i32.const 45) $dec)
  (elem $interp (i32.const 60) $shl)
  (elem $interp (i32.const 62) $shr)
  (elem $interp (i32.const 91) $start)
  (elem $interp (i32.const 93) $end)
  (elem $interp (i32.const 46) $prn)
  (global $pc (mut i32) (i32.const 0))
  (global $ptr (mut i32) (i32.const 0))
  (global $stack (mut i32) (i32.const 0))
  (global $out (mut i32) (i32.const 0))
  (func $inc
    get_global $ptr
    get_global $ptr
    i32.load8_u offset=448
    i32.const 1
    i32.add
    i32.store8 offset=448)
  (func $dec
    get_global $ptr
    get_global $ptr
    i32.load8_u offset=448
    i32.const 1
    i32.sub
    i32.store8 offset=448)
  (func $shl
    get_global $ptr
    i32.const 1
    i32.sub
    set_global $ptr)
  (func $shr
    get_global $ptr
    i32.const 1
    i32.add
    set_global $ptr)
  (func $start
    get_global $ptr
    i32.load8_u offset=448
    i32.eqz
    if
      call $skip
    else
      get_global $pc
      set_global $stack
    end)
  (func $end
    get_global $ptr
    i32.load8_u offset=448
    i32.eqz
    if
    else
      call $back
    end)
  (func $prn
    get_global $out
    get_global $ptr
    i32.load8_u offset=448
    i32.store8 offset=512
    get_global $out
    i32.const 1
    i32.add
    set_global $out)
  (func $skip (local i32)
    get_global $pc
    set_local 0
    loop $loop
      get_local 0
      i32.const 1
      i32.add
      tee_local 0
      i32.load8_u offset=0
      i32.const 93
      i32.ne
      br_if $loop
    end
    get_local 0
    set_global $pc)
  (func $back
    get_global $stack
    set_global $pc)
  (func $bf (local i32)
    i32.const 418
    set_local 0
    loop $loop
      get_local 0
      get_global $pc
      i32.load8_u offset=0
      call_indirect
      get_global $pc
      i32.const 1
      i32.add
      set_global $pc
      get_global $pc
      i32.gt_s
      br_if $loop
    end)
  (func $decode (param $key i32) (param $start i32) (param $length i32)
    get_local $length
    i32.const 0
    i32.gt_s
    if
      loop $loop
        get_local $start
        get_local $key
        get_local $start
        i32.load8_u
        i32.xor 
        i32.store8
        get_local $start
        i32.const 1
        i32.add
        set_local $start
        get_local $length
        i32.const 1
        i32.sub
        tee_local $length
        i32.const 0
        i32.gt_u
        br_if $loop
      end
    end)
  (func $entry (export "c")
    call $bf
    i32.const 29
    i32.const 512
    i32.const 41
    call $decode
    i32.const 512
    i32.const 41
    call $exec))
