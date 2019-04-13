use cryptopals::attacks::block::ecb_oracle;
use cryptopals::cyphers::block::aes::AESCypher;
use rand::{thread_rng, RngCore, Rng};

use cryptopals::cyphers::block::{BlockCypher, OperationMode};

#[test]
fn challenge_11() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let is_ecb: bool = rng.gen();
        let black_box = move |b: &[u8]| random_cypher(b, is_ecb);
        assert_eq!(is_ecb, ecb_oracle(&black_box, 16, 4).unwrap());
    }
}

fn random_cypher(plaintext: &[u8], is_ecb: bool) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut key = [0u8; 16];
    rng.fill_bytes(&mut key);

    let mode: OperationMode;
    if is_ecb {
        mode = OperationMode::ECB;
    } else {
        let mut iv = [0u8; 16];
        rng.fill_bytes(&mut iv);
        mode = OperationMode::CBC(iv.to_vec());
    }

    let cypher = BlockCypher::new(AESCypher::new(key.to_vec()), mode);
    let mut input = Vec::new();

    for _ in 0..rng.gen_range(5, 11) {
        input.push(rng.gen());
    }
    input.extend_from_slice(plaintext);
    for _ in 0..rng.gen_range(5, 11) {
        input.push(rng.gen());
    }
    cypher.encrypt(&input)
}

