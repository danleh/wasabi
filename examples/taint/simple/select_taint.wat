(module
    (import "imports" "output" (func $print (;0;) (param i32)))

    (func $source (param i32))
    (export "taint_source" (func $source))
    (func $sink (param i32))
    (export "taint_sink" (func $sink))

    (func $f (local $locA i32)
      i32.const 10
      local.set $locA

      ;; mark locA as tainted
      local.get $locA
      call $source

      local.get $locA ;; select if true
      i32.const 20    ;; select if false
      
      i32.const 1     ;; true
      select
      
      ;; pass value to sink
      call $sink
    )

    (start $f)
)
