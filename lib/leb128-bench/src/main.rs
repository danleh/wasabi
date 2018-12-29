#![feature(test)]

extern crate test;

use leb128::{read, write};
use leb128_daniel::{ReadLeb128, WriteLeb128};
use test::Bencher;

#[test]
fn test_equal() {
    for i in 0..1000000u64 {
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        vec1.write_leb128(i).unwrap();
        write::unsigned(&mut vec2, i).unwrap();
        assert_eq!(&vec1, &vec2);
    }
}

#[bench]
fn encode_daniel(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 0..100000u64 {
            let mut vec = Vec::new();
            vec.write_leb128(i).unwrap();
            let _o: u64 = (&mut &vec[..]).read_leb128().unwrap();
        }
    })
}

#[bench]
fn encode_crates_io(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 0..100000u64 {
            let mut vec = Vec::new();
            write::unsigned(&mut vec, i).unwrap();
            let _o = read::unsigned(&mut &vec[..]).unwrap();
        }
    })
}
