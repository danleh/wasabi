use std::error::Error;

use dashmap::DashMap;
use indicatif::{ProgressIterator, ParallelProgressIterator};

use rayon::prelude::*;

use test_utilities::*;

use crate::{*, types::TypeChecker};

const WASMBENCH_DIR: &str = "tests/WasmBench/valid-no-extensions";
const WASMBENCH_EXCLUDED_FILES: [&str; 3] = [
    // Valid, but creates very large allocations because it has >500k locals in >1k functions.
    "31fa012442fd637fca221db4fda94262e99759ab9667147cbedde083aabcc065",
    // Is actually invalid according to wasm-validate, not sure why it wasn't filtered out before.
    "4b1082f1c2d634aaebbe9b70331ac6639ab3fe7b0a52459ea4f6baa4f82a82ad",
    // Panics in the old parser, but works in the new one -> strictly better, so ignore.
    "a50d67cbaf770807cc1d1723ebc56333188b681538bf3f7679659b184d2f8020"
];

const WASM_TEST_INPUTS_DIR: &str = "../../tests/inputs";
const WASM_TEST_INPUT_LARGE: &str = "../../tests/inputs/real-world/bananabread/bb.wasm";
const WASM_TEST_INPUT_NAMES_SECTION: &str = "../../tests/inputs/name-section/wabt-tests/names.wasm";
// See below...
// const WASM_TEST_INPUT_EXTENDED_NAMES_SECTION: &str = "../../tests/inputs/name-section/extended-name-section/vuln.wasm";

#[test]
fn function_types_in_wasmbench() {
    let mut wasm_files = wasm_files(WASMBENCH_DIR).unwrap();
    for hash in WASMBENCH_EXCLUDED_FILES {
        wasm_files.retain(|path| !path.to_string_lossy().contains(hash));
    }

    let type_count = DashMap::new();

    wasm_files.par_iter().progress_count(wasm_files.len() as u64).for_each(|path| {
        let (module, _offsets, warnings) = Module::from_file(path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));
        if !warnings.is_empty() {
            eprintln!("Warnings parsing '{}': {:#?}", path.display(), warnings);
        }

        for func in module.functions {
            *type_count.entry(func.type_).or_insert(0) += 1;
            for instr in func.code().iter().flat_map(|code| &code.body) {
                if let Instr::CallIndirect(func_ty, ..) = instr {
                    *type_count.entry(*func_ty).or_insert(0u64) += 1;
                }
                if let Some(instr_ty) = instr.simple_type() {
                    *type_count.entry(instr_ty).or_insert(0) += 1;
                }
            }
        }
    });

    let mut type_count: Vec<_> = type_count.into_iter().collect();
    type_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    // Print the most common types first.
    // for (ty, count) in type_count.iter().take(1000) {
    //     println!("{:10} ; {}", count, ty);
    // }

    let val_type_seq_count = DashMap::new();
    type_count
        .par_iter()
        .for_each(|(func_ty, count)| {
            *val_type_seq_count.entry(func_ty.inputs()).or_insert(0u64) += count;
            *val_type_seq_count.entry(func_ty.results()).or_insert(0u64) += count;
        });
    let mut val_type_seq_count: Vec<_> = val_type_seq_count.into_iter().collect();
    val_type_seq_count.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    // Print the most common types first.
    let mut out = String::new();
    for (ty, count) in val_type_seq_count.iter().take(1000) {
        use std::fmt::Write;
        writeln!(&mut out, "{:10} ; [{}]", count, ty.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join(", ")).unwrap();
    }
    std::fs::write("val_type_seq_count.csv", out).unwrap();
    
}

#[test]
fn roundtrip_produces_same_module_ast() {
    let mut wasm_files = wasm_files(WASMBENCH_DIR).unwrap();
    for hash in WASMBENCH_EXCLUDED_FILES {
        wasm_files.retain(|path| !path.to_string_lossy().contains(hash));
    }

    wasm_files.iter().progress().for_each(|path| {
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
fn type_checking() {
    for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {
        println!("{}", path.display());
        let (module, _, _) = Module::from_file(&path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));
        TypeChecker::check_module(&module).expect("valid binary should type check");
    }
}

#[test]
fn decode_encode_is_valid_wasm() {
    for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {
        println!("{}", path.display());
        let (module, _, _) = Module::from_file(&path)
            .unwrap_or_else(|err| panic!("Could not parse valid binary '{}': {err}", path.display()));

        let output_path = &output_file(path, "encode").unwrap();
        module.to_file(output_path)
            .unwrap_or_else(|err| panic!("Could not encode valid binary to file '{}': {err}", output_path.display()));

        wasm_validate(output_path)
            .unwrap_or_else(|err| panic!("Written binary did not validate '{}': {err}", output_path.display()));
    }
}

#[test]
// Unfortunately, when switching from my own low-level parser to wasmparser, this fails
// because it is not quite as strict as my error reporting was.
#[ignore]
fn error_offsets_correct() {
    fn assert_error_offset(binary: Vec<u8>, expected_offset: usize) {
        let result = Module::from_bytes(&binary);
        assert!(result.is_err(), "Parsing did not fail, but should have because binary {:?} is invalid", binary);
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

#[test]
fn section_offsets_like_objdump() {
    // Use a wasm file with a custom section for testing section offsets.
    let (_module, offsets, _warnings) = Module::from_file(WASM_TEST_INPUT_NAMES_SECTION).unwrap();

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
    let (_module, offsets, _warnings) = Module::from_file(WASM_TEST_INPUT_LARGE).unwrap();

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
