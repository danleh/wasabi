(module
  (func $f0
    (local $l0 i32)
    ;; See the expected runtime stack and local state on the right.
    i32.const 3   ;; [3]
    local.tee $l0 ;; [3], {$l0 -> 3}
    i32.const 7   ;; [3, 7]
    local.set $l0 ;; [3], {$l0 -> 7}
    local.get $l0 ;; [3, 7]
    i32.add       ;; [10]
    drop          ;; []

    ;; The same exact code with local.set and local.get instead of local.tee
    i32.const 3   ;; [3]
    local.set $l0 ;; [], {$l0 -> 3}
    local.get $l0 ;; [3], {$l0 -> 3}
    i32.const 7   ;; [3, 7]
    local.set $l0 ;; [3], {$l0 -> 7}
    local.get $l0 ;; [3, 7]
    i32.add       ;; [10]
    drop          ;; []

    ;; This should translate to only a single assignment.
    i32.const 3   ;; [3]
    local.tee $l0 ;; [3], {$l0 -> 3}
    local.get $l0 ;; [3, 3], {$l0 -> 3}
    i32.add       ;; [10]
    drop          ;; []
  )
)
