use rand::{thread_rng, RngCore};

use cryptopals::attacks::block::{baat_ecb_decrypt, ecb_oracle, find_blocksize};
use cryptopals::cyphers::block::aes::AESCypher;
use cryptopals::cyphers::block::{BlockCypher, OperationMode};
use cryptopals::encoding::base64::Base64;

#[test]
fn challenge_12() {
    let b64 = Base64::new();
    let suffix = b64.decode(b"Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK");

    let black_box = BlackBox::new(&suffix);
    let encrypt = |b: &[u8]| black_box.encrypt(b);

    let blocksize = find_blocksize(&encrypt, 128).unwrap();
    assert_eq!(16, blocksize);

    let is_ecb = ecb_oracle(&encrypt, blocksize, 11).unwrap();
    assert!(is_ecb);

    let out = baat_ecb_decrypt(&encrypt, blocksize, 10)[0..suffix.len()].to_vec();

    assert_eq!(suffix, out);
}

struct BlackBox {
    cypher: BlockCypher<AESCypher>,
    suffix: Vec<u8>,
}

impl BlackBox {
    fn new(suffix: &[u8]) -> Self {
        let mut rng = thread_rng();
        let mut key = [0u8; 16];
        rng.fill_bytes(&mut key);
        let cypher = BlockCypher::new(AESCypher::new(key.to_vec()), OperationMode::ECB);
        BlackBox {
            cypher: cypher,
            suffix: suffix.to_vec(),
        }
    }

    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut plaintext = plaintext.to_vec();
        plaintext.extend_from_slice(&self.suffix);
        self.cypher.encrypt(&plaintext)
    }
}
