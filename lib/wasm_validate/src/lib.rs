use std::path::Path;
use std::process::Command;

/// utility function to call WABT's wasm-validate tool on a file (WABT needs to be on $PATH)
pub fn wasm_validate(path: impl AsRef<Path>) -> Result<(), String> {
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