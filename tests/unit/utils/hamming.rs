use cryptopals::utils::distances::hamming;

#[test]
fn hamming_0() {
    let buf_a = b"this is a test";
    let buf_b = b"wokka wokka!!!";
    assert_eq!(hamming(buf_a, buf_b), 37);
}
