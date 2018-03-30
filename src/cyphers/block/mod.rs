use rayon::slice::ParallelSlice;
use rayon::iter::ParallelIterator;

pub mod aes;

pub enum OperationMode {
    ECB
}

pub struct BlockCypher<T: Cypher> {
    operation_mode: OperationMode,
    cypher: T
}

impl<T: Cypher> BlockCypher<T> {
    pub fn new(cypher: T, operation_mode: OperationMode) -> Self {
        BlockCypher {
            operation_mode: operation_mode,
            cypher: cypher
        }
    }
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        match self.operation_mode {
            OperationMode::ECB => plaintext
                .par_chunks(self.cypher.blocksize())
                .flat_map(|b| self.cypher.encrypt_block(b)).collect()
        }
    }
    pub fn decrypt(&self, cyphertext: &[u8]) -> Vec<u8> {
        match self.operation_mode {
            OperationMode::ECB => cyphertext
                .par_chunks(self.cypher.blocksize())
                .flat_map(|b| self.cypher.decrypt_block(b)).collect()
        }
    }
}

pub trait Cypher where Self: Sync {
    fn blocksize(&self) -> usize;
    fn encrypt_block(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt_block(&self, cyphertext: &[u8]) -> Vec<u8>;
}
