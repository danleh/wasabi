use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;

/* Traits for encoding and decoding Leb128 primitive integers */

pub trait ReadLeb128<T>: io::Read {
    fn read_leb128(&mut self) -> io::Result<T>;
}

pub trait WriteLeb128<T>: io::Write {
    /// Returns the actual written byte count.
    fn write_leb128(&mut self, value: T) -> io::Result<usize>;
}

// Need to write this as a macro, not a generic impl because num_traits are quite lacking, e.g.,
// there is no "U as T" for primitive integers.
macro_rules! impl_leb128_integer {
    ($T:ident) => {
        impl<R: io::Read> ReadLeb128<$T> for R {
            fn read_leb128(&mut self) -> io::Result<$T> {
                let mut value = 0;
                let mut bytes_read = 0;
                let mut shift = 0;
                let mut byte = 0x80;

                while byte & 0x80 != 0 {
                    byte = self.read_u8()?;
                    // mask off continuation bit from byte and prepend lower 7 bits to value
                    if let Some(high_bits) = ((byte & 0x7f) as $T).checked_shl(shift) {
                        value |= high_bits;
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("LEB128 to {} overflow", stringify!($T))));
                    }
                    bytes_read += 1;
                    shift += 7;
                }

                Ok(value)
            }
        }

        impl<W: io::Write> WriteLeb128<$T> for W {
            fn write_leb128(&mut self, mut value: $T) -> io::Result<usize> {
                let mut bytes_written = 0;
                let mut more_bytes = true;

                while more_bytes {
                    // select low 7 bits of value
                    let mut byte_to_write = value as u8 & 0x7F;
                    // sign extends, important for signed integers!
                    value >>= 7;
                    bytes_written += 1;

                    // for unsigned integers, min_value and 0 are the same, but for signed ones the
                    // double check of value is important: -1 (all 1's) and 0 (all 0's) stop writing
                    more_bytes = value > $T::min_value() && value > 0;
                    if more_bytes {
                        byte_to_write |= 0x80;
                    }
                    self.write_u8(byte_to_write)?;
                }

                Ok(bytes_written as usize)
            }
        }
    }
}

impl_leb128_integer!(u32);
impl_leb128_integer!(usize);
impl_leb128_integer!(i32);
impl_leb128_integer!(i64);