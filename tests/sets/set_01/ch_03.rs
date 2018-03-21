use cryptopals::encoding::base16::Base16;
use cryptopals::xor::single_xor;
use cryptopals::decrypt::break_single_xor;

#[test]
fn challenge_03() {
    let b16 = Base16::new();
    let result: &[u8] = b"Cooking MC's like a pound of bacon";
    let input = b"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let bytes = b16.decode(input);
    let key = break_single_xor(&bytes, b' ').unwrap();
    let output: &[u8] = &single_xor(&bytes, key);
    assert_eq!(result, output);
}
