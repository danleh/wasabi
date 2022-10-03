(module
  (func $0
    ;; Those two should have different Wimpl representations.
    (block
      i32.const 3
      drop
      i32.const 7
      drop
    )
    (block
      i32.const 3
      i32.const 7
      drop
      drop
    )
  )
)
