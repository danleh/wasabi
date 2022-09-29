//! Because function types are commonly created, compared, copied etc. we have a very optimized
//! representation for them.
//! The goals are:
//! - Use at most 32 bits of memory, because they are frequently part of larger
//! AST data types, e.g., in functions and instructions.
//! - Should be cheap to compare.
//! - Should be cheap to copy, ideally just a memcpy (Rust: Copy trait)
//! - Should be cheap to create, which is espaclly common in parsing and type checking.
//! 
//! The first solution was to use some form of interning (a global arena for all function types).
//! The problem was that creating lots of function types was slow, because it had to create the
//! non-interned version first before comparing.

use std::num::*;

// use crate::{ValType, lowlevel};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Ref,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum FunctionType {
    GoedelNumber {
        inputs: u16,
        results: u8
    },
    ArenaAllocated {
        // TODO Once the niche optimizations are improved in later versions of the Rust language
        // (https://github.com/rust-lang/rust/issues/46213), this could be a NonZeroU32, to make
        // it a bit easier/idiomatic to work with.
        // But even with 3 bytes (2^24 = 16.7 million possible ids), it should be more then enough
        // space for a realistic amount of unique function types.
        id: [u8; 3]
    }
}

#[test]
fn size() {
    assert_eq!(std::mem::size_of::<FunctionType>(), 4);
    assert_eq!(std::mem::size_of::<Option<FunctionType>>(), 4);
    assert_eq!(std::mem::size_of::<Option<Option<FunctionType>>>(), 4);
}

const MAX_INPUTS: usize = 5;
const MAX_RESULTS: usize = 2;

impl FunctionType {
    pub fn new(inputs: &[ValType], results: &[ValType]) -> Self {
        use goedel_number::*;
        match (val_type_seq_to_goedel_number::<u16, MAX_INPUTS>(inputs), val_type_seq_to_goedel_number::<u8, MAX_RESULTS>(results)) {
            (Some(inputs), Some(results)) => {
                FunctionType::GoedelNumber {
                    inputs,
                    results
                }
            },
            _ => Self::new_arena(inputs, results)
        }
    }

    fn new_arena(inputs: &[ValType], results: &[ValType]) -> Self {
        unimplemented!()
    }
}


mod goedel_number {
    use super::*;

    // Forward direction: ValType slice to Gödel number.

    fn val_type_to_goedel_number(val_type: ValType) -> usize {
        match val_type {
            ValType::I32 => 0,
            ValType::I64 => 1,
            ValType::F32 => 2,
            ValType::F64 => 3,
            ValType::V128 => 4,
            ValType::Ref => 5,
        }
    }

    const fn val_type_max_goedel_number() -> usize {
        5
    }

    const fn val_type_seq_max_goedel_number(max_seq_len: usize) -> usize {
        // This is a geometric series, e.g., for 6 possible values it is:
        // 1 (for the empty sequence) 
        // + 6 (for the sequence with one element) 
        // + 36 ...
        // = (1 - 6^(max_seq_len+1)) / (1 - 6)
        let goedel_number_count = ((val_type_max_goedel_number() + 1).pow(max_seq_len as u32 + 1) - 1) / val_type_max_goedel_number();
        goedel_number_count - 1
    }

    #[test]
    fn test_goedel_number_constants() {
        assert_eq!(val_type_to_goedel_number(ValType::I32), 0);
        assert_eq!(val_type_to_goedel_number(ValType::F64), 3);
        assert_eq!(val_type_seq_max_goedel_number(0), 0);
        assert_eq!(val_type_seq_max_goedel_number(1), 6);
        assert_eq!(val_type_seq_max_goedel_number(2), 42);
    }

    /// Returns `None` if the `seq` is not representable as a Gödel number with
    /// that 
    pub fn val_type_seq_to_goedel_number<T: TryFrom<usize>, const MAX_SEQ_LEN: usize>(seq: &[ValType]) -> Option<T> {
        assert!(T::try_from(val_type_seq_max_goedel_number(MAX_SEQ_LEN)).is_ok(), "Sequences of up to length {MAX_SEQ_LEN} are too large to be represented as a {}", std::any::type_name::<T>());    
        
        let seq_too_long = seq.len() > MAX_SEQ_LEN;
        if seq_too_long {
            return None;
        }

        let mut result = 0usize;
        let mut exponent = 1usize;
        for val_type in seq { // seq: [i32]
            result *= exponent; // = 0 * 1 = 0
            result += val_type_to_goedel_number(*val_type) + 1;
            exponent *= val_type_max_goedel_number() + 1;
        }

        #[allow(non_snake_case)]
        let result = match T::try_from(result) {
            Ok(result) => result,
            Err(_number_too_large_for_T) => unreachable!("assert above should make overflow impossible")
        };
        Some(result)
    }

    #[test]
    fn test_val_type_seq_to_goedel_number() {
        assert_eq!(val_type_seq_to_goedel_number::<u8, 0>(&[]), Some(0));
        assert_eq!(val_type_seq_to_goedel_number::<u8, 0>(&[ValType::I32]), None);
        assert_eq!(val_type_seq_to_goedel_number::<u8, 1>(&[ValType::I32]), Some(1));
        assert_eq!(val_type_seq_to_goedel_number::<u8, 1>(&[ValType::Ref]), Some(6));
        assert_eq!(val_type_seq_to_goedel_number::<u8, 1>(&[ValType::I32, ValType::I32]), None);
        assert_eq!(val_type_seq_to_goedel_number::<u8, 2>(&[ValType::I32, ValType::I32]), Some(7));
    }

    #[test]
    #[should_panic]
    fn seq_goedel_numbers_too_large() {
        val_type_seq_to_goedel_number::<u8, 3>(&[]);
    }


    // Reverse direction: Gödel number to slice.
    // Use lookup table with statically allocated slices, because then there is less computation
    // and in particular no memory allocation for those slices.
    // Use the same lookup table for inputs and results.

    const MAX_INPUTS_RESULTS: usize = if MAX_INPUTS > MAX_RESULTS { MAX_INPUTS } else { MAX_RESULTS };

    type LookupType = [&'static [ValType]; val_type_seq_max_goedel_number(MAX_INPUTS_RESULTS) + 1];
    const LOOKUP_TABLE: LookupType = {
        let mut table = [[].as_slice(); val_type_seq_max_goedel_number(MAX_INPUTS_RESULTS) + 1];
    
        const MAX_SEQ: u8 = if INPUTS_VALTPYES > RESULTS_VALTYPES { INPUTS_VALTPYES } else { RESULTS_VALTYPES };
        // TODO recursively generate all possible sequences, for each seq
        // table[seq_to_bitstring(seq).unwrap() as usize] = seq;
    
        table
    };
    
    fn goedel_number_to_val_type_seq(u: usize) -> Option<&'static [ValType]> {
        LOOKUP_TABLE.get(u).copied()
    }

    #[test]
    fn test_goedel_number_to_val_type_seq() {
        assert_eq!(goedel_number_to_val_type_seq(0), Some([].as_slice()));
        assert_eq!(goedel_number_to_val_type_seq(1), Some([ValType::I32].as_slice()));
        assert_eq!(goedel_number_to_val_type_seq(6), Some([ValType::Ref].as_slice()));
        assert_eq!(goedel_number_to_val_type_seq(7), Some([ValType::I32, ValType::I32].as_slice()));
    }

}

// type FnTypeMockup = Option<(NonZeroU16, NonZeroU16)>;

// TODO derive from ValType via log2
const BITS_PER_VALTYPE: u8 = 3;

fn valtype_to_bits(valtype: ValType) -> u32 {
    let bits = match valtype {
        ValType::I32 => 1,
        ValType::I64 => 2,
        ValType::F32 => 3,
        ValType::F64 => 4,
        ValType::V128 => 5,
        ValType::Ref => 6,
    };
    assert!(bits < 2u32.pow(BITS_PER_VALTYPE as u32));
    bits
}

fn seq_to_bitstring<const MAX_SEQ: u8> (val_type_seq: &[ValType]) -> Option<u32> {
    assert!(2u64.pow(MAX_SEQ as u32) <= usize::MAX as u64);

    if val_type_seq.len() > MAX_SEQ as usize {
        None
    } else {
        let mut result = 0;
        for val_type in val_type_seq {
            result <<= BITS_PER_VALTYPE;
            result |= valtype_to_bits(*val_type);
        }
        Some(result)
    }
}

const INPUTS_VALTPYES: u8 = 3;
const RESULTS_VALTYPES: u8 = 2;

const INPUTS_BITS: u8 = INPUTS_VALTPYES * BITS_PER_VALTYPE;
const RESULTS_BITS: u8 = RESULTS_VALTYPES * BITS_PER_VALTYPE;
const USED_BITS: u8 = INPUTS_BITS + RESULTS_BITS;

fn pair_to_bitstring(inputs: &[ValType], results: &[ValType]) -> Option<NonZeroU32> {
    let inputs_bits = seq_to_bitstring::<INPUTS_VALTPYES>(inputs)?;
    let results_bits = seq_to_bitstring::<RESULTS_VALTYPES>(results)?;
    let result = (inputs_bits << RESULTS_BITS) | results_bits;
    let result = result.checked_add(1)?;
    Some(NonZeroU32::new(result).expect("impossible because of +1"))
}

// Reverse direction

// A lookup table from bitstring to slice of ValTypes
const SEQ_LOOKUP_TABLE: SeqLookupType = generate_lookup_table();
const SEQ_LOOKUP_ENTRIES: usize = 2usize.pow(if INPUTS_BITS > RESULTS_BITS { INPUTS_BITS } else { RESULTS_BITS } as u32);
type SeqLookupType = [Option<&'static [ValType]>; SEQ_LOOKUP_ENTRIES];
const fn generate_lookup_table() -> SeqLookupType {
    let mut table = [None; SEQ_LOOKUP_ENTRIES];

    // FIXME generate 
    const MAX_SEQ: u8 = if INPUTS_VALTPYES > RESULTS_VALTYPES { INPUTS_VALTPYES } else { RESULTS_VALTYPES };
    // TODO recursively generate all possible sequences, for each seq
    // table[seq_to_bitstring(seq).unwrap() as usize] = seq;

    table
}

fn bitstring_to_seq(bitstring: usize) -> Option<&'static [ValType]> {
    if bitstring > SEQ_LOOKUP_TABLE.len() {
        None
    } else {
        SEQ_LOOKUP_TABLE[bitstring]
    }
}

fn bitstring_to_pair(u: NonZeroU32) -> Option<(&'static [ValType], &'static [ValType])> {
    let u = u.get() - 1;
    let u = u as usize;
    let results_bits = u & ((1 << RESULTS_BITS) - 1);
    let inputs_bits = u >> RESULTS_BITS;

    let inputs = bitstring_to_seq(inputs_bits)?;
    let results = bitstring_to_seq(results_bits)?;

    Some((inputs, results))
}

// TODO automatic tests
// TODO generate lookup table: index -> (&'static [ValType], &'static [ValType])


// values of .0 < 256: statically allocated function types
// allocate vector of function types, such that index can directly give you the param and result slices

// values of .0 >= 256: dynamically allocated function types

// impl FunctionType {
//     pub fn from(inputs: &[ValType], results: &[ValType]) -> Self {
//         match (inputs, results) {
//             ([], []) => FunctionType::Empty,
//             ([input], []) => FunctionType::SingleInput(*input),
//             ([], [result]) => FunctionType::SingleResult(*result),
//             ([input], [result]) => FunctionType::Unary(*input, *result),
//             ([input1, input2], [result]) => FunctionType::Binary(*input1, *input2, *result),
//             _ => todo!()
//         }
//     }
// }

#[test]
fn pack() {
    fn print(val_type_seq: &[ValType]) {
        println!("{:?} == {:?}", val_type_seq, seq_to_bitstring::<16>(val_type_seq))
    }
    print(&[]);
    print(&[ValType::I32]);
    print(&[ValType::I64]);
    print(&[ValType::F32]);
    print(&[ValType::F64]);
    print(&[ValType::V128]);
    print(&[ValType::Ref]);
    print(&[ValType::I32, ValType::I32]);
}