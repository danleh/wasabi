
/// See https://webassembly.org/roadmap/ and https://github.com/WebAssembly/proposals.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WasmExtension {
    // Extensions that are already standardized and merged into WebAssembly 1.1:
    NontrappingFloatToInt,
    SignExtensionOps,
    MultiValue,
    ReferenceTypes,
    BulkMemoryOperations,
    Simd,

    // In rough decreasing order of stability (i.e., increasing order of
    // breaking changes):
    ThreadsAtomics,
    RelaxedSimd,
    Memory64,
    ExceptionHandling,
    ExtendedNameSection,
    TailCalls,
    TypeImports,
    MultiMemory,
    ModuleLinking,
}

impl WasmExtension {
    pub fn name(self) -> &'static str {
        use WasmExtension::*;
        match self {
            NontrappingFloatToInt => "non-trapping float-to-int conversions",
            SignExtensionOps => "sign-extension operators",
            MultiValue => "multiple return/result values",
            ReferenceTypes => "reference types",
            BulkMemoryOperations => "bulk memory operations",
            Simd => "SIMD",

            ThreadsAtomics => "threads and atomics",
            RelaxedSimd => "relaxed SIMD",
            Memory64 => "64-bit memory",
            ExceptionHandling => "exception handling",
            ExtendedNameSection => "extended name section",
            TailCalls => "tail calls",
            TypeImports => "type imports",
            MultiMemory => "multiple memories",
            ModuleLinking => "module linking",
        }
    }

    #[rustfmt::skip]
    pub fn url(self) -> &'static str {
        use WasmExtension::*;
        match self {
            NontrappingFloatToInt => r"https://github.com/WebAssembly/nontrapping-float-to-int-conversions",
            SignExtensionOps => r"https://github.com/WebAssembly/sign-extension-ops",
            MultiValue => r"https://github.com/WebAssembly/multi-value",
            ReferenceTypes => r"https://github.com/WebAssembly/reference-types",
            BulkMemoryOperations => r"https://github.com/WebAssembly/bulk-memory-operations",
            Simd => r"https://github.com/WebAssembly/simd",

            ThreadsAtomics => r"https://github.com/WebAssembly/threads",
            RelaxedSimd => r"https://github.com/WebAssembly/relaxed-simd",
            Memory64 => r"https://github.com/WebAssembly/memory64",
            ExceptionHandling => r"https://github.com/WebAssembly/exception-handling",
            ExtendedNameSection => r"https://github.com/WebAssembly/extended-name-section",
            TailCalls => r"https://github.com/WebAssembly/tail-call",
            TypeImports => r"https://github.com/WebAssembly/proposal-type-imports",
            MultiMemory => r"https://github.com/WebAssembly/multi-memory",
            ModuleLinking => r"https://github.com/WebAssembly/module-linking",
        }
    }
}
