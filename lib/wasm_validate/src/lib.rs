extern crate walkdir;

use std::path::{Path, PathBuf};

/// utility function to call WABT's wasm-validate tool on a file (WABT needs to be on $PATH)
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

/// utility function to return all wasm files under a root directory
pub fn wasm_files(root_dir: impl AsRef<Path>) -> impl Iterator<Item=PathBuf> {
    use walkdir::WalkDir;
    WalkDir::new(root_dir.as_ref()).into_iter()
        .map(Result::unwrap)
        .map(|entry| entry.path().to_owned())
        .filter(|path| path.extension() == Some("wasm".as_ref()))
}
