(module
  (func (export "main")
    (if 
      (i32.lt_s
        (i32.const 42)
        (i32.const 42)
      )
      (then
        return
      )
      (else
        return
      )
    )
  )
)

