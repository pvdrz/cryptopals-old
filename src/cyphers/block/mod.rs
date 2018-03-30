use rayon::slice::ParallelSlice;
use rayon::iter::ParallelIterator;

pub mod aes;

pub enum OperationMode {
    ECB
}

pub trait BlockCypher where Self: Sync {
    fn blocksize(&self) -> usize;
    fn operation_mode(&self) -> &OperationMode;
    fn encrypt_block(&self, plaintext: &[u8]) -> Vec<u8>;
    fn decrypt_block(&self, cyphertext: &[u8]) -> Vec<u8>;
    fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        match self.operation_mode() {
            OperationMode::ECB => plaintext.par_chunks(self.blocksize())
                .flat_map(|b| self.encrypt_block(b)).collect()
        }
    }
    fn decrypt(&self, cyphertext: &[u8]) -> Vec<u8> {
        match self.operation_mode() {
            OperationMode::ECB => cyphertext.par_chunks(self.blocksize())
                .flat_map(|b| self.decrypt_block(b)).collect()
        }
    }
}
