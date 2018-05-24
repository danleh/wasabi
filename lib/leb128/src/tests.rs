use {ReadLeb128, WriteLeb128};

#[test]
/// exhaustively test that decode(encode(value)) == value for u16 and i16
fn roundtrips() {
    for u in u16::min_value()..=u16::max_value() {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_leb128(u).unwrap();
        let u_decode: u16 = buf.as_slice().read_leb128().unwrap();
        assert_eq!(u, u_decode,
                   "\nbuffer:{}",
                   buf.iter().map(|byte| format!(" 0x{:x}", byte)).collect::<Vec<String>>().concat());
    }

    for i in i16::min_value()..=i16::max_value() {
        let mut buf: Vec<u8> = Vec::new();
        buf.write_leb128(i).unwrap();
        let i_decode: i16 = buf.as_slice().read_leb128().unwrap();
        assert_eq!(i, i_decode,
                   "\nbuffer:{}",
                   buf.iter().map(|byte| format!(" 0x{:x}", byte)).collect::<Vec<String>>().concat());
    }
}