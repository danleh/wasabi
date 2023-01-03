//! Utility functions for testing Wasabi and the Wasm library.

use std::io;
use std::io::BufRead;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use icu::collator::Collator;
use icu::collator::CollatorOptions;
use icu::locid::Locale;
use icu::locid::locale;
use once_cell::sync::Lazy;

// TODO: Improve testing and CI setup:
// 1. Run CI on every commit/PR. Automate with GitHub Actions or similar.
// 2. Save and commit a list of all .wasm files that previously passed the tests
// as a measure against regressions.
// 3. Don't require anything besides Rust to run the tests (what about
// WABT, Emscripten, spec tests?).
// 4. Wasabi should pass all tests in the valid-no-extensions directory, verify.

const TEST_INPUTS_DIR: &str = "../../test-inputs/";
// TODO: Use an explicit list of valid files instead, that one can load from
// disk and update when we support more WebAssembly features.

pub static ALL_VALID_TEST_BINARIES: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    ValidInputsList::parse(VALID_INPUTS_LIST_FILE).existing_files().collect()
});

// pub static ALL_VALID_TEST_BINARIES: Lazy<Vec<PathBuf>> = Lazy::new(|| {
//     println!("Collecting all valid .wasm binaries from '{}'...", TEST_INPUTS_DIR);
//     let mut valid_binaries = wasm_files(TEST_INPUTS_DIR).unwrap();
//     const EXCLUDED: &[&str] = &[
//         // Known invalid files:
//         "invalid",
//         // The full set of WasmBench files is too large to run in CI.
//         "all-binaries-metadata",
//         "filtered-binaries-metadata",
//         "valid-no-extensions",
//         // Valid, but creates very large allocations because it has >500k locals in >1k functions.
//         "31fa012442fd637fca221db4fda94262e99759ab9667147cbedde083aabcc065",
//         //  Uses typed select instruction, which is non-MVP, but is not filtered out by wasm-validate below.
//         "wasm-spec-tests/build/select.wasm",
//         "wasm-spec-tests\\build\\select.wasm", // Windows path.
//     ];
//     for excluded in EXCLUDED.iter() {
//         valid_binaries.retain(|path| !path.to_string_lossy().contains(excluded));
//     }
//     println!("{} .wasm files found, validating...", valid_binaries.len());
//     // Filter out files that are already invalid according to `wasm-validate`.
//     let valid_binaries: Vec<_> = valid_binaries
//         .into_par_iter()
//         .filter(|path| wasm_validate(path).is_ok())
//         .collect();
//     println!("{} valid .wasm binaries.", valid_binaries.len());
//     valid_binaries
// });

/// Call WABT's wasm-validate tool on a file (WABT needs to be on $PATH).
pub fn wasm_validate(path: impl AsRef<Path>) -> Result<(), String> {
    use std::process::Command;

    let path = path.as_ref();
    let validate_output = Command::new("wasm-validate")
        .arg("--ignore-custom-section-errors")
        // Disable all extensions that we don't support yet.
        .arg("--disable-saturating-float-to-int")
        .arg("--disable-sign-extension")
        .arg("--disable-simd")
        .arg("--disable-multi-value")
        .arg("--disable-bulk-memory")
        .arg("--disable-reference-types")
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

/// Return all *.wasm files recursively under a root directory that do not have "out/" in their 
/// path (because out/ is the default instrumentation output directory of Wasabi).
fn wasm_files(root_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, String> {
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

const VALID_INPUTS_LIST_FILE: &str = "../../test-inputs/valid-inputs.txt";

#[derive(Debug)]
struct ValidInputsList {
    list_file: PathBuf,
    base_dir: PathBuf,
    lines: Vec<ValidInputsLine>,
}

impl ValidInputsList {
    pub fn parse(list_file: impl AsRef<Path>) -> Self {
        let list_file = list_file.as_ref().to_owned();
        let base_dir = list_file.parent().expect("list file has no parent directory").to_owned();
        let lines = io::BufReader::new(File::open(&list_file).unwrap())
            .lines()
            .map(|line| {
                let line = line.unwrap();
                match line.strip_prefix('#') {
                    Some(comment) => ValidInputsLine::Comment(comment.trim().to_string()),
                    None => ValidInputsLine::File(PathComponents::from(line)),
                }
            })
            .collect();

        Self {
            list_file,
            base_dir,
            lines
        }
    }

    pub fn save(&self) {
        let mut writer = io::BufWriter::new(File::create(&self.list_file).unwrap());
        for line in &self.lines {
            match line {
                ValidInputsLine::Comment(comment) => writeln!(writer, "# {comment}"),
                ValidInputsLine::File(path_components) => writeln!(writer, "{}", PathBuf::from(path_components).display()),
            }.unwrap();
        }
    }

    // Returns a `Vec` for easier `par_iter` and progress bar support.
    pub fn all_files(&self) -> Vec<PathBuf> {
        self.lines
            .iter()
            .filter_map(|line| 
                if let ValidInputsLine::File(path_components) = line {
                    Some(self.base_dir.clone().join(PathBuf::from(path_components)))
                } else {
                    None
                })
            .collect()
    }
    
    pub fn existing_files(&self) -> impl Iterator<Item = PathBuf> {
        self.all_files().into_iter().filter(|path| path.exists())
    }

    fn make_line(&self, path: impl AsRef<Path>) -> ValidInputsLine {
        let path = path.as_ref().strip_prefix(&self.base_dir).expect("The path must be relative to the list file.");
        ValidInputsLine::File(PathComponents::from(path))
    }

    pub fn contains_file(&self, path: impl AsRef<Path>) -> bool {
        self.lines.contains(&self.make_line(path))
    }

    pub fn add_file_sorted(&mut self, path: impl AsRef<Path>) -> bool {
        let new_line = self.make_line(path);
        for (i, line) in self.lines.iter().enumerate() {
            use std::cmp::Ordering::*;
            match line.cmp(&new_line) {
                // Already present.
                Equal => return false,
                // Continue searching.
                Less => {},
                // Insert in increasing order.
                Greater => {
                    self.lines.insert(i, new_line);
                    return true;
                },
            }
        }
        self.lines.push(new_line);
        true
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum ValidInputsLine {
    // Sort comments before files.
    Comment(String),
    File(PathComponents),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PathComponents(Vec<LocaleString>);

impl<P> From<P> for PathComponents
where P: AsRef<Path> {
    fn from(path: P) -> Self {
        Self(path.as_ref()
            .components()
            .map(|component| LocaleString(component.as_os_str().to_str().expect("non-portable path").to_owned()))
            .collect())
    }
}

impl From<&PathComponents> for PathBuf {
    fn from(components: &PathComponents) -> Self {
        PathBuf::from(components.0.iter().map(|s| s.0.as_str()).collect::<Vec<_>>().join("/"))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct LocaleString(String);

impl PartialOrd for LocaleString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const LOCALE_EN_US: Locale = locale!("en_US");

thread_local! {
    static COLLATOR: Collator = {
        Collator::try_new_unstable(&icu_testdata::unstable(), &LOCALE_EN_US.into(), CollatorOptions::new()).unwrap()
    };
}

impl Ord for LocaleString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        COLLATOR.with(|collator| collator.compare(&self.0, &other.0))
    }
}

#[cfg(test)]
mod commands {
    use super::*;

    use indicatif::ParallelProgressIterator;
    use rayon::prelude::*;

    #[test]
    pub fn validate_valid_inputs_list() {
        let valid_inputs = ValidInputsList::parse(VALID_INPUTS_LIST_FILE);

        println!("Validating all Wasm binaries in the list '{VALID_INPUTS_LIST_FILE}'...");
        let validated_binaries_count = valid_inputs
            .all_files()
            .par_iter()
            .progress()
            // Abort parallel processing as early as possible.
            .panic_fuse()
            .map(|path| wasm_validate(path).unwrap())
            .count();
        println!("Validated all {validated_binaries_count} Wasm binaries in the list.");
    }

    #[test]
    pub fn update_valid_inputs_list() {
        let valid_inputs = ValidInputsList::parse(VALID_INPUTS_LIST_FILE);

        let more_inputs_root_dir = valid_inputs.base_dir.clone();
        let valid_inputs = std::sync::RwLock::new(valid_inputs);
        println!("Checking for new Wasm binaries in '{}'...", more_inputs_root_dir.display());
        // TODO remove wasm_files
        let added_binaries_count = wasm_files(more_inputs_root_dir).unwrap()
            .par_iter()
            .progress()
            .filter(|path|
                // Early exit: Don't validate binaries that are already in the list.
                !valid_inputs.read().unwrap().contains_file(path)
                    && wasm_validate(path).is_ok() 
                    && valid_inputs.write().unwrap().add_file_sorted(path))
            .count();

        valid_inputs.into_inner().unwrap().save();
        println!("Added {added_binaries_count} new valid Wasm binaries to the list.");
    }
}