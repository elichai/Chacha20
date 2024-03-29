#![allow(dead_code)]
#![no_std]
#![cfg_attr(feature = "nightly", feature(test))]
mod chacha;
mod matrix;
mod traits;

use crate::matrix::{chacha_20_rounds, Matrix};
use crate::traits::clear;

pub fn encrypt(plaintext: &mut [u8], key: [u8; 32], nonce: [u8; 12], mut ctr: u32) {
    assert!(ctr <= 1);
    let mut matrix = Matrix::from_params(key, nonce, &ctr);
    for chunk in plaintext.chunks_mut(64) {
        matrix.rewite(key, nonce, &ctr);
        matrix = chacha_20_rounds(matrix.clone());
        xor(chunk, matrix.as_le_bytes());
        ctr += 1;
    }
    clear(&mut ctr);
}

pub fn decrypt(ciphertext: &mut [u8], key: [u8; 32], nonce: [u8; 12], ctr: u32) {
    encrypt(ciphertext, key, nonce, ctr)
}

#[inline(always)]
pub(crate) fn xor(lhs: &mut [u8], rhs: &[u8]) {
    debug_assert!(lhs.len() <= rhs.len());
    for i in 0..lhs.len() {
        lhs[i] ^= rhs[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto::chacha20::ChaCha20;
    use crypto::symmetriccipher::SynchronousStreamCipher;

    #[test]
    fn test_encrypt_decrypt() {
        let cipher = [
            0x6e, 0x2e, 0x35, 0x9a, 0x25, 0x68, 0xf9, 0x80, 0x41, 0xba, 0x07, 0x28, 0xdd, 0x0d, 0x69, 0x81, 0xe9, 0x7e, 0x7a, 0xec,
            0x1d, 0x43, 0x60, 0xc2, 0x0a, 0x27, 0xaf, 0xcc, 0xfd, 0x9f, 0xae, 0x0b, 0xf9, 0x1b, 0x65, 0xc5, 0x52, 0x47, 0x33, 0xab,
            0x8f, 0x59, 0x3d, 0xab, 0xcd, 0x62, 0xb3, 0x57, 0x16, 0x39, 0xd6, 0x24, 0xe6, 0x51, 0x52, 0xab, 0x8f, 0x53, 0x0c, 0x35,
            0x9f, 0x08, 0x61, 0xd8, 0x07, 0xca, 0x0d, 0xbf, 0x50, 0x0d, 0x6a, 0x61, 0x56, 0xa3, 0x8e, 0x08, 0x8a, 0x22, 0xb6, 0x5e,
            0x52, 0xbc, 0x51, 0x4d, 0x16, 0xcc, 0xf8, 0x06, 0x81, 0x8c, 0xe9, 0x1a, 0xb7, 0x79, 0x37, 0x36, 0x5a, 0xf9, 0x0b, 0xbf,
            0x74, 0xa3, 0x5b, 0xe6, 0xb4, 0x0b, 0x8e, 0xed, 0xf2, 0x78, 0x5e, 0x42, 0x87, 0x4d,
        ];
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00];
        let mut data =
            *b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.";

        let plaintext = data.clone();
        encrypt(&mut data, key, nonce, 1);

        assert_eq!(&data[..], &cipher[..]);

        decrypt(&mut data, key, nonce, 1);
        assert_eq!(&data[..], &plaintext[..]);
    }

    #[test]
    fn test_against_crate() {
        let key = [0xab; 32];
        let nonce = [1u8; 12];
        let mut other_state = ChaCha20::new(&key, &nonce);
        let mut data = [0xCA; 1024];
        let mut output = [0u8; 1024];
        other_state.process(&data, &mut output);

        encrypt(&mut data, key, nonce, 0);

        assert_eq!(&data[..], &output[..]);
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    extern crate test;
    use self::test::{black_box, Bencher};
    use super::*;
    const MiB: usize = 1024 * 1024;

    #[bench]
    pub fn chacha20(bh: &mut Bencher) {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let mut data = [01u8; MiB];
        bh.iter(|| {
            encrypt(&mut data, key, nonce, 1);
            black_box(&data);
        });
    }
}
