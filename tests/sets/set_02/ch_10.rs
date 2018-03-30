use cryptopals::cyphers::block::aes::AESCypher;
use cryptopals::cyphers::block::{BlockCypher, OperationMode::CBC};
use cryptopals::encoding::base64::Base64;
use cryptopals::utils::io::load_as_bytes;

#[test]
fn challenge_10() {
    let b64 = Base64::new();
    let key = b"YELLOW SUBMARINE".to_vec();
    let cypher = BlockCypher::new(AESCypher::new(key), CBC(vec![0u8, 16]));

    let cyphertext = b64.decode(&load_as_bytes("data/10.txt").unwrap());
    let plaintext = cypher.decrypt(&cyphertext);
    let result: &[u8] = b"I7 ringin\' the bell \nA rockin\' on the mike while the fly girls yell";

    assert_eq!(result, &plaintext[0..result.len()]);
}
