use cryptopals::cyphers::rijndael::encrypt::byte_substitution;
use cryptopals::cyphers::rijndael::encrypt::mix_columns;
use cryptopals::cyphers::rijndael::encrypt::shift_rows;
use cryptopals::cyphers::rijndael::encrypt::key_schedule;

#[test]
fn byte_substitution_0() {
    assert_eq!(0x25, byte_substitution(&vec![0xc2])[0]);
}

#[test]
fn shift_rows_0() {
    let input: Vec<u8> = (0..16).collect();
    let result = vec![0u8, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12, 1, 6, 11];
    assert_eq!(shift_rows(&input), result);
}

#[test]
fn mix_columns_0() {
    let input = vec![1, 1, 1, 1, 198, 198, 198, 198, 212, 212, 212, 213, 45, 38, 49, 76];
    let result = vec![1, 1, 1, 1, 198, 198, 198, 198, 213, 213, 215, 214, 77, 126, 189, 248];
    assert_eq!(mix_columns(&input), result);
}

#[test]
fn key_schedule_0() {
    let input: Vec<u8> = vec![0u8; 16];
    let result = vec![
        vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        vec![0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63, 0x62, 0x63, 0x63, 0x63],
        vec![0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa, 0x9b, 0x98, 0x98, 0xc9, 0xf9, 0xfb, 0xfb, 0xaa],
        vec![0x90, 0x97, 0x34, 0x50, 0x69, 0x6c, 0xcf, 0xfa, 0xf2, 0xf4, 0x57, 0x33, 0x0b, 0x0f, 0xac, 0x99],
        vec![0xee, 0x06, 0xda, 0x7b, 0x87, 0x6a, 0x15, 0x81, 0x75, 0x9e, 0x42, 0xb2, 0x7e, 0x91, 0xee, 0x2b],
        vec![0x7f, 0x2e, 0x2b, 0x88, 0xf8, 0x44, 0x3e, 0x09, 0x8d, 0xda, 0x7c, 0xbb, 0xf3, 0x4b, 0x92, 0x90],
        vec![0xec, 0x61, 0x4b, 0x85, 0x14, 0x25, 0x75, 0x8c, 0x99, 0xff, 0x09, 0x37, 0x6a, 0xb4, 0x9b, 0xa7],
        vec![0x21, 0x75, 0x17, 0x87, 0x35, 0x50, 0x62, 0x0b, 0xac, 0xaf, 0x6b, 0x3c, 0xc6, 0x1b, 0xf0, 0x9b],
        vec![0x0e, 0xf9, 0x03, 0x33, 0x3b, 0xa9, 0x61, 0x38, 0x97, 0x06, 0x0a, 0x04, 0x51, 0x1d, 0xfa, 0x9f],
        vec![0xb1, 0xd4, 0xd8, 0xe2, 0x8a, 0x7d, 0xb9, 0xda, 0x1d, 0x7b, 0xb3, 0xde, 0x4c, 0x66, 0x49, 0x41],
        vec![0xb4, 0xef, 0x5b, 0xcb, 0x3e, 0x92, 0xe2, 0x11, 0x23, 0xe9, 0x51, 0xcf, 0x6f, 0x8f, 0x18, 0x8e]
    ];
    assert_eq!(key_schedule(&input), result);
}
