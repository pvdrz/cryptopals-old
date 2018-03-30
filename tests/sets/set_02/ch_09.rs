use cryptopals::utils::padding::pkcs7;

#[test]
fn challenge_09() {
    let input = b"YELLOW SUBMARINE";
    let output: &[u8] = &pkcs7(input, 20);
    let result: &[u8] = b"YELLOW SUBMARINE\x04\x04\x04\x04";
    assert_eq!(result, output)
}
