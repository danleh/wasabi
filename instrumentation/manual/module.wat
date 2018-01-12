(module
  (import "imports" "output" (func $i (param i32)))

  ;; counter instrumentation code to insert top-level
  (global $counter (mut i32) (i32.const 0))
  (export "get_counter" (func $get_counter))
  (func $get_counter (result i32) get_global $counter)
  (func $increment_count
    get_global $counter
    i32.const 1
    i32.add
    set_global $counter
  )
  ;; before every instruction-to-be-counted:
  ;;    call $increment_count

  ;; from ackermann.rs
  (export "ackermann" (func $ackermann))
    (func $ackermann (param i32 i32) (result i32)
      (local i32)
      block  ;; label = @1
        get_local 0
        i32.eqz
        br_if 0 (;@1;)
        loop  ;; label = @2
          get_local 0
          i32.const -1
          i32.add
          set_local 2
          block  ;; label = @3
            get_local 1
            i32.eqz
            br_if 0 (;@3;)
            get_local 0
            get_local 1
            i32.const -1
            i32.add
            call $increment_count ;; to be inserted by instrumentation
            call $ackermann
            set_local 1
            get_local 2
            set_local 0
            get_local 2
            br_if 1 (;@2;)
            br 2 (;@1;)
          end
          i32.const 1
          set_local 1
          get_local 2
          set_local 0
          get_local 2
          br_if 0 (;@2;)
        end
      end
      get_local 1
      i32.const 1
      i32.add)

  ;;(func $start
  ;;  i32.const 42
  ;;  call $increment_count
  ;;  call $i)
  ;;(start $start)
)
