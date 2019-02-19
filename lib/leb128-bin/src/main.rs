use leb128::{ReadLeb128, WriteLeb128};
use std::error::Error;
use std::io::{self, BufRead};
use structopt::StructOpt;

/// Utility to encode/decode integers to/from LEB128.
#[derive(StructOpt, Debug)]
#[structopt(name = "leb128", bin_name = "leb128", raw(setting = "structopt::clap::AppSettings::AllowLeadingHyphen"))]
struct Options {
    /// Decode LEB128 to integer
    #[structopt(short = "d", long = "decode", display_order = 1)]
    decode: bool,

    /// Encode/decode signed LEB128 instead of unsigned
    #[structopt(short = "s", long = "signed", display_order = 2)]
    signed: bool,

    /// Integer (decimal) or LEB128 (hex bytes) input.
    /// When not given, read standard input.
    #[structopt(name = "INPUT")]
    input: Option<String>,
}

fn main() -> Result<(), Box<Error>> {
    let opt = Options::from_args();

    // take input either from command line (if given) or stdin
    let input = if let Some(input_argument) = opt.input {
        input_argument
    } else {
        io::stdin().lock().lines().next().unwrap()?
    };

    if opt.decode {
        let input = input.trim_start_matches("0x");
        let buf = hex::decode(input)?;
        let input_num_bytes = buf.len();
        let mut cursor = io::Cursor::new(buf);
        let result = if opt.signed {
            let int: i64 = cursor.read_leb128()?;
            int.to_string()
        } else {
            let int: u64 = cursor.read_leb128()?;
            int.to_string()
        };
        if cursor.position() < input_num_bytes as u64 {
            Err(io::Error::new(io::ErrorKind::InvalidData, "provided LEB128 has too many bytes"))?;
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
