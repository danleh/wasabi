(module
  (func

    ;; Default alignment and explicitly given, see how that is encoded in the binary.
    i32.const 0
    i32.const 0
    i32.store
    i32.const 0
    i32.const 0
    i32.store align=1
    i32.const 0
    i32.const 0
    i32.store align=2  
    i32.const 0
    i32.const 0
    i32.store align=4  

    ;; Quick test for i64 as well.
    i32.const 0
    i64.const 0
    i64.store
    i32.const 0
    i64.const 0
    i64.store align=8

    ;; error: alignment must not be larger than natural alignment (8)
    ;; i32.const 0
    ;; i64.const 0
    ;; i64.store align=16

    ;; Default alignments for shorter load instructions.
    i32.const 0
    i32.load
    drop
    i32.const 0
    i32.load8_s
    drop
    i32.const 0
    i32.load8_u
    drop
    i32.const 0
    i32.load16_s
    drop
    i32.const 0
    i32.load16_u
    drop
    
  )
  (memory 1)
)
