use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub struct Leb128<T> {
    pub value: T,
    /// When reading an `Leb128<T>`, the number of bytes in which `value` was encoded.
    /// When writing an `Leb128<T>`, the minimum number of bytes used to encode `value`.
    ///
    /// This way, reading and writing an `Leb128<T>` always results in the same number of bytes,
    /// i.e., it round-trips.
    pub byte_count: u8,
}


/* Convenience */

impl<T> Leb128<T> {
    /// Create a new `value` with the byte_count from an `old_value`.
    pub fn with_byte_count<U>(value: T, old_value: &Leb128<U>) -> Self {
        Leb128 {
            value,
            byte_count: old_value.byte_count,
        }
    }
}

impl<T> Deref for Leb128<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for Leb128<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> From<T> for Leb128<T> {
    fn from(value: T) -> Self {
        Leb128 {
            value,
            byte_count: 0,
        }
    }
}


/* Traits for encoding and decoding Leb128 primitive integers */

pub trait ReadLeb128<T>: io::Read {
    fn read_leb128(&mut self) -> io::Result<Leb128<T>>;
}

pub trait WriteLeb128<T>: io::Write {
    /// Write LEB128 encoded `T`, using at least `value.byte_count` bytes.
    /// Returns the actual written byte count.
    fn write_leb128(&mut self, value: &Leb128<T>) -> io::Result<usize>;
}

// Need to write this as a macro, not a generic impl because
// a) num_traits are quite lacking, e.g., there is no "U as T" for primitive integers
// b) cannot use specialization: impl<T: PrimInt> for Leb128<T> overlaps (and is NOT more special
//    than) for example impl<T: Wasm> for Leb128<Vec<T>>
macro_rules! impl_leb128_integer {
    ($T:ident) => {
        impl<R: io::Read> ReadLeb128<$T> for R {
            fn read_leb128(&mut self) -> io::Result<Leb128<$T>> {
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

                Ok(Leb128 {
                    value,
                    byte_count: bytes_read
                })
            }
        }

        impl<W: io::Write> WriteLeb128<$T> for W {
            fn write_leb128(&mut self, leb128: &Leb128<$T>) -> io::Result<usize> {
                let mut value = leb128.value;
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
                    more_bytes = (value > $T::min_value() && value > 0) || bytes_written < leb128.byte_count;
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