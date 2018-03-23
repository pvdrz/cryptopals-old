use cryptopals::xor::repeating_xor;
use cryptopals::encoding::base16::Base16;

#[test]
fn challenge_05() {
    let b16 = Base16::new();
    let input = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";
    let output = b16.decode(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    let result = repeating_xor(input, key);
    assert_eq!(result, output);
}
