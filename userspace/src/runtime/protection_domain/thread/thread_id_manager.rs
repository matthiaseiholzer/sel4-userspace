use core::mem::size_of;

use super::Id;

pub struct ThreadIdManager {
    pub threads_used: [usize; Self::NUM_USIZE],
}

impl ThreadIdManager {
    pub const MAX_NUM_THREADS: usize = 128;
    const BITS_USIZE: usize = (size_of::<usize>() * 8);
    const NUM_USIZE: usize = Self::MAX_NUM_THREADS / Self::BITS_USIZE;
}

impl Default for ThreadIdManager {
    fn default() -> Self {
        ThreadIdManager {
            threads_used: [0; Self::NUM_USIZE],
        }
    }
}

impl ThreadIdManager {
    pub fn alloc_id(&mut self) -> Result<Id, ()> {
        for i in 0..Self::NUM_USIZE {
            if self.threads_used[i] != !0 {
                for j in 0..Self::BITS_USIZE {
                    let bit_v = (self.threads_used[i] & (1 << j)) >> j;
                    if bit_v == 0 {
                        let bit = (i * Self::BITS_USIZE + j) as u16;
                        self.set_bit(bit, true);
                        return Ok(bit as Id);
                    }
                }
            }
        }

        Err(())
    }

    pub fn free_id(&mut self, id: Id) {
        self.set_bit(id, false);
    }

    fn set_bit(&mut self, bit: u16, value: bool) {
        let idx = bit as usize / Self::BITS_USIZE;
        let offset = bit as usize % Self::BITS_USIZE;
        self.threads_used[idx] =
            (self.threads_used[idx] & !(1 << offset)) | ((value as usize) << offset)
    }
}
