use crate::traits::{clear, MutArithmetics};

use core::ops::{AddAssign, Index, IndexMut};
use core::{fmt, mem, slice};

const U32_SIZE: usize = mem::size_of::<u32>();
const U32_ALIGN: usize = mem::align_of::<u32>();

pub fn chacha_20_rounds(matrix_before: Matrix) -> Matrix {
    let mut matrix_after = chacha_20_rounds_internal(matrix_before.clone());

    matrix_after += matrix_before;

    matrix_after
}

fn chacha_20_rounds_internal(mut matrix: Matrix) -> Matrix {
    for _ in 0..10 {
        // column rounds
        matrix.quarter_round(0, 4, 8, 12);
        matrix.quarter_round(1, 5, 9, 13);
        matrix.quarter_round(2, 6, 10, 14);
        matrix.quarter_round(3, 7, 11, 15);
        // diagonal rounds
        matrix.quarter_round(0, 5, 10, 15);
        matrix.quarter_round(1, 6, 11, 12);
        matrix.quarter_round(2, 7, 8, 13);
        matrix.quarter_round(3, 4, 9, 14);
    }

    matrix
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix([u32; 16]);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, byte) in self.0.iter().enumerate() {
            write!(f, "{:08x}  ", byte)?;
            if (i + 1) % 4 == 0 {
                writeln!(f, "")?;
            }
        }
        Ok(())
    }
}

impl Matrix {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_params(key: [u8; 32], nonce: [u8; 12], ctr: &u32) -> Self {
        let mut matrix = Self::new();
        matrix.set_key_u8(key).unwrap(); // I know that it's the right length
        matrix.set_nonce_u8(nonce).unwrap(); // I know that it's the right length
        matrix.set_ctr(ctr);
        matrix
    }

    pub fn rewite(&mut self, key: [u8; 32], nonce: [u8; 12], ctr: &u32) {
        self.set_constants();
        self.set_key_u8(key).unwrap();
        self.set_nonce_u8(nonce).unwrap();
        self.set_ctr(ctr);
    }

    pub fn set_constants(&mut self) {
        self[0] = 0x61707865;
        self[1] = 0x3320646e;
        self[2] = 0x79622d32;
        self[3] = 0x6b206574;
    }

    pub fn existing(matrix: [u32; 16]) -> Self {
        Matrix(matrix)
    }

    pub fn set_key(&mut self, key: &[u32]) {
        assert_eq!(key.len(), 8);
        self[4] = key[0];
        self[5] = key[1];
        self[6] = key[2];
        self[7] = key[3];
        self[8] = key[4];
        self[9] = key[5];
        self[10] = key[6];
        self[11] = key[7];
    }

    pub fn set_key_u8(&mut self, mut key: [u8; 32]) -> Option<()> {
        self.set_key(slice_u8_to_u32(&mut key)?);
        clear(&mut key);
        Some(())
    }

    pub fn set_nonce(&mut self, nonce: &[u32]) -> Option<()> {
        assert_eq!(nonce.len(), 3);
        self[13] = nonce[0];
        self[14] = nonce[1];
        self[15] = nonce[2];

        Some(())
    }

    pub fn set_nonce_u8(&mut self, mut nonce: [u8; 12]) -> Option<()> {
        self.set_nonce(slice_u8_to_u32(&mut nonce)?);
        clear(&mut nonce);
        Some(())
    }

    pub fn set_ctr(&mut self, ctr: &u32) {
        self[12] = *ctr;
    }

    #[rustfmt::skip]
    pub fn quarter_round(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let (a, b, c, d) = self.mut_four(a, b, c, d);
        a.wrapping_add_mut(*b); d.xor_mut(*a); d.rotate_left_mut(16);
        c.wrapping_add_mut(*d); b.xor_mut(*c); b.rotate_left_mut(12);
        a.wrapping_add_mut(*b); d.xor_mut(*a); d.rotate_left_mut(8);
        c.wrapping_add_mut(*d); b.xor_mut(*c); b.rotate_left_mut(7);
    }

    fn mut_four(&mut self, a: usize, b: usize, c: usize, d: usize) -> (&mut u32, &mut u32, &mut u32, &mut u32) {
        if (a == b) || (b == c) || (c == d) {
            panic!("Can't return more than one mut ref to the same variable");
        }
        unsafe {
            (
                &mut *(&mut self[a] as *mut _),
                &mut *(&mut self[b] as *mut _),
                &mut *(&mut self[c] as *mut _),
                &mut *(&mut self[d] as *mut _),
            )
        }
    }

    pub fn as_native_bytes(&self) -> &[u8] {
        assert_eq!(self.0.len(), 16);
        let ptr = self.0.as_ptr() as _;
        unsafe { slice::from_raw_parts(ptr, 16 * U32_SIZE) }
    }

    pub fn as_le_bytes(&mut self) -> &[u8] {
        memory_be_to_le(&mut self.0);
        self.as_native_bytes()
    }
}

#[inline(always)]
pub fn memory_be_to_le(_slice: &mut [u32]) {
    #[cfg(not(target_endian = "little"))]
    {
        for byte in _slice.iter_mut() {
            *byte = byte.swap_bytes();
        }
    }
}

fn slice_u8_to_u32(orig: &mut [u8]) -> Option<&[u32]> {
    let ptr = orig.as_ptr();
    if orig.len() % 4 != 0 || ptr as usize % U32_ALIGN != 0 {
        return None;
    }
    let res = unsafe { slice::from_raw_parts_mut(ptr as *mut u32, orig.len() / U32_SIZE) };
    memory_be_to_le(res);
    // Need to flip the bytes if it's not little endian.

    Some(res)
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::existing([
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
        ])
    }
}

impl Index<usize> for Matrix {
    type Output = u32;
    #[inline]
    fn index(&self, indx: usize) -> &Self::Output {
        &self.0[indx]
    }
}

impl IndexMut<usize> for Matrix {
    #[inline]
    fn index_mut(&mut self, indx: usize) -> &mut Self::Output {
        &mut self.0[indx]
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, other: Self) {
        assert_eq!(self.0.len(), other.0.len());
        for (s, o) in self.0.iter_mut().zip(other.0.iter()) {
            s.wrapping_add_mut(*o);
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        clear(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_round() {
        #[rustfmt::skip]
        let mut m = Matrix::existing([
            0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0x2a5f714c,
            0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0x3d631689, 0x2098d9d6, 0x91dbd320,
        ]);
        #[rustfmt::skip]
        let expected_res = Matrix::existing([
            0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0xcfacafd2,
            0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0xccc07c79, 0x2098d9d6, 0x91dbd320,
        ]);

        m.quarter_round(2, 7, 8, 13);

        assert_eq!(m, expected_res);
    }

    #[test]
    fn test_encrypt_block2() {
        #[rustfmt::skip]
        let matrix_before = Matrix::existing([
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c,
            0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
            0x00000001, 0x00000000, 0x4a000000, 0x00000000,
        ]);
        #[rustfmt::skip]
        let matrix_after = Matrix::existing([
            0xf3514f22, 0xe1d91b40, 0x6f27de2f, 0xed1d63b8,
            0x821f138c, 0xe2062c3d, 0xecca4f7e, 0x78cff39e,
            0xa30a3b8a, 0x920a6072, 0xcd7479b5, 0x34932bed,
            0x40ba4c79, 0xcd343ec6, 0x4c2c21ea, 0xb7417df0,
        ]);

        assert_eq!(chacha_20_rounds(matrix_before), matrix_after);
    }

    #[test]
    fn test_block_creation() {
        let res = Matrix::existing([
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514,
            0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x00000000, 0x4a000000, 0x00000000,
        ]);
        let key = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13,
            0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];
        let nonce = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00];
        let ctr = 1;

        assert_eq!(Matrix::from_params(key, nonce, &ctr), res);
    }

    #[test]
    fn test_encrypt_block() {
        #[rustfmt::skip]
            let after_setup = Matrix::existing([
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
            0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c,
            0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c,
            0x00000001, 0x09000000, 0x4a000000, 0x00000000,
        ]);
        #[rustfmt::skip]
            let after_rounds = Matrix::existing([
            0x837778ab, 0xe238d763, 0xa67ae21e, 0x5950bb2f,
            0xc4f2d0c7, 0xfc62bb2f, 0x8fa018fc, 0x3f5ec7b7,
            0x335271c2, 0xf29489f3, 0xeabda8fc, 0x82e46ebd,
            0xd19c12b4, 0xb04e16de, 0x9e83d0cb, 0x4e3c50a2,
        ]);
        #[rustfmt::skip]
            let finished = Matrix::existing([
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3,
            0xc7f4d1c7, 0x0368c033, 0x9aaa2204, 0x4e6cd4c3,
            0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9,
            0xd19c12b5, 0xb94e16de, 0xe883d0cb, 0x4e3c50a2,
        ]);
        let serialized = [
            0x10, 0xf1, 0xe7, 0xe4, 0xd1, 0x3b, 0x59, 0x15, 0x50, 0x0f, 0xdd, 0x1f, 0xa3, 0x20, 0x71, 0xc4, 0xc7, 0xd1, 0xf4, 0xc7,
            0x33, 0xc0, 0x68, 0x03, 0x04, 0x22, 0xaa, 0x9a, 0xc3, 0xd4, 0x6c, 0x4e, 0xd2, 0x82, 0x64, 0x46, 0x07, 0x9f, 0xaa, 0x09,
            0x14, 0xc2, 0xd7, 0x05, 0xd9, 0x8b, 0x02, 0xa2, 0xb5, 0x12, 0x9c, 0xd1, 0xde, 0x16, 0x4e, 0xb9, 0xcb, 0xd0, 0x83, 0xe8,
            0xa2, 0x50, 0x3c, 0x4e,
        ];
        let mut matrix = Matrix::default();
        matrix
            .set_key_u8([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
                0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ])
            .unwrap();

        matrix.set_nonce_u8([0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00]).unwrap();
        matrix.set_ctr(&1);

        assert_eq!(after_setup, matrix);

        let mut matrix = chacha_20_rounds_internal(matrix);
        assert_eq!(after_rounds, matrix);

        matrix += after_setup;

        assert_eq!(finished, matrix);
        assert_eq!(matrix.as_le_bytes(), &serialized[..]);
    }
}
