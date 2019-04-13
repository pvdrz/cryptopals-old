mod encrypt;
mod decrypt;

use crate::cyphers::xor::fixed_xor;
use crate::cyphers::block::Cypher;

pub struct AESCypher {
    key: Vec<u8>,
    schedule: Vec<Vec<u8>>,
    blocksize: usize
}

impl AESCypher {
    pub fn new(key: Vec<u8>) -> Self {
        let schedule = encrypt::key_schedule(&key);
        AESCypher {
            key: key,
            schedule: schedule,
            blocksize: 16
        }
    }
}

impl Cypher for AESCypher {
    fn blocksize(&self) -> usize {
        self.blocksize
    }

    fn encrypt_block(&self, plaintext: &[u8]) -> Vec<u8> {
        let mut out = fixed_xor(plaintext, &self.key);
        for (i, key) in self.schedule.iter().enumerate() {
            out = encrypt::byte_substitution(&out);
            out = encrypt::shift_rows(&out);
            if i != 9 {
                out = encrypt::mix_columns(&out);
            }
            out = fixed_xor(&out, &key);
        }
        out
    }

    fn decrypt_block(&self, cyphertext: &[u8]) -> Vec<u8> {
        let mut out = cyphertext.to_vec();
        for (i, key) in self.schedule.iter().enumerate().rev() {
            out = fixed_xor(&out, &key);
            if i != 9 {
                out = decrypt::mix_columns(&out);
            }
            out = decrypt::shift_rows(&out);
            out = decrypt::byte_substitution(&out);
        }
        out = fixed_xor(&out, &self.key);
        out
    }
}
