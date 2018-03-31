use cyphers::xor::fixed_xor;
use rayon::slice::ParallelSlice;
use rayon::iter::ParallelIterator;
use utils::padding::pkcs7;

pub mod aes;

pub enum OperationMode {
    ECB,
    CBC(Vec<u8>)
}

pub struct BlockCypher<T: Cypher> {
    operation_mode: OperationMode,
    blocksize: usize,
    cypher: T
}

impl<T: Cypher> BlockCypher<T> {
    pub fn new(cypher: T, operation_mode: OperationMode) -> Self {
        BlockCypher {
            operation_mode: operation_mode,
            blocksize: cypher.blocksize(),
            cypher: cypher
        }
    }
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let plaintext: &[u8] = &pkcs7(plaintext, self.blocksize);
        match self.operation_mode {
            OperationMode::ECB => plaintext
                .par_chunks(self.blocksize)
                .flat_map(|b| self.cypher.encrypt_block(b)).collect(),
            OperationMode::CBC(ref init_vec) => {
                let mut vec = init_vec.clone();
                let mut cyphertext = Vec::new();
                for block in plaintext.chunks(self.blocksize) {
                    vec = self.cypher.encrypt_block(&fixed_xor(block, &vec));
                    cyphertext.extend_from_slice(&vec);
                }
                cyphertext
            }
        }
    }
    pub fn decrypt(&self, cyphertext: &[u8]) -> Vec<u8> {
        match self.operation_mode {
            OperationMode::ECB => cyphertext
                .par_chunks(self.blocksize)
                .flat_map(|b| self.cypher.decrypt_block(b)).collect(),
            OperationMode::CBC(ref init_vec) => {
                let mut vec = init_vec.clone();
                let mut plaintext = Vec::new();
                for block in cyphertext.chunks(self.blocksize) {
                    plaintext.extend_from_slice(
                        &fixed_xor(&self.cypher.decrypt_block(&block), &vec));
                    vec = block.to_vec();
                }
                plaintext
            }
        }
    }
}

pub trait Cypher where Self: Sync {
    fn blocksize(&self) -> usize;
    fn encrypt_block(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt_block(&self, cyphertext: &[u8]) -> Vec<u8>;
}
