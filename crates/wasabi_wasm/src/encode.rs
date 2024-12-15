//! Code for encoding our AST back to the WebAssembly binary format.
//! Uses `wasm-encoder` for the actual low-level work.

use std::sync::RwLock;

use nohash_hasher::IntMap;
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use wasm_encoder as we;
use wasm_encoder::Encode;

use crate::*;

/// Add marker types for type-safe `Idx<T>` for the low-level binary format.
/// Since I cannot extend wasm_encoder and there cannot be another, clashing `we` module, wrap
/// this in an intermediate `marker` module.
mod marker {
    pub mod we {
        pub struct FunctionType;
        pub struct Function;
        pub struct Global;
        pub struct Table;
        pub struct Memory;
    }
}

#[derive(Default)]
struct EncodeState {
    types_idx: RwLock<FxHashMap<FunctionType, Idx<marker::we::FunctionType>>>,

    // Mapping of indices from the high-level AST to the low-level binary format.
    // This is necessary, because in the WebAssembly binary format all imported elements
    // (functions, globals, etc.) come before (i.e., have a lower index) than all
    // "locally-defined" (i.e., non-imported) elements.
    // We thus have to re-index functions, globals, etc. and use this here to do so.
    function_idx: IntMap<Idx<Function>, Idx<marker::we::Function>>,
    global_idx: IntMap<Idx<Global>, Idx<marker::we::Global>>,
    table_idx: IntMap<Idx<Table>, Idx<marker::we::Table>>,
    memory_idx: IntMap<Idx<Memory>, Idx<marker::we::Memory>>,

    last_encoded_section: Option<SectionId>,
    custom_sections_encoded: usize,
}

macro_rules! encode_state_idx_fns {
    ($insert_fn: ident, $map_fn: ident, $field: ident, $ty: ident, $index_space_str: expr) => {
        /// Add a new mapping from a high-level index to the low-level index in the binary.
        ///
        /// Unlike for types, the high-level index must be new, otherwise the function panics.
        /// If you want to map an existing index, use `map_*_idx` instead.
        fn $insert_fn(&mut self, highlevel_idx: Idx<$ty>) -> Idx<marker::we::$ty> {
            let new_lowlevel_idx = Idx::from(self.$field.len());
            let was_new = self
                .$field
                .insert(highlevel_idx, new_lowlevel_idx)
                .is_none();
            assert!(
                was_new,
                "high-level index {:?} was inserted twice",
                highlevel_idx
            );
            new_lowlevel_idx
        }

        fn $map_fn(&self, highlevel_idx: Idx<$ty>) -> Result<Idx<marker::we::$ty>, EncodeError> {
            self.$field
                .get(&highlevel_idx)
                .copied()
                .ok_or_else(|| EncodeError::index(highlevel_idx, $index_space_str))
        }
    };
}

impl EncodeState {
    fn get_or_insert_type(&self, type_: FunctionType) -> Idx<marker::we::FunctionType> {
        let mut types_idx = self.types_idx.write().unwrap();
        let new_idx = Idx::from(types_idx.len());
        *types_idx.entry(type_).or_insert(new_idx)
    }

    encode_state_idx_fns!(
        insert_function_idx,
        map_function_idx,
        function_idx,
        Function,
        "function"
    );
    encode_state_idx_fns!(insert_table_idx, map_table_idx, table_idx, Table, "table");
    encode_state_idx_fns!(
        insert_memory_idx,
        map_memory_idx,
        memory_idx,
        Memory,
        "memory"
    );
    encode_state_idx_fns!(
        insert_global_idx,
        map_global_idx,
        global_idx,
        Global,
        "global"
    );
}

pub fn encode_module(module: &Module) -> Result<Vec<u8>, EncodeError> {
    let mut encoder = wasm_encoder::Module::new();
    let mut state = EncodeState::default();

    // Note that the order in which the high-level AST is traversed is not equal to the order
    // in which low-level sections are written out to the binary.
    // Consider the type section: It must be the FIRST section of the low-level binary, but the
    // set of all types in the module is only known once we have iterated over all functions,
    // globals, etc. during conversion to the low-level binary format.
    // Alternatively, one could iterate also twice over the high-level module. Once to collect
    // all types, then write the type section, and then once again to encode the rest of the
    // module. This would make the lifetime of the allocated sections a bit shorter.
    // In principle, however, all sections must be fully resident in memory before one can write
    // them out anyway, because the section size in bytes is prepended to its contents.

    // First, traverse all imported functions, globals, etc., such that they are at the
    // beginning of all index spaces.
    let import_section = encode_imports(module, &mut state);

    // Then traverse all non-imported functions, globals, etc., such that their indices and
    // types are in `state`.
    let function_section = encode_functions(module, &mut state);
    let (table_section, element_section) = encode_tables(module, &mut state)?;
    let (memory_section, data_section) = encode_memories(module, &mut state)?;
    let global_section = encode_globals(module, &mut state)?;

    // The code section can also contain types we haven't seen so far (e.g., in `call_indirect`),
    // so it must be processed before encoding the type section.
    // However the functions, globals, tables, etc. referred to in instructions should all
    // already be known from processing the sections above. If NOT, this is an error in the
    // input highlevel module and we report it.
    let code_section = encode_code(module, &mut state)?;

    // Now, `state` contains all types that appear in the module, so we are ready encode the
    // type section.
    let type_section = encode_types(&state);

    // Then, write all sections in the correct order into the binary.
    // For the section order, see https://webassembly.github.io/spec/core/binary/modules.html#binary-module
    // Intersperse the correct custom sections in between as well.
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !type_section.is_empty() {
        encoder.section(&type_section);
    }
    state.last_encoded_section = Some(SectionId::Type);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !import_section.is_empty() {
        encoder.section(&import_section);
    }
    state.last_encoded_section = Some(SectionId::Import);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !function_section.is_empty() {
        encoder.section(&function_section);
    }
    state.last_encoded_section = Some(SectionId::Function);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !table_section.is_empty() {
        encoder.section(&table_section);
    }
    state.last_encoded_section = Some(SectionId::Table);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !memory_section.is_empty() {
        encoder.section(&memory_section);
    }
    state.last_encoded_section = Some(SectionId::Memory);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !global_section.is_empty() {
        encoder.section(&global_section);
    }
    state.last_encoded_section = Some(SectionId::Global);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    let export_section = encode_exports(module, &mut state)?;
    if !export_section.is_empty() {
        encoder.section(&export_section);
    }
    state.last_encoded_section = Some(SectionId::Export);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if let Some(function_idx) = module.start {
        let start_section = we::StartSection {
            function_index: state.map_function_idx(function_idx)?.to_u32(),
        };
        encoder.section(&start_section);
    }
    state.last_encoded_section = Some(SectionId::Start);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !element_section.is_empty() {
        encoder.section(&element_section);
    }
    state.last_encoded_section = Some(SectionId::Element);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !code_section.is_empty() {
        encoder.section(&code_section);
    }
    state.last_encoded_section = Some(SectionId::Code);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    if !data_section.is_empty() {
        encoder.section(&data_section);
    }
    state.last_encoded_section = Some(SectionId::Data);
    encode_and_insert_custom(&mut encoder, &mut state, module);
    // Custom name section is only valid after data section, see
    // https://webassembly.github.io/spec/core/appendix/custom.html#name-section
    let name_section = encode_names(module, &state)?;
    if let Some(name_section) = name_section {
        encoder.section(&name_section);
        state.last_encoded_section = Some(SectionId::Custom("name".to_string()));
    }
    encode_and_insert_custom(&mut encoder, &mut state, module);

    Ok(encoder.finish())
}

fn encode_imports(module: &Module, state: &mut EncodeState) -> we::ImportSection {
    let mut import_section = we::ImportSection::new();

    macro_rules! add_imports {
        ($elem_iter: ident, $state_insert_fn: ident, $ll_import_type: ident, $hl_to_ll_closure: expr) => {
            for (hl_idx, elem) in module.$elem_iter() {
                if let Some((module_name, name)) = elem.import() {
                    state.$state_insert_fn(hl_idx);
                    import_section.import(
                        module_name,
                        name,
                        we::EntityType::$ll_import_type($hl_to_ll_closure(elem)),
                    );
                }
            }
        };
    }

    add_imports!(functions, insert_function_idx, Function, |f: &Function| {
        state.get_or_insert_type(f.type_).to_u32()
    });
    add_imports!(tables, insert_table_idx, Table, |t: &Table| {
        we::TableType::from(t.limits)
    });
    add_imports!(memories, insert_memory_idx, Memory, |m: &Memory| {
        we::MemoryType::from(m.limits)
    });
    add_imports!(globals, insert_global_idx, Global, |g: &Global| {
        we::GlobalType::from(g.type_)
    });

    import_section
}

fn encode_exports(
    module: &Module,
    state: &mut EncodeState,
) -> Result<we::ExportSection, EncodeError> {
    let mut export_section = we::ExportSection::new();

    macro_rules! add_exports {
        ($elem_iter: ident, $export_kind: ident, $map_idx_fn: ident) => {
            for (hl_idx, elem) in module.$elem_iter() {
                for name in &elem.export {
                    export_section.export(
                        name,
                        we::ExportKind::$export_kind,
                        state.$map_idx_fn(hl_idx)?.to_u32(),
                    );
                }
            }
        };
    }

    add_exports!(functions, Func, map_function_idx);
    add_exports!(tables, Table, map_table_idx);
    add_exports!(memories, Memory, map_memory_idx);
    add_exports!(globals, Global, map_global_idx);

    Ok(export_section)
}

/// Encode the types in the order in that we gave indices to them.
fn encode_types(state: &EncodeState) -> we::TypeSection {
    let mut type_section = we::TypeSection::new();

    let types_idx = state.types_idx.read().unwrap();

    let mut types_ordered: Vec<(&FunctionType, Idx<marker::we::FunctionType>)> =
        types_idx.iter().map(|(t, i)| (t, *i)).collect();
    types_ordered.sort_unstable_by_key(|&(_, idx)| idx);
    assert_eq!(
        types_idx.len(),
        types_ordered
            .last()
            .map(|(_, idx)| idx.to_usize() + 1)
            .unwrap_or(0),
        "type index space should not have any holes, mapping: {types_idx:?}"
    );
    for (type_, _) in types_ordered {
        type_section.function(
            type_.inputs().iter().copied().map(we::ValType::from),
            type_.results().iter().copied().map(we::ValType::from),
        );
    }
    type_section
}

fn encode_functions(module: &Module, state: &mut EncodeState) -> we::FunctionSection {
    let mut function_section = we::FunctionSection::new();

    for (function_idx, function) in module.functions() {
        if let Some(_code) = function.code() {
            state.insert_function_idx(function_idx);

            let ll_type_idx = state.get_or_insert_type(function.type_);
            function_section.function(ll_type_idx.to_u32());
        }
    }

    function_section
}

fn encode_tables(
    module: &Module,
    state: &mut EncodeState,
) -> Result<(we::TableSection, we::ElementSection), EncodeError> {
    let mut table_section = we::TableSection::new();
    let mut element_section = we::ElementSection::new();

    for (hl_table_idx, table) in module.tables() {
        let ll_table_idx = if table.import.is_none() {
            table_section.table(we::TableType::from(table.limits));
            state.insert_table_idx(hl_table_idx)
        } else {
            state.map_table_idx(hl_table_idx)?
        };

        for hl_element in &table.elements {
            // `wasm-encoder` uses None as the table index to signify the MVP binary format.
            // Use that whenever possible, to avoid producing a binary using extensions.
            let ll_table_idx = if ll_table_idx.to_u32() == 0 {
                None
            } else {
                Some(ll_table_idx.to_u32())
            };
            let ll_offset = encode_single_instruction_with_end(&hl_element.offset, state)?;
            let ll_elements = hl_element
                .functions
                .iter()
                .map(|function_idx| state.map_function_idx(*function_idx).map(Idx::to_u32))
                .collect::<Result<Vec<u32>, _>>()?;
            let ll_elements = we::Elements::Functions(ll_elements.as_slice());
            element_section.active(ll_table_idx, &ll_offset, we::ValType::FuncRef, ll_elements);
        }
    }

    Ok((table_section, element_section))
}

fn encode_memories(
    module: &Module,
    state: &mut EncodeState,
) -> Result<(we::MemorySection, we::DataSection), EncodeError> {
    let mut memory_section = we::MemorySection::new();
    let mut data_section = we::DataSection::new();

    for (hl_memory_idx, memory) in module.memories() {
        let ll_memory_idx = if memory.import.is_none() {
            memory_section.memory(we::MemoryType::from(memory.limits));
            state.insert_memory_idx(hl_memory_idx)
        } else {
            state.map_memory_idx(hl_memory_idx)?
        };

        for data in &memory.data {
            let ll_offset = encode_single_instruction_with_end(&data.offset, state)?;
            let ll_data = data.bytes.iter().copied();
            data_section.active(ll_memory_idx.to_u32(), &ll_offset, ll_data);
        }
    }

    Ok((memory_section, data_section))
}

fn encode_globals(
    module: &Module,
    state: &mut EncodeState,
) -> Result<we::GlobalSection, EncodeError> {
    let mut global_section = we::GlobalSection::new();

    for (global_idx, global) in module.globals() {
        if let Some(init) = global.init() {
            state.insert_global_idx(global_idx);
            let ll_init = encode_single_instruction_with_end(init, state)?;
            global_section.global(we::GlobalType::from(global.type_), &ll_init);
        }
    }

    Ok(global_section)
}

fn encode_code(module: &Module, state: &mut EncodeState) -> Result<we::CodeSection, EncodeError> {
    let mut code_section = we::CodeSection::new();

    // Encode function bodies in parallel.
    let ll_functions = module
        .functions
        .par_iter()
        .filter_map(Function::code)
        .map(|code| -> Result<we::Function, EncodeError> {
            let ll_locals_iter = code
                .locals
                .iter()
                .map(|local| we::ValType::from(local.type_));
            let mut ll_function = we::Function::new_with_locals_types(ll_locals_iter);
            for instr in &code.body {
                ll_function.instruction(&encode_instruction(instr, state)?);
            }
            Ok(ll_function)
        })
        .collect::<Result<Vec<we::Function>, _>>()?;
    for ll_function in ll_functions {
        code_section.function(&ll_function);
    }

    Ok(code_section)
}

// TODO generify to include all sections, not just custom sections
// fn insert_section<T>(encoder: &mut wasm_encoder::Module, state: &mut EncodeState, section: T, module: &Module, previous_section: Option<SectionId>)
//     where T: wasm_encoder::Section {
fn encode_and_insert_custom(
    encoder: &mut wasm_encoder::Module,
    state: &mut EncodeState,
    module: &Module,
) {
    for custom in module
        .custom_sections
        .iter()
        .skip(state.custom_sections_encoded)
    {
        // FIXME what if the reference .after section is no longer present?
        // Right now, this would drop the custom section.
        if state.last_encoded_section == custom.previous_section {
            encoder.section(&wasm_encoder::CustomSection {
                name: &custom.name,
                data: &custom.content[..],
            });
            state.custom_sections_encoded += 1;
            state.last_encoded_section = Some(SectionId::Custom(custom.name.clone()));
        }
    }
}

fn encode_single_instruction_with_end(
    instrs: &[Instr],
    state: &mut EncodeState,
) -> Result<we::ConstExpr, EncodeError> {
    match instrs {
        [single_instr, Instr::End] => {
            let mut instr_bytes = Vec::with_capacity(8);
            encode_instruction(single_instr, state)?.encode(&mut instr_bytes);
            Ok(we::ConstExpr::raw(instr_bytes))
        },
        _ => Err(EncodeError::message(format!("expected exactly one instruction, followed by an end, but got {instrs:?}. If there is more than one instruction, this is not supported by wasm-encoder for an unknown reason."))),
    }
}

fn encode_instruction(
    hl_instr: &Instr,
    state: &EncodeState,
) -> Result<we::Instruction<'static>, EncodeError> {
    Ok(match *hl_instr {
        Instr::Unreachable => we::Instruction::Unreachable,
        Instr::Nop => we::Instruction::Nop,

        Instr::Block(block_type) => we::Instruction::Block(encode_block_type(block_type, state)),
        Instr::Loop(block_type) => we::Instruction::Loop(encode_block_type(block_type, state)),
        Instr::If(block_type) => we::Instruction::If(encode_block_type(block_type, state)),
        Instr::Else => we::Instruction::Else,
        Instr::End => we::Instruction::End,

        Instr::Br(label) => we::Instruction::Br(label.to_u32()),
        Instr::BrIf(label) => we::Instruction::BrIf(label.to_u32()),
        Instr::BrTable { ref table, default } => we::Instruction::BrTable(
            table.iter().map(|label| label.to_u32()).collect(),
            default.to_u32(),
        ),

        Instr::Return => we::Instruction::Return,
        Instr::Call(function_idx) => {
            we::Instruction::Call(state.map_function_idx(function_idx)?.to_u32())
        }
        Instr::CallIndirect(ref function_type, table_idx) => we::Instruction::CallIndirect {
            ty: state.get_or_insert_type(*function_type).to_u32(),
            table: state.map_table_idx(table_idx)?.to_u32(),
        },

        Instr::Drop => we::Instruction::Drop,
        Instr::Select => we::Instruction::Select,

        Instr::Local(LocalOp::Get, local_idx) => we::Instruction::LocalGet(local_idx.to_u32()),
        Instr::Local(LocalOp::Set, local_idx) => we::Instruction::LocalSet(local_idx.to_u32()),
        Instr::Local(LocalOp::Tee, local_idx) => we::Instruction::LocalTee(local_idx.to_u32()),
        Instr::Global(GlobalOp::Get, global_idx) => {
            we::Instruction::GlobalGet(state.map_global_idx(global_idx)?.to_u32())
        }
        Instr::Global(GlobalOp::Set, global_idx) => {
            we::Instruction::GlobalSet(state.map_global_idx(global_idx)?.to_u32())
        }

        Instr::Load(LoadOp::I32Load, memarg) => we::Instruction::I32Load(memarg.into()),
        Instr::Load(LoadOp::I64Load, memarg) => we::Instruction::I64Load(memarg.into()),
        Instr::Load(LoadOp::F32Load, memarg) => we::Instruction::F32Load(memarg.into()),
        Instr::Load(LoadOp::F64Load, memarg) => we::Instruction::F64Load(memarg.into()),
        Instr::Load(LoadOp::I32Load8S, memarg) => we::Instruction::I32Load8S(memarg.into()),
        Instr::Load(LoadOp::I32Load8U, memarg) => we::Instruction::I32Load8U(memarg.into()),
        Instr::Load(LoadOp::I32Load16S, memarg) => we::Instruction::I32Load16S(memarg.into()),
        Instr::Load(LoadOp::I32Load16U, memarg) => we::Instruction::I32Load16U(memarg.into()),
        Instr::Load(LoadOp::I64Load8S, memarg) => we::Instruction::I64Load8S(memarg.into()),
        Instr::Load(LoadOp::I64Load8U, memarg) => we::Instruction::I64Load8U(memarg.into()),
        Instr::Load(LoadOp::I64Load16S, memarg) => we::Instruction::I64Load16S(memarg.into()),
        Instr::Load(LoadOp::I64Load16U, memarg) => we::Instruction::I64Load16U(memarg.into()),
        Instr::Load(LoadOp::I64Load32S, memarg) => we::Instruction::I64Load32S(memarg.into()),
        Instr::Load(LoadOp::I64Load32U, memarg) => we::Instruction::I64Load32U(memarg.into()),

        Instr::Store(StoreOp::I32Store, memarg) => we::Instruction::I32Store(memarg.into()),
        Instr::Store(StoreOp::I64Store, memarg) => we::Instruction::I64Store(memarg.into()),
        Instr::Store(StoreOp::F32Store, memarg) => we::Instruction::F32Store(memarg.into()),
        Instr::Store(StoreOp::F64Store, memarg) => we::Instruction::F64Store(memarg.into()),
        Instr::Store(StoreOp::I32Store8, memarg) => we::Instruction::I32Store8(memarg.into()),
        Instr::Store(StoreOp::I32Store16, memarg) => we::Instruction::I32Store16(memarg.into()),
        Instr::Store(StoreOp::I64Store8, memarg) => we::Instruction::I64Store8(memarg.into()),
        Instr::Store(StoreOp::I64Store16, memarg) => we::Instruction::I64Store16(memarg.into()),
        Instr::Store(StoreOp::I64Store32, memarg) => we::Instruction::I64Store32(memarg.into()),

        Instr::MemorySize(memory_idx) => {
            we::Instruction::MemorySize(state.map_memory_idx(memory_idx)?.to_u32())
        }
        Instr::MemoryGrow(memory_idx) => {
            we::Instruction::MemoryGrow(state.map_memory_idx(memory_idx)?.to_u32())
        }

        Instr::Const(Val::I32(value)) => we::Instruction::I32Const(value),
        Instr::Const(Val::I64(value)) => we::Instruction::I64Const(value),
        Instr::Const(Val::F32(value)) => we::Instruction::F32Const(value.into_inner()),
        Instr::Const(Val::F64(value)) => we::Instruction::F64Const(value.into_inner()),

        Instr::Unary(UnaryOp::I32Eqz) => we::Instruction::I32Eqz,
        Instr::Unary(UnaryOp::I64Eqz) => we::Instruction::I64Eqz,
        Instr::Unary(UnaryOp::I32Clz) => we::Instruction::I32Clz,
        Instr::Unary(UnaryOp::I32Ctz) => we::Instruction::I32Ctz,
        Instr::Unary(UnaryOp::I32Popcnt) => we::Instruction::I32Popcnt,
        Instr::Unary(UnaryOp::I64Clz) => we::Instruction::I64Clz,
        Instr::Unary(UnaryOp::I64Ctz) => we::Instruction::I64Ctz,
        Instr::Unary(UnaryOp::I64Popcnt) => we::Instruction::I64Popcnt,
        Instr::Unary(UnaryOp::F32Abs) => we::Instruction::F32Abs,
        Instr::Unary(UnaryOp::F32Neg) => we::Instruction::F32Neg,
        Instr::Unary(UnaryOp::F32Ceil) => we::Instruction::F32Ceil,
        Instr::Unary(UnaryOp::F32Floor) => we::Instruction::F32Floor,
        Instr::Unary(UnaryOp::F32Trunc) => we::Instruction::F32Trunc,
        Instr::Unary(UnaryOp::F32Nearest) => we::Instruction::F32Nearest,
        Instr::Unary(UnaryOp::F32Sqrt) => we::Instruction::F32Sqrt,
        Instr::Unary(UnaryOp::F64Abs) => we::Instruction::F64Abs,
        Instr::Unary(UnaryOp::F64Neg) => we::Instruction::F64Neg,
        Instr::Unary(UnaryOp::F64Ceil) => we::Instruction::F64Ceil,
        Instr::Unary(UnaryOp::F64Floor) => we::Instruction::F64Floor,
        Instr::Unary(UnaryOp::F64Trunc) => we::Instruction::F64Trunc,
        Instr::Unary(UnaryOp::F64Nearest) => we::Instruction::F64Nearest,
        Instr::Unary(UnaryOp::F64Sqrt) => we::Instruction::F64Sqrt,
        Instr::Unary(UnaryOp::I32WrapI64) => we::Instruction::I32WrapI64,
        Instr::Unary(UnaryOp::I32TruncF32S) => we::Instruction::I32TruncF32S,
        Instr::Unary(UnaryOp::I32TruncF32U) => we::Instruction::I32TruncF32U,
        Instr::Unary(UnaryOp::I32TruncF64S) => we::Instruction::I32TruncF64S,
        Instr::Unary(UnaryOp::I32TruncF64U) => we::Instruction::I32TruncF64U,
        Instr::Unary(UnaryOp::I64ExtendI32S) => we::Instruction::I64ExtendI32S,
        Instr::Unary(UnaryOp::I64ExtendI32U) => we::Instruction::I64ExtendI32U,
        Instr::Unary(UnaryOp::I64TruncF32S) => we::Instruction::I64TruncF32S,
        Instr::Unary(UnaryOp::I64TruncF32U) => we::Instruction::I64TruncF32U,
        Instr::Unary(UnaryOp::I64TruncF64S) => we::Instruction::I64TruncF64S,
        Instr::Unary(UnaryOp::I64TruncF64U) => we::Instruction::I64TruncF64U,
        Instr::Unary(UnaryOp::F32ConvertI32S) => we::Instruction::F32ConvertI32S,
        Instr::Unary(UnaryOp::F32ConvertI32U) => we::Instruction::F32ConvertI32U,
        Instr::Unary(UnaryOp::F32ConvertI64S) => we::Instruction::F32ConvertI64S,
        Instr::Unary(UnaryOp::F32ConvertI64U) => we::Instruction::F32ConvertI64U,
        Instr::Unary(UnaryOp::F32DemoteF64) => we::Instruction::F32DemoteF64,
        Instr::Unary(UnaryOp::F64ConvertI32S) => we::Instruction::F64ConvertI32S,
        Instr::Unary(UnaryOp::F64ConvertI32U) => we::Instruction::F64ConvertI32U,
        Instr::Unary(UnaryOp::F64ConvertI64S) => we::Instruction::F64ConvertI64S,
        Instr::Unary(UnaryOp::F64ConvertI64U) => we::Instruction::F64ConvertI64U,
        Instr::Unary(UnaryOp::F64PromoteF32) => we::Instruction::F64PromoteF32,
        Instr::Unary(UnaryOp::I32ReinterpretF32) => we::Instruction::I32ReinterpretF32,
        Instr::Unary(UnaryOp::I64ReinterpretF64) => we::Instruction::I64ReinterpretF64,
        Instr::Unary(UnaryOp::F32ReinterpretI32) => we::Instruction::F32ReinterpretI32,
        Instr::Unary(UnaryOp::F64ReinterpretI64) => we::Instruction::F64ReinterpretI64,
        Instr::Unary(UnaryOp::I32Extend8S) => we::Instruction::I32Extend8S,
        Instr::Unary(UnaryOp::I32Extend16S) => we::Instruction::I32Extend16S,
        Instr::Unary(UnaryOp::I64Extend8S) => we::Instruction::I64Extend8S,
        Instr::Unary(UnaryOp::I64Extend16S) => we::Instruction::I64Extend16S,
        Instr::Unary(UnaryOp::I64Extend32S) => we::Instruction::I64Extend32S,

        Instr::Binary(BinaryOp::I32Eq) => we::Instruction::I32Eq,
        Instr::Binary(BinaryOp::I32Ne) => we::Instruction::I32Ne,
        Instr::Binary(BinaryOp::I32LtS) => we::Instruction::I32LtS,
        Instr::Binary(BinaryOp::I32LtU) => we::Instruction::I32LtU,
        Instr::Binary(BinaryOp::I32GtS) => we::Instruction::I32GtS,
        Instr::Binary(BinaryOp::I32GtU) => we::Instruction::I32GtU,
        Instr::Binary(BinaryOp::I32LeS) => we::Instruction::I32LeS,
        Instr::Binary(BinaryOp::I32LeU) => we::Instruction::I32LeU,
        Instr::Binary(BinaryOp::I32GeS) => we::Instruction::I32GeS,
        Instr::Binary(BinaryOp::I32GeU) => we::Instruction::I32GeU,
        Instr::Binary(BinaryOp::I64Eq) => we::Instruction::I64Eq,
        Instr::Binary(BinaryOp::I64Ne) => we::Instruction::I64Ne,
        Instr::Binary(BinaryOp::I64LtS) => we::Instruction::I64LtS,
        Instr::Binary(BinaryOp::I64LtU) => we::Instruction::I64LtU,
        Instr::Binary(BinaryOp::I64GtS) => we::Instruction::I64GtS,
        Instr::Binary(BinaryOp::I64GtU) => we::Instruction::I64GtU,
        Instr::Binary(BinaryOp::I64LeS) => we::Instruction::I64LeS,
        Instr::Binary(BinaryOp::I64LeU) => we::Instruction::I64LeU,
        Instr::Binary(BinaryOp::I64GeS) => we::Instruction::I64GeS,
        Instr::Binary(BinaryOp::I64GeU) => we::Instruction::I64GeU,
        Instr::Binary(BinaryOp::F32Eq) => we::Instruction::F32Eq,
        Instr::Binary(BinaryOp::F32Ne) => we::Instruction::F32Ne,
        Instr::Binary(BinaryOp::F32Lt) => we::Instruction::F32Lt,
        Instr::Binary(BinaryOp::F32Gt) => we::Instruction::F32Gt,
        Instr::Binary(BinaryOp::F32Le) => we::Instruction::F32Le,
        Instr::Binary(BinaryOp::F32Ge) => we::Instruction::F32Ge,
        Instr::Binary(BinaryOp::F64Eq) => we::Instruction::F64Eq,
        Instr::Binary(BinaryOp::F64Ne) => we::Instruction::F64Ne,
        Instr::Binary(BinaryOp::F64Lt) => we::Instruction::F64Lt,
        Instr::Binary(BinaryOp::F64Gt) => we::Instruction::F64Gt,
        Instr::Binary(BinaryOp::F64Le) => we::Instruction::F64Le,
        Instr::Binary(BinaryOp::F64Ge) => we::Instruction::F64Ge,
        Instr::Binary(BinaryOp::I32Add) => we::Instruction::I32Add,
        Instr::Binary(BinaryOp::I32Sub) => we::Instruction::I32Sub,
        Instr::Binary(BinaryOp::I32Mul) => we::Instruction::I32Mul,
        Instr::Binary(BinaryOp::I32DivS) => we::Instruction::I32DivS,
        Instr::Binary(BinaryOp::I32DivU) => we::Instruction::I32DivU,
        Instr::Binary(BinaryOp::I32RemS) => we::Instruction::I32RemS,
        Instr::Binary(BinaryOp::I32RemU) => we::Instruction::I32RemU,
        Instr::Binary(BinaryOp::I32And) => we::Instruction::I32And,
        Instr::Binary(BinaryOp::I32Or) => we::Instruction::I32Or,
        Instr::Binary(BinaryOp::I32Xor) => we::Instruction::I32Xor,
        Instr::Binary(BinaryOp::I32Shl) => we::Instruction::I32Shl,
        Instr::Binary(BinaryOp::I32ShrS) => we::Instruction::I32ShrS,
        Instr::Binary(BinaryOp::I32ShrU) => we::Instruction::I32ShrU,
        Instr::Binary(BinaryOp::I32Rotl) => we::Instruction::I32Rotl,
        Instr::Binary(BinaryOp::I32Rotr) => we::Instruction::I32Rotr,
        Instr::Binary(BinaryOp::I64Add) => we::Instruction::I64Add,
        Instr::Binary(BinaryOp::I64Sub) => we::Instruction::I64Sub,
        Instr::Binary(BinaryOp::I64Mul) => we::Instruction::I64Mul,
        Instr::Binary(BinaryOp::I64DivS) => we::Instruction::I64DivS,
        Instr::Binary(BinaryOp::I64DivU) => we::Instruction::I64DivU,
        Instr::Binary(BinaryOp::I64RemS) => we::Instruction::I64RemS,
        Instr::Binary(BinaryOp::I64RemU) => we::Instruction::I64RemU,
        Instr::Binary(BinaryOp::I64And) => we::Instruction::I64And,
        Instr::Binary(BinaryOp::I64Or) => we::Instruction::I64Or,
        Instr::Binary(BinaryOp::I64Xor) => we::Instruction::I64Xor,
        Instr::Binary(BinaryOp::I64Shl) => we::Instruction::I64Shl,
        Instr::Binary(BinaryOp::I64ShrS) => we::Instruction::I64ShrS,
        Instr::Binary(BinaryOp::I64ShrU) => we::Instruction::I64ShrU,
        Instr::Binary(BinaryOp::I64Rotl) => we::Instruction::I64Rotl,
        Instr::Binary(BinaryOp::I64Rotr) => we::Instruction::I64Rotr,
        Instr::Binary(BinaryOp::F32Add) => we::Instruction::F32Add,
        Instr::Binary(BinaryOp::F32Sub) => we::Instruction::F32Sub,
        Instr::Binary(BinaryOp::F32Mul) => we::Instruction::F32Mul,
        Instr::Binary(BinaryOp::F32Div) => we::Instruction::F32Div,
        Instr::Binary(BinaryOp::F32Min) => we::Instruction::F32Min,
        Instr::Binary(BinaryOp::F32Max) => we::Instruction::F32Max,
        Instr::Binary(BinaryOp::F32Copysign) => we::Instruction::F32Copysign,
        Instr::Binary(BinaryOp::F64Add) => we::Instruction::F64Add,
        Instr::Binary(BinaryOp::F64Sub) => we::Instruction::F64Sub,
        Instr::Binary(BinaryOp::F64Mul) => we::Instruction::F64Mul,
        Instr::Binary(BinaryOp::F64Div) => we::Instruction::F64Div,
        Instr::Binary(BinaryOp::F64Min) => we::Instruction::F64Min,
        Instr::Binary(BinaryOp::F64Max) => we::Instruction::F64Max,
        Instr::Binary(BinaryOp::F64Copysign) => we::Instruction::F64Copysign,
    })
}

fn encode_names(
    module: &Module,
    state: &EncodeState,
) -> Result<Option<we::NameSection>, EncodeError> {
    // Because `wasm-encode`s name section and name subsections don't track whether they are
    // empty, and we don't want to write empty sections, wrap them in an `Option` here and
    // lazily initialize on access. Then, write them only if they are not `None`.
    let mut functions_subsection: Option<we::NameMap> = None;
    let mut locals_subsection: Option<we::IndirectNameMap> = None;
    for (hl_function_idx, function) in module.functions() {
        let ll_function_idx = state.map_function_idx(hl_function_idx)?.to_u32();

        if let Some(name) = &function.name {
            functions_subsection
                .get_or_insert_with(Default::default)
                .append(ll_function_idx, name);
        }

        let mut local_names: Option<we::NameMap> = None;
        for (local_idx, local) in function.param_or_locals() {
            if let Some(name) = local.name() {
                local_names
                    .get_or_insert_with(Default::default)
                    .append(local_idx.to_u32(), name);
            }
        }
        if let Some(local_names) = local_names {
            locals_subsection
                .get_or_insert_with(Default::default)
                .append(ll_function_idx, &local_names);
        }
    }

    let mut name_section: Option<we::NameSection> = None;
    if let Some(module_name) = &module.name {
        name_section
            .get_or_insert_with(Default::default)
            .module(module_name);
    }
    if let Some(functions_subsection) = &functions_subsection {
        name_section
            .get_or_insert_with(Default::default)
            .functions(functions_subsection);
    }
    if let Some(locals_subsection) = &locals_subsection {
        name_section
            .get_or_insert_with(Default::default)
            .locals(locals_subsection);
    }

    Ok(name_section)
}

fn encode_block_type(func_or_block_ty: FunctionType, state: &EncodeState) -> we::BlockType {
    match (func_or_block_ty.inputs(), func_or_block_ty.results()) {
        // Prefer the more compact inline encoding for Wasm MVP block types.
        ([], []) => we::BlockType::Empty,
        ([], [val_type]) => we::BlockType::Result((*val_type).into()),
        // Only fall back to a reference to a function type if necessary.
        (_inputs, _results) => {
            we::BlockType::FunctionType(state.get_or_insert_type(func_or_block_ty).to_u32())
        }
    }
}

impl From<GlobalType> for we::GlobalType {
    fn from(hl_global_type: GlobalType) -> Self {
        Self {
            val_type: hl_global_type.0.into(),
            mutable: match hl_global_type.1 {
                Mutability::Const => false,
                Mutability::Mut => true,
            },
        }
    }
}

impl From<Limits> for we::TableType {
    fn from(limits: Limits) -> Self {
        Self {
            element_type: we::ValType::FuncRef,
            minimum: limits.initial_size,
            maximum: limits.max_size,
        }
    }
}

impl From<Limits> for we::MemoryType {
    fn from(limits: Limits) -> Self {
        Self {
            minimum: limits.initial_size.into(),
            maximum: limits.max_size.map(|u32| u32.into()),
            memory64: false,
            shared: false,
        }
    }
}

impl From<ValType> for we::ValType {
    fn from(hl_val_type: ValType) -> Self {
        use ValType::*;
        match hl_val_type {
            I32 => we::ValType::I32,
            I64 => we::ValType::I64,
            F32 => we::ValType::F32,
            F64 => we::ValType::F64,
        }
    }
}

impl From<Memarg> for we::MemArg {
    fn from(hl_memarg: Memarg) -> Self {
        Self {
            offset: hl_memarg.offset.into(),
            align: hl_memarg.alignment_exp.into(),
            memory_index: 0,
        }
    }
}
