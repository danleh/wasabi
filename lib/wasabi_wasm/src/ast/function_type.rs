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
        match (val_type_seq_to_goedel_number::<MAX_INPUTS>(inputs), val_type_seq_to_goedel_number::<MAX_RESULTS>(results)) {
            (Some(inputs), Some(results)) => {
                FunctionType::GoedelNumber {
                    inputs: inputs.try_into().expect("overflow from usize to smaller type"),
                    results: results.try_into().expect("overflow from usize to smaller type")
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

    const fn val_type_to_goedel_number(val_type: ValType) -> usize {
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
    /// that sequence length.
    pub const fn val_type_seq_to_goedel_number<const MAX_SEQ_LEN: usize>(seq: &[ValType]) -> Option<usize> {
        let seq_too_long = seq.len() > MAX_SEQ_LEN;
        if seq_too_long {
            return None;
        }

        let mut result = 0usize;
        let mut exponent = 1usize;
        let mut i = 0;
        // Cannot use for loop in const fn -.-
        while i < seq.len() {
            let val_type = seq[i];
            result *= exponent;
            result += val_type_to_goedel_number(val_type) + 1;
            exponent *= val_type_max_goedel_number() + 1;
            i += 1;
        }

        Some(result)
    }

    #[test]
    fn test_val_type_seq_to_goedel_number() {
        assert_eq!(val_type_seq_to_goedel_number::<0>(&[]), Some(0));
        assert_eq!(val_type_seq_to_goedel_number::<0>(&[ValType::I32]), None);
        assert_eq!(val_type_seq_to_goedel_number::<1>(&[ValType::I32]), Some(1));
        assert_eq!(val_type_seq_to_goedel_number::<1>(&[ValType::Ref]), Some(6));
        assert_eq!(val_type_seq_to_goedel_number::<1>(&[ValType::I32, ValType::I32]), None);
        assert_eq!(val_type_seq_to_goedel_number::<2>(&[ValType::I32, ValType::I32]), Some(7));
    }


    // Reverse direction: Gödel number to slice.
    // Use lookup table with statically allocated slices, because then there is less computation
    // and in particular no memory allocation for those slices.
    // Use the same lookup table for inputs and results.

    macro_rules! table_init_entries {
        // Internal macros are marked with @macro, as recommended by https://danielkeep.github.io/tlborm/book/pat-internal-rules.html.
        // Uses push-down accumulation, as described here https://danielkeep.github.io/tlborm/book/pat-push-down-accumulation.html
        // The first argument is essentially a "unary counter", see https://danielkeep.github.io/tlborm/book/blk-counting.html
        // The second argument (after the -> ) is the accumulated result so far.
        // This case is the recursion anchor: return the result array:
        (@accum $table:ident, ( ) -> $($seq:expr),+ ) => {{
            use ValType::*;
            $(
                let slice = $seq.as_slice();
                if let Some(index) = val_type_seq_to_goedel_number::<MAX_INPUTS_RESULTS>(slice) {
                    $table[index] = slice;
                }
            )+
        }};
        // In the accumulator is a list of bracketed lists of identifiers.
        // The inner lists may be empty, the outer cannot not.
        (@accum $table:ident, ( $x:tt ) -> $( [ $($seq:ident),* ] ),+ ) => { 
            table_init_entries!(@accum $table, $x -> $( 
                [$($seq),*], // FIXME Somehow this is appearing multiple times, not sure why
                [$($seq,)* I32], 
                [$($seq,)* I64], 
                [$($seq,)* F32], 
                [$($seq,)* F64], 
                [$($seq,)* V128], 
                [$($seq,)* Ref]
            ),+ )
        };
        // "Public macro interface".
        ( $table:ident, $x:tt ) => { table_init_entries!(@accum $table, ($x) -> []) };
    }

    #[test]
    fn test_macro() {
        // trace_macros!(true);
        // table_init_entries!(());
        let mut t = vec![&[][..]; 350000];
        table_init_entries!(t, (((()))));
        println!("{:?}", t.iter().rfind(|x| !x.is_empty()));
        // table_init_entries!(((())));
        // table_init_entries!((((()))));
        // trace_macros!(false);
    }

    const MAX_INPUTS_RESULTS: usize = if MAX_INPUTS > MAX_RESULTS { MAX_INPUTS } else { MAX_RESULTS };

    type LookupType = [&'static [ValType]; val_type_seq_max_goedel_number(MAX_INPUTS_RESULTS) + 1];
    const LOOKUP_TABLE: LookupType = {
        let mut table = [[].as_slice(); val_type_seq_max_goedel_number(MAX_INPUTS_RESULTS) + 1];

        table_init_entries!(table, ((())));

        table
    };

    #[test]
    fn test_lookup_table() {
        assert_eq!(LOOKUP_TABLE[0], []);
        assert_eq!(LOOKUP_TABLE[1], [ValType::I32]);
        assert_eq!(LOOKUP_TABLE[2], [ValType::I64]);
        assert_eq!(LOOKUP_TABLE[3], [ValType::F32]);
        assert_eq!(LOOKUP_TABLE[4], [ValType::F64]);
        assert_eq!(LOOKUP_TABLE[5], [ValType::V128]);
        assert_eq!(LOOKUP_TABLE[6], [ValType::Ref]);
        assert_ne!(LOOKUP_TABLE[LOOKUP_TABLE.len()-1], []);
    }

    fn all_possible_seqs(max_len: usize) -> Box<dyn Iterator<Item = &'static [ValType]>> {
        match max_len {
            0 => Box::new(std::iter::once([].as_slice())) as Box<dyn Iterator<Item = &'static [ValType]>>,
            n => Box::new(
                all_possible_seqs(n-1)
                    .chain(all_possible_seqs(n-1).flat_map(|seq|
                        [ValType::I32, ValType::I64, ValType::F32, ValType::F64, ValType::V128, ValType::Ref].iter().map(move |val_type| {
                            let mut new_seq = seq.to_vec();
                            new_seq.push(*val_type);
                            &*Box::leak(new_seq.into_boxed_slice())
                        })
                    ))) as Box<dyn Iterator<Item = &'static [ValType]>>
        }
    }
    
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

mod arena {
    // TODO
}