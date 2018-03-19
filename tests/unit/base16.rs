use cryptopals::encoding::base16::Base16;

#[test]
fn base16_encode() {
    let b16 = Base16::new();
    let input = b"it just works";
    let result = b"6974206a75737420776f726b73";
    assert_eq!(b16.encode(input), result);
}

#[test]
fn base16_decode() {
    let b16 = Base16::new();
    let input = b"6974206a75737420776f726b73";
    let result = b"it just works";
    assert_eq!(b16.decode(input), result);
}
