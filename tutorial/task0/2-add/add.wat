(module
  ;; Export this function under the name "add".
  ;; Note: exports have only a single "level", unlike imports, which have two.
  (func (export "add") (param i32 i32) (result i32)
    ;; Function arguments are (implicitly) in the first n_arg locals.
    ;; Locals are function-wide variables that are independent of the stack.
    ;; local.get "loads" a local onto the stack.
    local.get 1
    local.get 0
    i32.add
    ;; Implicitly return the top value on the stack as the function result.
  )
)
