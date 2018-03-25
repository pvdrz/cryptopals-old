use cryptopals::attacks::xor::break_repeating_xor;
use cryptopals::cyphers::xor::repeating_xor;
use cryptopals::encoding::base64::Base64;
use cryptopals::utils::io::load_as_bytes;

#[test]
fn challenge_06() {
    let b64 = Base64::new();
    let input = b64.decode(&load_as_bytes("data/6.txt").unwrap());
    let result: &[u8] = b"I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike";
    let key = break_repeating_xor(&input, 4);
    let output: &[u8] = &repeating_xor(&input, &key)[0..result.len()];
    assert_eq!(output, result);
}
