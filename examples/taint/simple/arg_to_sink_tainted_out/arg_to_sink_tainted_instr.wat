(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32 i32)))
  (type (;2;) (func (param i32 i32 i32 i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func))
  (import "__wasabi_hooks" "begin_function" (func (;0;) (type 0)))
  (import "__wasabi_hooks" "start" (func (;1;) (type 0)))
  (import "__wasabi_hooks" "return" (func (;2;) (type 0)))
  (import "__wasabi_hooks" "end_function" (func (;3;) (type 0)))
  (import "__wasabi_hooks" "i32_const" (func (;4;) (type 1)))
  (import "__wasabi_hooks" "local_set_i" (func (;5;) (type 2)))
  (import "__wasabi_hooks" "local_get_i" (func (;6;) (type 2)))
  (import "__wasabi_hooks" "call_i" (func (;7;) (type 2)))
  (import "__wasabi_hooks" "call_post" (func (;8;) (type 0)))
  (func (;9;) (type 3) (param i32)
    i32.const 0
    i32.const -1
    call 0
    i32.const 0
    i32.const -1
    call 2
    i32.const 0
    i32.const 0
    call 3)
  (func (;10;) (type 3) (param i32)
    i32.const 1
    i32.const -1
    call 0
    i32.const 1
    i32.const -1
    call 2
    i32.const 1
    i32.const 0
    call 3)
  (func (;11;) (type 3) (param i32)
    (local i32)
    i32.const 2
    i32.const -1
    call 0
    local.get 0
    i32.const 2
    i32.const 0
    i32.const 0
    local.get 0
    call 6
    local.tee 1
    i32.const 2
    i32.const 1
    i32.const 1
    local.get 1
    call 7
    call 10
    i32.const 2
    i32.const 1
    call 8
    i32.const 2
    i32.const -1
    call 2
    i32.const 2
    i32.const 2
    call 3)
  (func (;12;) (type 4)
    (local i32 i32 i32)
    global.get 0
    if  ;; label = @1
      i32.const 0
      global.set 0
      i32.const 3
      i32.const -1
      call 1
    end
    i32.const 3
    i32.const -1
    call 0
    i32.const 33
    i32.const 3
    i32.const 0
    i32.const 33
    call 4
    local.set 0
    i32.const 3
    i32.const 1
    i32.const 0
    local.get 0
    call 5
    local.get 0
    i32.const 3
    i32.const 2
    i32.const 0
    local.get 0
    call 6
    local.tee 1
    i32.const 3
    i32.const 3
    i32.const 0
    local.get 1
    call 7
    call 9
    i32.const 3
    i32.const 3
    call 8
    local.get 0
    i32.const 3
    i32.const 4
    i32.const 0
    local.get 0
    call 6
    local.tee 2
    i32.const 3
    i32.const 5
    i32.const 2
    local.get 2
    call 7
    call 11
    i32.const 3
    i32.const 5
    call 8
    i32.const 3
    i32.const -1
    call 2
    i32.const 3
    i32.const 6
    call 3)
  (global (;0;) (mut i32) (i32.const 1))
  (export "taint_source" (func 9))
  (export "taint_sink" (func 10))
  (start 12))
