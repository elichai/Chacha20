#![allow(dead_code)]

mod matrix;
mod traits;

use crate::matrix::Matrix;

fn chacha_20_rounds(matrix_before: Matrix) -> Matrix {
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

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut matrix = Matrix::default();
        matrix
            .set_key_u8([
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12,
                0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
            ])
            .unwrap();

        matrix.set_nonce_u8([0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00]).unwrap();
        matrix.set_ctr(1);

        assert_eq!(after_setup, matrix);

        let mut matrix = chacha_20_rounds_internal(matrix);
        assert_eq!(after_rounds, matrix);

        matrix += after_setup;

        assert_eq!(finished, matrix);
    }
}
