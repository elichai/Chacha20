use crate::matrix::{chacha_20_rounds, Matrix};
use crate::traits::clear;

pub struct Chacha20 {
    ctr: u32,
    nonce: [u8; 12],
    key: [u8; 32],
}

impl Chacha20 {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Chacha20 { ctr: 0, nonce, key }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        self.encrypt(data)
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        let mut matrix = Matrix::from_params(self.key, self.nonce, &self.ctr);
        for chunk in data.chunks_mut(64) {
            matrix.rewite(self.key, self.nonce, &self.ctr);
            matrix = chacha_20_rounds(matrix.clone());
            crate::xor(chunk, matrix.as_le_bytes());
            self.ctr += 1;
        }
    }
}

impl Drop for Chacha20 {
    fn drop(&mut self) {
        clear(&mut self.ctr);
        clear(&mut self.nonce);
        clear(&mut self.key);
    }
}
