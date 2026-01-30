use std::path::PathBuf;

use enumset::EnumSet;
use enumset::EnumSetType;
use serde::Deserialize;
use serde::Serialize;
use serde_plain;
use structopt::StructOpt;

/// Instruments a WebAssembly binary for later dynamic analysis.
/// Produces two files:{n}
///  - an instrumented version of the <input.wasm>, and{n}
///  - an input.wasabi.js file with statically extracted information about the binary and (Wasabi-internal) low-level hooks and runtime.
#[derive(StructOpt, Debug)]
#[structopt(
    // Show options in the help message in the order they are declared here.
    setting = structopt::clap::AppSettings::DeriveDisplayOrder,
    // Do not distinguish FLAGS from OPTIONS in usage string.
    setting = structopt::clap::AppSettings::UnifiedHelpMessage,
    setting = structopt::clap::AppSettings::DisableVersion,
    help_message = "Print this help information.",
    usage = "wasabi [OPTIONS] <input.wasm>",
)]
pub struct Options {
    /// WebAssembly binary to instrument.
    #[structopt(value_name = "input.wasm")]
    pub input_file: PathBuf,

    /// Generate JavaScript code for inclusion in Node.js, not the browser.
    /// Import Wasabi before the WebAssembly module to analyze with
    /// `const Wasabi = require('<filename>.wasabi.js');`
    #[structopt(short = "n", long = "node")]
    pub node_js: bool,

    /// Output directory (created if it does not exist).
    #[structopt(
        short = "o",
        long = "output-dir",
        value_name = "dir",
        default_value = "out/"
    )]
    pub output_dir: PathBuf,

    /// Instrument ONLY for the given list of hooks, not for all hooks. [default: all]
    #[structopt(
        long = "hooks",
        // Must give multiple values as "hookA,hookB" (i.e., comma separated).
        // Otherwise, we have a parsing ambiguity with the positional input file argument.
        require_delimiter = true,
        value_name = "hooks",
    )]
    pub hooks: Vec<Hook>,

    /// Instrument the binary for all hooks EXCEPT for the given ones.
    /// {n}Cannot be combined with the previous option.
    /// {n}<hooks>... is a comma-separated list of hooks like "br,br_if".
    #[structopt(
        long = "no-hooks",
        require_delimiter = true,
        value_name = "hooks",
        conflicts_with = "hooks"
    )]
    pub no_hooks: Vec<Hook>,
}

// Derive parsing, pretty-printing, and convenience like getting all variants of the enum.
#[derive(Debug, Serialize, Deserialize, EnumSetType)]
#[serde(rename_all = "snake_case")]
/// High-level hook names, modulo minor changes:
/// - no trailing underscores (originally to avoid clashes with JavaScript keywords)
/// - grouping together call_pre and call_post into single call option
pub enum Hook {
    Start,

    Nop,
    Unreachable,

    Br,
    BrIf,
    BrTable,

    If,
    Begin,
    End,

    // together for call_pre and call_post
    Call,
    Return,

    Drop,
    // together for Select and TypedSelect
    Select,

    Const,
    Unary,
    Binary,

    Load,
    Store,

    MemorySize,
    MemoryGrow,
    MemoryFill,
    MemoryCopy,
    MemoryInit,

    TableGet,
    TableSet,
    TableSize,
    TableGrow,
    TableFill,
    TableCopy,
    TableInit,

    RefIsNull,

    Local,
    Global,
}

// Use serde_plain for parsing strings to enum variants.
impl std::str::FromStr for Hook {
    type Err = serde_plain::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str(s)
    }
}

// Offers convenient HookSet::all() method.
pub type HookSet = EnumSet<Hook>;
