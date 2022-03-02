(module
  (func $f0 (result i32)
    (local $l0 i32)
    ;; with expected runtime stack and local state
    i32.const 3   ;; [3]
    local.tee $l0 ;; [3], {$l0 -> 3}
    i32.const 7   ;; [3, 7]
    local.set $l0 ;; [3], {$l0 -> 7}
    local.get $l0 ;; [3, 7]
    i32.add       ;; [10]
  )
)
