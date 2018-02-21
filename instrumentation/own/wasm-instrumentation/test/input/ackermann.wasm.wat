(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func))
  (func (;0;) (type 0) (param i32 i32) (result i32)
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
          call 0
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
  (func (;1;) (type 0) (param i32 i32) (result i32)
    i32.const 0)
  (func (;2;) (type 1)
    (local i32 i32)
    get_local 0
    get_local 1
    call 1
    drop)
  (table (;0;) 0 anyfunc)
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "ackermann" (func 0))
  (export "main" (func 1))
  (start 2)
  (data (i32.const 4) "\10\00\10\00"))
