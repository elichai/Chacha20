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

pub fn clear<T: Default>(obj: &mut T) {
    use core::{mem, ptr, sync::atomic};
    let zeroed = T::default();
    unsafe {
        let ptr = obj as *mut T;
        ptr::write_volatile(ptr, mem::zeroed());
        ptr::write_volatile(ptr, zeroed);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}
