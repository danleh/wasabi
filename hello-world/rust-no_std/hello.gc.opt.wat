(module
  (type (;0;) (func (param i32 i32 i32)))
  (func (;0;) (type 0) (param i32 i32 i32)
    get_local 2
    if  ;; label = @1
      get_local 0
      get_local 1
      i32.load
      i32.store offset=4
      get_local 0
      i32.const 1
      i32.store
      return
    end
    get_local 0
    i32.const 0
    i32.store)
  (memory (;0;) 17)
  (export "memory" (memory 0))
  (export "start" (func 0))
  (data (i32.const 4) "\10\00\10"))
