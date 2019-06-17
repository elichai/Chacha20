pub trait MutArithmetics {
    fn wrapping_add_mut(&mut self, rhs: u32);
    fn rotate_left_mut(&mut self, n: u32);
    fn xor_mut(&mut self, rhs: u32);
}


impl MutArithmetics for u32 {
    #[inline(always)]
    fn wrapping_add_mut(&mut self, rhs: u32) {
        *self = self.wrapping_add(rhs);
    }

    #[inline(always)]
    fn rotate_left_mut(&mut self, n: u32) {
        *self = self.rotate_left(n)
    }

    #[inline(always)]
    fn xor_mut(&mut self, n: u32) {
        *self ^= n;
    }
}


