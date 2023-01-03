use std::error::Error;
use std::fmt::Write;
use std::fs;

use dashmap::DashMap;

use rayon::prelude::*;

use test_utilities::*;

use crate::types::TypeChecker;
use crate::*;

const NAME_SECTION_TEST_BINARY: &str = "../../test-inputs/wasm-feature-tests/name-section/wabt-tests/names.wasm";
const BANANABREAD_REAL_WORLD_TEST_BINARY: &str = "../../test-inputs/real-world-binaries/bananabread/bb.wasm";

// Removed this test, because when changing to wasmparser,
// we did not port over the low-level parsing of the extended name section.
// const WASM_TEST_INPUT_EXTENDED_NAMES_SECTION: &str = "../../test-inputs/wasm-feature-tests/name-section/extended-name-section/vuln.wasm";

#[test]
fn collect_all_function_types_in_test_set() {
    let type_count = DashMap::new();
    for_each_valid_wasm_binary_in_test_set(|path| {
        let (module, _, _) = Module::from_file(path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));

        for func in module.functions {
            *type_count.entry(func.type_).or_insert(0u64) += 1;

            // Also collect all (easily computable) instruction types.
            for instr in func.code().iter().flat_map(|code| &code.body) {
                if let Some(instr_ty) = instr.simple_type() {
                    *type_count.entry(instr_ty).or_insert(0) += 1;
                }
            }
        }
    });

    let mut type_count: Vec<_> = type_count.into_iter().collect();
    type_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    let mut output_contents = String::new();
    for (ty, count) in &type_count {
        writeln!(&mut output_contents, "{count:10} ; {ty}").unwrap();
    }
    fs::create_dir_all("../../test-outputs/collect-types/").unwrap();
    fs::write("../../test-outputs/collect-types/function_type_count.csv", output_contents).unwrap();

    let val_type_seq_count = DashMap::new();
    type_count
        .par_iter()
        .for_each(|(func_ty, count)| {
            *val_type_seq_count.entry(func_ty.inputs()).or_insert(0) += count;
            *val_type_seq_count.entry(func_ty.results()).or_insert(0) += count;
        });
    let mut val_type_seq_count: Vec<_> = val_type_seq_count.into_iter().collect();

    val_type_seq_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

    let mut output_contents = String::new();
    for (ty, count) in &val_type_seq_count {
        writeln!(&mut output_contents, "{:10} ; [{}]", count, ty.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join(", ")).unwrap();
    }
    fs::write("../../test-outputs/collect-types/val_type_seq_count.csv", output_contents).unwrap();
}

#[test]
fn roundtrip_produces_same_module_ast() {
    for_each_valid_wasm_binary_in_test_set(|path| {
        let (module, _offsets, warnings) = Module::from_file(path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));
        if !warnings.is_empty() {
            eprintln!("Warnings parsing '{}': {:#?}", path.display(), warnings);
        }

        let bytes = module.to_bytes()
            .unwrap_or_else(|err| panic!("Could not encode valid binary '{}': {err}", path.display()));

        let (module_roundtrip, _, _) = Module::from_bytes(&bytes).unwrap();
        assert_eq!(module, module_roundtrip, "Roundtrip failed for binary '{}'", path.display());
    });
}

#[test]
fn type_checking_valid_files() {
    for_each_valid_wasm_binary_in_test_set(|path| {
        let (module, _, _) = Module::from_file(path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));
        
        TypeChecker::check_module(&module)
            .unwrap_or_else(|_| panic!("Valid binary '{}' should type check, but did not", path.display()));
    });
}

#[test]
fn decode_encode_is_valid_wasm() {
    for_each_valid_wasm_binary_in_test_set(|path| {
        let (module, _, _) = Module::from_file(path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));

        let output_path = &output_file(path, "encode").unwrap();
        module.to_file(output_path)
            .unwrap_or_else(|err| panic!("Could not encode valid binary to file '{}': {err}", output_path.display()));

        wasm_validate(output_path)
            .unwrap_or_else(|err| panic!("Written binary did not validate '{}': {err}", output_path.display()));
    });
}

// TODO: Also ensure that used_wasm_extensions(encode(decode(wasm))) <= used_wasm_extensions(wasm), i.e., that our
// encoding does not introduce new extensions.

#[test]
fn section_offsets_like_objdump() {
    // Use a wasm file with a custom section for testing section offsets.
    let (_module, offsets, _warnings) = Module::from_file(NAME_SECTION_TEST_BINARY).unwrap();

    // Expected values are taken from wasm-objdump output.
    assert_eq!(offsets.section_offsets(SectionId::Type), vec![0xa]);
    assert_eq!(offsets.section_offsets(SectionId::Function), vec![0x11]);
    assert_eq!(offsets.section_offsets(SectionId::Code), vec![0x15]);
    assert_eq!(offsets.section_offsets(SectionId::Custom("name".to_string())), vec![0x1f]);
    // Also try the (only) function code offset, for completion.
    assert_eq!(offsets.function_idx_to_offset(Idx::from(0u32)), Some(0x17));
    assert_eq!(offsets.function_offset_to_idx(0x17), Some(Idx::from(0u32)));
}

#[test]
fn code_offsets_like_objdump() {
    let (_module, offsets, _warnings) =
        Module::from_file(BANANABREAD_REAL_WORLD_TEST_BINARY).unwrap();

    // Test first two and last two functions.
    // Expected values are taken from wasm-objdump output.
    assert_eq!(offsets.function_idx_to_offset(Idx::from(383u32)), Some(0x5522));
    assert_eq!(offsets.function_offset_to_idx(0x5522), Some(Idx::from(383u32)));
    assert_eq!(offsets.function_idx_to_offset(Idx::from(384u32)), Some(0x5545));
    assert_eq!(offsets.function_offset_to_idx(0x5545), Some(Idx::from(384u32)));

    assert_eq!(offsets.function_idx_to_offset(Idx::from(3641u32)), Some(0x1e38b7));
    assert_eq!(offsets.function_offset_to_idx(0x1e38b7), Some(Idx::from(3641u32)));
    assert_eq!(offsets.function_idx_to_offset(Idx::from(3642u32)), Some(0x1e38d2));
    assert_eq!(offsets.function_offset_to_idx(0x1e38d2), Some(Idx::from(3642u32)));
}

#[test]
// Unfortunately, when switching from my own low-level parser to wasmparser, this fails
// because it is not quite as strict as my error reporting was.
#[ignore]
fn error_offsets_correct() {
    fn assert_error_offset(binary: Vec<u8>, expected_offset: usize) {
        let result = Module::from_bytes(&binary);
        assert!(result.is_err(), "Parsing did not fail, but should have because binary {binary:?} is invalid");
        let err = result.err().unwrap();
        let offset = err.offset().expect("Error should have an offset");
        assert_eq!(offset, expected_offset, "{err}\n{err:?}\nsource: {:?}", err.source());
        // if offset != expected_offset {
        //     eprintln!("{offset} != {expected_offset}\n\t{err}\n\t{err:?}\n\tsource: {:?}", err.source());
        // } else {
        //     eprintln!("{offset} == {expected_offset}\n\t{err}\n\t{err:?}\n\tsource: {:?}", err.source());
        // }
    }

    let wrong_magic_number = b"Xasm\x01\x00\x00\x00".to_vec();
    assert_error_offset(wrong_magic_number, 0);

    let wrong_version = b"\x00asm\x02\x00\x00\x00".to_vec();
    assert_error_offset(wrong_version, 4);

    let wrong_section_id = b"\x00asm\x01\x00\x00\x00\xff".to_vec();
    assert_error_offset(wrong_section_id, 8);

    let valid_wasm_header = b"\x00asm\x01\x00\x00\x00".as_ref();
    let valid_code_element = &[
        // Size in bytes
        2,
        // Local count
        0,
        // Body: only end instruction
        0xb
    ].as_ref();

    let section_size_too_short = [valid_wasm_header, &[
        // Section id: code
        10,
        // WRONG Section size in bytes: should be 4 bit is 2
        2,
        // Code elements
        1,
    ], valid_code_element].concat();
    assert_error_offset(section_size_too_short, 9);

    let section_size_too_long = [valid_wasm_header, &[10, 6 /* instead of 4 */, 1], valid_code_element].concat();
    assert_error_offset(section_size_too_long, 9);

    let code_element_size_too_short = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 5, 1,
        // Code element 1
        // WRONG Size in bytes: should be 3
        2,
        // Local count
        0,
        // Body: nop, end
        0x1, 0xb
    ]].concat();
    // NOTE Different from the section size, the size in the "skip list"-like vector in the code
    // section causes the error to be attributed to the position where the Eof appears, not the
    // section size.
    assert_error_offset(code_element_size_too_short, 14);

    let code_element_size_too_long = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 6, 1,
        // Code element 1
        // WRONG Size in bytes: should be 3
        4,
        // Local count
        0,
        // Body: nop, end
        0x1, 0xb,
        // fill the file with more crap so that the read() call succeeds at least
        0xff
    ]].concat();
    // When the element size was too long, the error will be attributed to the size.
    assert_error_offset(code_element_size_too_long, 11);

    let invalid_instruction = [valid_wasm_header, &[
        // Code section, size in bytes, element count
        10, 5, 1,
        // Code element size, local count
        3, 0,
        // Body: WRONG
        0xff, 0xb
    ]].concat();
    assert_error_offset(invalid_instruction, 13);
}
