use ast::*;

// Right now the structure in module ast::* is extremely low-level, i.e., faithful to the original
// encoding (e.g. order of sections, order of types in Type section, width of LEB128 numbers etc.)
// This allows decoding-encoding to round-trip, but is tedious to work with for instrumentation.
// TODO Is round-trip/this "faithfulness" to the exact original representation necessary?
// Or should we only provide a high-level AST that logically captures everything but may be
// serialized differently than the original module?

// TODO Would this higher level Module/AST format be more convenient to work with?
// - no WithSize<T> or Leb128<T>
// - no explicit TypeIdx, all types are inlined and the Type section is built upon serialization
//   with a HashMap to still avoid type duplication, then all inlined types are replaced with idx
//   into the "HashMap".
// - functions combines Type and Code section
// - table combines Table and Element (initialization of tables) section
// - memory combines Memory and Data (initialization of memory) section
struct HighLevelModule {
    start: Option<FunctionIdx>,

    imports: Vec<Import>,
    exports: Vec<Export>,

    functions: Vec<(FuncType, Locals, Expr)>,
    table: (TableType, Vec<(Expr, Vec<FunctionIdx>)>),
    memory: (MemoryType, Vec<(Expr, Vec<u8>)>),
    globals: Vec<Global>,

    custom_sections: Vec<Vec<u8>>,
}

// TODO "streaming AST" API: return Module {} after reading only the first 8 bytes, implement
// Iterator<Item = Section> for Module -> Module must somehow retain the reader to do so...
