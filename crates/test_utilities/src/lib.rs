//! Utility functions for testing Wasabi and the Wasm library.

use core::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::path::PathBuf;

use indicatif::ParallelProgressIterator;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use sysinfo::System;

const VALID_WASM_BINARIES_LIST_FILE: &str = "../../test-inputs/valid-wasm-binaries.txt";

static VALID_WASM_BINARIES: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    ValidInputsList::parse(VALID_WASM_BINARIES_LIST_FILE)
        .existing_files()
        .collect()
});

#[test]
pub fn should_be_more_than_ten_valid_test_binaries() {
    println!(
        "\n{} valid Wasm binaries in test set:",
        VALID_WASM_BINARIES.len()
    );
    for path in VALID_WASM_BINARIES.iter() {
        println!("  {}", path.display());
    }
    assert!(VALID_WASM_BINARIES.len() > 10);
}

/// Runs the test in parallel on all valid Wasm binaries in our test set,
/// and show a progress bar, since it might take long.
pub fn for_each_valid_wasm_binary_in_test_set(test_fn: impl Fn(&Path) + Send + Sync) {
    // Always make sure the progress bar is printed on a new line.
    eprintln!();

    VALID_WASM_BINARIES
        .par_iter()
        // Abort parallel processing as early as possible.
        .panic_fuse()
        .progress()
        .for_each(|path| {
            let module_size_bytes = fs::metadata(path)
                    .map(|file| file.len())
                    .unwrap_or(0);

            const AST_BYTES_PER_INSTRUCTION_BYTE_APPROX: u64 = 100;
            let memory_needed_for_ast_approx = module_size_bytes * AST_BYTES_PER_INSTRUCTION_BYTE_APPROX;

            let memory_available = {
                let mut system = System::new();
                system.refresh_memory();
                system.available_memory()
            };
            if memory_needed_for_ast_approx > memory_available {
                eprintln!("Skipping {} due to running low on memory...\n\t{:10} bytes memory available\n\t{:10} bytes module size\n\t{:10} bytes memory approximately required", path.display(), memory_available, module_size_bytes, memory_needed_for_ast_approx);
                return;
            }

            test_fn(path)
        });
}

#[derive(Debug)]
/// Information about the input file, in case there is an error during validation.
pub struct WasmFileInfo {
    pub path: PathBuf,
    pub file_size: Option<usize>,
    /// This is useful especially for WasmBench files, which are named by their SHA256 hash.
    pub sha256_digest: String,
}

impl WasmFileInfo {
    pub fn new(path: &Path) -> Self {
        let mut result = Self {
            path: path.to_owned(),
            file_size: None,
            sha256_digest: String::new(),
        };
        // Try to add file size and SHA hash, if available.
        if let Ok(bytes) = fs::read(path) {
            result.file_size = Some(bytes.len());
            result.sha256_digest = sha256::digest(bytes);
        }
        result
    }
}

impl fmt::Display for WasmFileInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}', ", self.path.display())?;
        if let Some(size) = self.file_size {
            write!(f, "size: {size} bytes, SHA256: {}", self.sha256_digest)
        } else {
            assert!(self.sha256_digest.is_empty());
            write!(f, "no size or SHA256 available, does the file exist?")
        }
    }
}

#[derive(Debug)]
pub enum WasmValidateError {
    InvalidWasmFile {
        input_file: WasmFileInfo,
        status_code: i32,
        stdout: String,
        stderr: String,
    },
    CouldNotValidate {
        input_file: WasmFileInfo,
        error: String,
    },
}

impl std::error::Error for WasmValidateError {}

impl fmt::Display for WasmValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WasmValidateError::InvalidWasmFile {
                input_file,
                status_code,
                stdout,
                stderr,
            } => {
                writeln!(f, "invalid Wasm file {input_file}")?;
                writeln!(f, "\twasm-validate status code: {status_code}")?;
                writeln!(f, "\tstdout: {stdout}")?;
                writeln!(f, "\tstderr: {stderr}")
            }
            WasmValidateError::CouldNotValidate { input_file, error } => {
                writeln!(f, "could not validate Wasm file {input_file}")?;
                writeln!(f, "possibly crash, error: {error}")
            }
        }
    }
}

/// Call WABT's wasm-validate tool on a file (WABT needs to be on $PATH).
pub fn wasm_validate(path: impl AsRef<Path>) -> Result<(), WasmValidateError> {
    use std::process::Command;

    let path = path.as_ref();
    let validate_output = Command::new("wasm-validate")
        .arg("--ignore-custom-section-errors")
        // Disable all extensions that we don't support yet.
        // .arg("--disable-saturating-float-to-int")
        // .arg("--disable-sign-extension")
        // .arg("--disable-simd")
        // .arg("--disable-multi-value")
        // .arg("--disable-bulk-memory")
        // .arg("--disable-reference-types")
        .arg(path)
        .output();

    match validate_output {
        Ok(validate_output) => match validate_output.status.code() {
            Some(0) => {
                assert!(validate_output.stdout.is_empty());
                assert!(validate_output.stderr.is_empty());
                Ok(())
            },
            Some(status_code) => Err(WasmValidateError::InvalidWasmFile {
                input_file: WasmFileInfo::new(path),
                status_code,
                stdout: String::from_utf8_lossy(&validate_output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&validate_output.stderr).to_string()
            }),
            None => Err(WasmValidateError::CouldNotValidate {
                input_file: WasmFileInfo::new(path),
                error: "wasm-validate terminated without a status code, on Linux this means it was terminated by a signal (possibly the OOM killer)".to_string()
            }),
        },
        Err(err) => Err(WasmValidateError::CouldNotValidate {
            input_file: WasmFileInfo::new(path),
            error: err.to_string()
        }),
    }
}

/// Ad-hoc utility function: map input .wasm file to file in output dir with custom
/// subdirectory, e.g., bla.wasm + "transformXYZ" -> "outputs/transformXYZ/bla.wasm"
pub fn output_file(
    test_input_file: impl AsRef<Path>,
    output_subdir: &'static str,
) -> io::Result<PathBuf> {
    use std::fs;

    // Replace input path component with output + output subdirectory.
    let output_file = test_input_file
        .as_ref()
        .iter()
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

#[derive(Debug)]
#[allow(dead_code)] // Used in test code.
struct ValidInputsList {
    list_file: PathBuf,
    base_dir: PathBuf,
    lines: Vec<ValidInputsLine>,
}

impl ValidInputsList {
    pub fn parse(list_file: impl AsRef<Path>) -> Self {
        let list_file = list_file.as_ref().to_owned();
        let base_dir = list_file
            .parent()
            .expect("list file has no parent directory")
            .to_owned();
        let lines = io::BufReader::new(File::open(&list_file).unwrap())
            .lines()
            .map(|line| {
                let line = line.unwrap();
                match line.strip_prefix('#') {
                    Some(comment) => ValidInputsLine::Comment(comment.trim().to_string()),
                    None => ValidInputsLine::file(line),
                }
            })
            .collect();

        Self {
            list_file,
            base_dir,
            lines,
        }
    }

    #[cfg(test)]
    pub fn save(&self) {
        use std::io::Write;

        let mut writer = io::BufWriter::new(File::create(&self.list_file).unwrap());
        for line in &self.lines {
            match line {
                ValidInputsLine::Comment(comment) => writeln!(writer, "# {comment}"),
                ValidInputsLine::File(path_components) => {
                    writeln!(writer, "{}", PathBuf::from(path_components).display())
                }
            }
            .unwrap();
        }
    }

    // Returns a `Vec` for easier `par_iter` and progress bar support.
    pub fn all_files(&self) -> Vec<PathBuf> {
        self.lines
            .iter()
            .filter_map(|line| {
                if let ValidInputsLine::File(path_components) = line {
                    Some(self.base_dir.clone().join(PathBuf::from(path_components)))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn existing_files(&self) -> impl Iterator<Item = PathBuf> {
        self.all_files().into_iter().filter(|path| path.exists())
    }

    #[cfg(test)]
    pub fn contains_file(&self, path: impl AsRef<Path>) -> bool {
        self.lines
            .contains(&ValidInputsLine::file_strip_base(path, &self.base_dir))
    }

    #[cfg(test)]
    pub fn add_file_sorted(&mut self, path: impl AsRef<Path>) -> bool {
        let new_line = ValidInputsLine::file_strip_base(path, &self.base_dir);
        for (i, line) in self.lines.iter().enumerate() {
            use std::cmp::Ordering::*;
            match line.cmp(&new_line) {
                // Already present.
                Equal => return false,
                // Continue searching.
                Less => {}
                // Insert in increasing order.
                Greater => {
                    self.lines.insert(i, new_line);
                    return true;
                }
            }
        }
        self.lines.push(new_line);
        true
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ValidInputsLine {
    // Sort comments before files.
    Comment(String),
    File(String),
}

impl ValidInputsLine {
    /// Always use '/' slashes in the list file, to keep it consistent even when updating it on Windows.
    pub fn file(path: impl AsRef<Path>) -> Self {
        Self::File(
            path.as_ref()
                .to_str()
                .expect("The path must be valid UTF-8.")
                .replace('\\', "/"),
        )
    }

    #[cfg(test)]
    /// Keep paths relative to the list file, to not repeat the common prefix.
    pub fn file_strip_base(path: impl AsRef<Path>, base: impl AsRef<Path>) -> Self {
        let path = path
            .as_ref()
            .strip_prefix(base)
            .expect("The path must be relative to the list file.");
        Self::file(path)
    }
}

#[cfg(test)]
/// Nicer string ordering via Unicode collation.
/// The default Rust string ordering is based on byte values and sorts
/// all uppercase characters before lowercase characters (among other things).
impl Ord for ValidInputsLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use icu::collator::Collator;
        use icu::collator::CollatorOptions;
        use icu::locid::locale;
        use icu::locid::Locale;

        const LOCALE_EN_US: Locale = locale!("en_US");
        thread_local! {
            static COLLATOR: Collator = {
                Collator::try_new(&LOCALE_EN_US.into(), CollatorOptions::new()).unwrap()
            };
        }

        use ValidInputsLine::*;
        match (self, other) {
            (Comment(self_), Comment(other)) => COLLATOR.with(|c| c.compare(self_, other)),
            (File(self_), File(other)) => COLLATOR.with(|c| c.compare(self_, other)),
            // Sort comments before files.
            (Comment(_), File(_)) => std::cmp::Ordering::Less,
            (File(_), Comment(_)) => std::cmp::Ordering::Greater,
        }
    }
}

#[cfg(test)]
impl PartialOrd for ValidInputsLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
/// Return all *.wasm files recursively under a root directory that do not have "out/" in their
/// path (because out/ is the default instrumentation output directory of Wasabi).
fn wasm_files(root_dir: impl AsRef<Path>) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    let mut wasm_files = Vec::new();
    for entry in WalkDir::new(&root_dir) {
        let path = entry.unwrap().path().to_owned();
        if std::fs::metadata(&path).unwrap().is_file() {
            if let Some("wasm") = path.extension().and_then(|os_str| os_str.to_str()) {
                if !path
                    .components()
                    .flat_map(|comp| comp.as_os_str().to_str())
                    .any(|dir| dir == "out")
                {
                    wasm_files.push(path);
                }
            }
        }
    }
    wasm_files
}

#[test]
#[ignore] // Ignore by default, to avoid accidentally updating the list file.
pub fn update_valid_inputs_list() {
    let valid_inputs = ValidInputsList::parse(VALID_WASM_BINARIES_LIST_FILE);

    println!("Validating all Wasm binaries in list '{VALID_WASM_BINARIES_LIST_FILE}'...");
    let validated_binaries_count = valid_inputs
        // NOTE: Validate ALL binaries, which panics if any are missing.
        .all_files()
        .par_iter()
        // Abort parallel processing as early as possible.
        .panic_fuse()
        .progress()
        .map(|path| {
            wasm_validate(path)
                .expect("all Wasm files in the valid input list should validate successfully")
        })
        .count();
    println!("Validated all {validated_binaries_count} Wasm binaries in the list.");

    let more_inputs_root_dir = valid_inputs.base_dir.clone();
    let valid_inputs = std::sync::RwLock::new(valid_inputs);
    println!(
        "Checking for new Wasm binaries in '{}/'...",
        more_inputs_root_dir.display()
    );
    let added_binaries_count = wasm_files(more_inputs_root_dir)
        .par_iter()
        .panic_fuse()
        .progress()
        .filter(|path|
            // Early exit: Don't validate binaries that are already in the list.
            !valid_inputs.read().unwrap().contains_file(path)
                && wasm_validate(path).is_ok()
                && valid_inputs.write().unwrap().add_file_sorted(path))
        .count();

    valid_inputs.into_inner().unwrap().save();
    println!("Added {added_binaries_count} new valid Wasm binaries to the list. See the diff of '{VALID_WASM_BINARIES_LIST_FILE}' for the files.");
}
