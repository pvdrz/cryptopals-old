use cryptopals::xor::fixed_xor;
use cryptopals::encoding::base16::Base16;

#[test]
fn challenge_02() {
    let b16 = Base16::new();
    let buf_a = b16.decode(b"1c0111001f010100061a024b53535009181c");
    let buf_b = b16.decode(b"686974207468652062756c6c277320657965");
    let result = b16.decode(b"746865206b696420646f6e277420706c6179");
    let output = fixed_xor(&buf_a, &buf_b);
    assert_eq!(result, output)
}
