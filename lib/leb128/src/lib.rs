use std::io;

use byteorder::{ReadBytesExt, WriteBytesExt};

#[cfg(test)]
mod tests;

/* Traits for encoding and decoding Leb128 primitive integers */

pub trait ReadLeb128<T>: io::Read {
    fn read_leb128(&mut self) -> io::Result<T>;
}

pub trait WriteLeb128<T>: io::Write {
    /// Returns the actual written byte count.
    fn write_leb128(&mut self, value: T) -> io::Result<usize>;
}

/* Trait implementations */

const CONTINUATION_BIT: u8 = 0x80;

fn is_set_continuation_bit(byte: u8) -> bool {
    byte & CONTINUATION_BIT == CONTINUATION_BIT
}

const SIGN_BIT: u8 = 0x40;

fn is_set_sign_bit(byte: u8) -> bool {
    byte & SIGN_BIT == SIGN_BIT
}

macro_rules! signed {
    ($T: ident) => ($T::min_value() != 0)
}

// Need to write this as a macro, not a generic impl because num_traits are quite lacking, e.g.,
// there is no "U as T" for primitive integers.
macro_rules! impl_leb128_integer {
    ($T: ident) => {
        impl<R: io::Read> ReadLeb128<$T> for R {
            fn read_leb128(&mut self) -> io::Result<$T> {
                let mut value = 0;
                // useful if you want to preserve length when encoding this LEB128 again (unused currently though)
                let mut _bytes_read = 0;
                let mut shift = 0;
                let mut byte = CONTINUATION_BIT;

                while is_set_continuation_bit(byte) {
                    byte = self.read_u8()?;
                    // mask off continuation bit from byte and prepend lower 7 bits to value
                    if let Some(high_bits) = ((byte & !CONTINUATION_BIT) as $T).checked_shl(shift) {
                        value |= high_bits;
                    } else {
                        return Err(io::Error::new(io::ErrorKind::InvalidData, format!("LEB128 to {} overflow", stringify!($T))));
                    }
                    _bytes_read += 1;
                    shift += 7;
                }

                if signed!($T) {
                    let (sign_extend, did_overflow) = (!0 as $T).overflowing_shl(shift);
                    if is_set_sign_bit(byte) && !did_overflow {
                        value |= sign_extend;
                    }
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
                    let mut byte_to_write = value as u8 & !CONTINUATION_BIT;
                    // sign extends, important for signed integers!
                    value >>= 7;
                    bytes_written += 1;

                    more_bytes = value != if signed!($T) && is_set_sign_bit(byte_to_write) { !0 } else { 0 };
                    if more_bytes {
                        byte_to_write |= CONTINUATION_BIT;
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

// for testing, can be exhaustively checked for correctness
impl_leb128_integer!(u16);
impl_leb128_integer!(i16);