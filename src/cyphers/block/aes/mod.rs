mod encrypt;
mod decrypt;

use cyphers::xor::fixed_xor;
use cyphers::block::OpMode;
use rayon::slice::ParallelSlice;
use rayon::iter::ParallelIterator;

pub struct AESCypher {
    key: Vec<u8>,
    schedule: Vec<Vec<u8>>,
    mode: OpMode
}

impl AESCypher {
    pub fn new(key: Vec<u8>, mode: OpMode) -> Self {
        let schedule = encrypt::key_schedule(&key);
        AESCypher {
            key: key,
            schedule: schedule,
            mode: mode
        }
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

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        match self.mode {
            OpMode::ECB => plaintext.par_chunks(16)
                .flat_map(|b| self.encrypt_block(b)).collect()
        }
    }

    pub fn decrypt(&self, cyphertext: &[u8]) -> Vec<u8> {
        match self.mode {
            OpMode::ECB => cyphertext.par_chunks(16)
                .flat_map(|b| self.decrypt_block(b)).collect()
        }
    }
}
