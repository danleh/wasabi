//! Utility functions for testing Wasabi and the Wasm library.

use std::io;
use std::path::Path;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use rayon::prelude::*;

const TEST_INPUTS_DIR: &str = "../../test-inputs/";
pub static ALL_VALID_TEST_BINARIES: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    println!("Collecting all valid .wasm binaries from '{}'...", TEST_INPUTS_DIR);
    let mut valid_binaries = wasm_files(TEST_INPUTS_DIR).unwrap();
    const EXCLUDED: &[&str] = &[
        // Known invalid files:
        "invalid",
        // The full set of WasmBench files is too large to run in CI.
        "all-binaries-metadata",
        "filtered-binaries-metadata",
        // Valid, but creates very large allocations because it has >500k locals in >1k functions.
        "31fa012442fd637fca221db4fda94262e99759ab9667147cbedde083aabcc065",
    ];
    for excluded in EXCLUDED.iter() {
        valid_binaries.retain(|path| !path.to_string_lossy().contains(excluded));
    }
    // Filter out files that are already invalid according to wasm-validate:
    let valid_binaries: Vec<_> = valid_binaries
        .into_par_iter()
        .filter(|path| wasm_validate(path).is_ok())
        .collect();
    println!("{} files validated.", valid_binaries.len());
    valid_binaries
});

/// Call WABT's wasm-validate tool on a file (WABT needs to be on $PATH).
pub fn wasm_validate(path: impl AsRef<Path>) -> Result<(), String> {
    use std::process::Command;

    let path = path.as_ref();
    let validate_output = Command::new("wasm-validate")
        .arg("--ignore-custom-section-errors")
        .arg(path)
        .output()
        .map_err(|err| err.to_string())?;

    if validate_output.status.success() {
        Ok(())
    } else {
        Err(format!("invalid wasm file {}\n{}",
                    path.display(),
                    String::from_utf8_lossy(&validate_output.stderr)))
    }
}

/// Return all *.wasm files recursively under a root directory, that do not have "out/" in their 
/// path (because out/ is the default instrumentation output directory of Wasabi).
pub fn wasm_files(root_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
    use walkdir::WalkDir;

    let mut wasm_files = Vec::new();
    for entry in WalkDir::new(&root_dir) {
        let path = entry.map_err(|err| err.to_string())?.path().to_owned();
        if let Some("wasm") = path.extension().and_then(|os_str| os_str.to_str()) {
            if std::fs::metadata(&path).map_err(|err| err.to_string())?.is_file() {
                if !path.components().flat_map(|comp| comp.as_os_str().to_str()).any(|dir| dir == "out") {
                    wasm_files.push(path);
                }
            }
        }
    }
    Ok(wasm_files)
}

/// Very ad-hoc utility function: map input .wasm file to file in output dir with custom 
/// subdirectory, e.g., bla.wasm + "transformXYZ" -> "outputs/transformXYZ/bla.wasm"
pub fn output_file(test_input_file: impl AsRef<Path>, output_subdir: &'static str) -> io::Result<PathBuf> {
    use std::fs;

    // Replace input path component with output + output subdirectory.
    let output_file = test_input_file.as_ref().iter()
        .flat_map(|component| {
            let component = component.to_str().unwrap();
            if component == "test-inputs" {
                vec!["test-outputs", output_subdir]
            } else {
                vec![component]
            }
        })
        .collect::<PathBuf>();
    
    assert_ne!(test_input_file.as_ref(), output_file);

    // Ensure the directory exists.
    fs::create_dir_all(output_file.parent().unwrap_or(&output_file))?;
    Ok(output_file)
}

#[test]
pub fn should_be_more_than_ten_valid_test_binaries() {
    assert!(ALL_VALID_TEST_BINARIES.len() > 10);
}