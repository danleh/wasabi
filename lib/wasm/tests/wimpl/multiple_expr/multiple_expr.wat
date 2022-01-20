(module
  (func $0
    i32.const 0
    i32.const 1
    i32.add
    i32.const 3
    i32.add
	f64.const 45.6
	f64.abs
    drop
    drop
  )
;;   (func $1
;;     (i32.store
;;       (i32.add)
;;     )
;;   )
)
;; (module
;;   (type (;0;) (func))
;;   (func (;0;) (type 0)
;;     (i32.add
;;       (i32.const 0)
;;       (i32.const 1))
;;     (drop
;;       (i32.add
;;         (i32.const 2)
;;         (i32.const 3)))
;;     (drop)))
