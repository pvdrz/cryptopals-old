use cryptopals::cyphers::rijndael::encrypt::byte_substitution;
use cryptopals::cyphers::rijndael::encrypt::mix_columns;
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

#[test]
fn mix_columns_0() {
    let input = vec![1, 1, 1, 1, 198, 198, 198, 198, 212, 212, 212, 213, 45, 38, 49, 76];
    let result = vec![1, 1, 1, 1, 198, 198, 198, 198, 213, 213, 215, 214, 77, 126, 189, 248];
    assert_eq!(mix_columns(&input), result);
}
