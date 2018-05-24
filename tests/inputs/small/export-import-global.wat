(module
  (import "bla" "blub" (global $1 i32))
  (export "exported" (global $1))
)
;; note: re-exports of imported stuff is possible
