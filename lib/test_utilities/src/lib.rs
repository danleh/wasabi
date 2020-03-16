//! Utility functions for testing Wasabi and the Wasm library.

use std::io;
use std::path::{Path, PathBuf};

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

/// Return all *.wasm files recursively under a root directory, that do not have "out/" in their path.
pub fn wasm_files(root_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
    use walkdir::WalkDir;

    let mut wasm_files = Vec::new();
    for entry in WalkDir::new(&root_dir) {
        let path = entry.map_err(|err| err.to_string())?.path().to_owned();
        // Only files with .wasm extension.
        if let Some("wasm") = path.extension().and_then(|os_str| os_str.to_str()) {
            // Only when not in "out/" directories, which are files already instrumented by Wasabi.
            if !path.components().flat_map(|comp| comp.as_os_str().to_str()).any(|dir| dir == "out") {
                wasm_files.push(path);
            }
        }
    }
    Ok(wasm_files)
}

/// Very ad-hoc utility function: map input .wasm file to file in output dir with custom subdirectory
/// e.g., bla.wasm + "transformXYZ" -> "outputs/transformXYZ/bla.wasm"
pub fn output_file(test_input_file: impl AsRef<Path>, output_subdir: &'static str) -> io::Result<PathBuf> {
    use std::fs;

    let output_subdir = format!("outputs/{}/", output_subdir);
    let output_file = PathBuf::from(test_input_file.as_ref().to_string_lossy()
        .replace("inputs/", &output_subdir));
    // Ensure the directory exists.
    fs::create_dir_all(output_file.parent().unwrap_or(&output_file))?;
    Ok(output_file)
}