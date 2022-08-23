(module
    (import "host" "element_offset" (global $element_offset i32))
    (func $main (export "main")
        (nop)
    )
    (table $table 1 funcref)
    (elem $table (global.get $element_offset) $main)
)