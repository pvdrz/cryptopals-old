use cryptopals::encoding::base16::Base16;
use cryptopals::encoding::base64::Base64;

#[test]
fn challenge_01 () {
    let b16 = Base16::new();
    let b64 = Base64::new();
    let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let result: &[u8] = b"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let output: &[u8] = &b64.encode(&b16.decode(input));
    assert_eq!(output, result);
}
