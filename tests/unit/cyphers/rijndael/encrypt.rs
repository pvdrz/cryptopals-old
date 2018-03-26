use cryptopals::cyphers::rijndael::encrypt::byte_substitution;
use cryptopals::cyphers::rijndael::encrypt::shift_rows;

#[test]
fn byte_substitution_0() {
    assert_eq!(0x25, byte_substitution(&vec![0xc2])[0]);
}

#[test]
fn shift_rows_0() {
    let input: Vec<u8> = (0..16).collect();
    let result = vec![0u8, 4, 8, 12, 5, 9, 13, 1, 10, 14, 2, 6, 15, 3, 7, 11];
    assert_eq!(shift_rows(&input), result);
}
