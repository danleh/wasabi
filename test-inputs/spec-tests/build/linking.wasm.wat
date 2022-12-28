(module
  (type $t0 (func (result i32)))
  (func $get_memory_0_ (type $t0) (result i32)
    i32.const 0
    i32.load8_u)
  (func $get_table_0_ (type $t0) (result i32)
    i32.const 0
    call_indirect $table (type $t0))
  (table $table 1 funcref)
  (memory $memory 1)
  (export "memory" (memory 0))
  (export "table" (table 0))
  (export "get memory[0]" (func $get_memory_0_))
  (export "get table[0]" (func $get_table_0_)))
