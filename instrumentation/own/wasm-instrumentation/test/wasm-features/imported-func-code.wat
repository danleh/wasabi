(module
  (import "env" "glob" (global $0 i32))
  (global $0 i32 (i32.const 0))
)
;; cannot import and provide global at the same time:
;; error: redefinition of global "$0"
