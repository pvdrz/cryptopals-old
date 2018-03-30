use cryptopals::cyphers::block::aes::Cypher;
use cryptopals::cyphers::block::aes::Mode::ECB;
use cryptopals::encoding::base64::Base64;
use cryptopals::utils::io::load_as_bytes;

#[test]
fn challenge_07() {
    let b64 = Base64::new();
    let key = b"YELLOW SUBMARINE".to_vec();
    let cyphertext = b64.decode(&load_as_bytes("data/7.txt").unwrap());
    let cypher = Cypher::new(key, ECB);
    let plaintext = cypher.decrypt(&cyphertext);
    let result: &[u8] = b"I\'m back and I\'m ringin\' the bell";
    assert_eq!(result, &plaintext[0..result.len()]);
}
