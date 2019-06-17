mod traits;

use traits::MutArithmetics;

use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq)]
struct Matrix(Vec<u32>);


impl Matrix {

    pub fn existing(matrix: [u32; 16]) -> Self {
        Matrix(matrix.to_vec())
    }

    fn quarter_round(&mut self, a: usize, b: usize, c: usize, d: usize) {
        let (a, b, c, d) = self.mut_four(a,b,c,d);
        a.wrapping_add_mut(*b); d.xor_mut(*a); d.rotate_left_mut(16);
        c.wrapping_add_mut(*d); b.xor_mut(*c); b.rotate_left_mut(12);
        a.wrapping_add_mut(*b); d.xor_mut(*a); d.rotate_left_mut(8);
        c.wrapping_add_mut(*d); b.xor_mut(*c); b.rotate_left_mut(7);
    }


    fn mut_four(&mut self, a: usize, b: usize, c: usize, d: usize) -> (&mut u32, &mut u32, &mut u32, &mut u32) {
        if (a == b) || (b == c) || (c == d) {
            panic!("Can't return more than one mut ref to the safe variable");
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quarter_round() {
        let mut m = Matrix(vec![0x11111111, 0x01020304, 0x9b8d6f43, 0x01234567]);
        m.quarter_round(0,1,2,3);
        assert_eq!(m, Matrix(vec![0xea2a92f4, 0xcb1cf8ce, 0x4581472e, 0x5881c4bb]));
    }

    #[test]
    fn test_state_round() {
        let mut m = Matrix::existing([
            0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0x2a5f714c,
            0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0x3d631689, 0x2098d9d6, 0x91dbd320,
            ]);
        let expected_res = Matrix::existing([
            0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a,
            0x44c20ef3, 0x3390af7f, 0xd9fc690b, 0xcfacafd2,
            0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963,
            0x5c971061, 0xccc07c79, 0x2098d9d6, 0x91dbd320,
        ]);

        m.quarter_round(2,7,8,13);

        assert_eq!(m, expected_res);

    }
}