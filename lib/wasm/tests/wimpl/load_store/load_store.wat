(module
  (func $1 
    (i32.const 42) 	;; value
    (i32.const 0) 	;; addr
    (i32.store offset=0) 
    (i32.const 0) 
    (i32.load offset=0)
    (drop)
  )
  (memory (;0;) 256 256)
)
