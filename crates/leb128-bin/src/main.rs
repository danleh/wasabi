use wasabi_leb128::{ReadLeb128, WriteLeb128};
use main_error::MainError;
use std::io::{self, BufRead};
use structopt::StructOpt;

/// Utility to encode/decode (decimal) integers to/from (hex) LEB128.
#[derive(StructOpt, Debug)]
#[structopt(
    name = "leb128",
    // Allow to pass, e.g., -1 as a number to convert (and do not parse it as an option/flag).
    setting = structopt::clap::AppSettings::AllowNegativeNumbers
)]
struct Options {
    /// Decode LEB128 to integer (default: encode integer to LEB128)
    #[structopt(short = "d", long = "decode", display_order = 1)]
    decode: bool,

    /// Encode/decode signed LEB128 (default: unsigned LEB128)
    #[structopt(short = "s", long = "signed", display_order = 2)]
    signed: bool,

    /// Integer (decimal) to encode or LEB128 (hex bytes) to decode.
    /// {n}Read from standard input if not given.
    /// {n}Hex bytes must have two digits each (one per nibble).
    /// {n}Hex bytes may be prefixed with '0x' and may contain spaces.
    #[structopt(name = "INPUT")]
    input: Vec<String>,
}

fn main() -> Result<(), MainError> {
    let opt = Options::from_args();

    let input = if opt.input.is_empty() {
        // Read input from stdin if no number is given as argument.
        io::stdin().lock().lines().next().unwrap()?
    } else {
        // Concatenate space-separated groups of a single number.
        opt.input.concat()
    };

    if opt.decode {
        let input = input.replace("0x", "").replace(' ', "");
        let buf = hex::decode(input)?;
        let input_num_bytes = buf.len();
        let mut cursor = io::Cursor::new(buf);
        let result = if opt.signed {
            let (int, _): (i64, _) = cursor.read_leb128()?;
            int.to_string()
        } else {
            let (int, _): (u64, _) = cursor.read_leb128()?;
            int.to_string()
        };
        if cursor.position() < input_num_bytes as u64 {
            Err("provided LEB128 has too many bytes")?;
        }
        println!("{}", result)
    } else {
        let mut buf = Vec::new();
        if opt.signed {
            let input: i64 = input.parse()?;
            buf.write_leb128(input)?;
        } else {
            let input: u64 = input.parse()?;
            buf.write_leb128(input)?;
        }
        println!("0x{}", hex::encode(buf))
    }

    Ok(())
}
