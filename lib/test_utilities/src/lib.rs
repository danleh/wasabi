//! utility functions for testing

extern crate walkdir;
use std::path::{Path, PathBuf};

/// call WABT's wasm-validate tool on a file (WABT needs to be on $PATH)
pub fn wasm_validate(path: impl AsRef<Path>) -> Result<(), String> {
    use std::process::Command;

    let path = path.as_ref();
    let validate_output = Command::new("wasm-validate")
        .arg(path)
        .output()
        .map_err(|err| err.to_string())
        .unwrap();

    if validate_output.status.success() {
        Ok(())
    } else {
        Err(format!("invalid wasm file {}\n{}",
                    path.display(),
                    String::from_utf8(validate_output.stderr).unwrap()))
    }
}

/// return all *.wasm files under a root directory
pub fn wasm_files(root_dir: impl AsRef<Path>) -> impl Iterator<Item=PathBuf> {
    use walkdir::WalkDir;

    let root_dir = root_dir.as_ref();
    if !root_dir.exists() {
        panic!("root_dir \"{}\" does not exist", root_dir.display());
    }
    WalkDir::new(root_dir).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}

pub fn output_file(test_input_file: impl AsRef<Path>, output_subdir: &'static str) -> PathBuf {
    use std::fs;

    let output_subdir = format!("outputs/{}/", output_subdir);
    let output_file = PathBuf::from(test_input_file.as_ref().to_str().unwrap()
        .replace("inputs/", &output_subdir));
    // ensure the directory exists
    fs::create_dir_all(output_file.parent().unwrap_or(&output_file)).unwrap();
    output_file
}