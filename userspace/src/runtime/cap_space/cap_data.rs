use core::default::Default;

use core::clone::Clone;
use core::convert::Into;
use core::prelude::rust_2024::derive;

#[derive(Clone)]
pub struct CapData {
    pub guard: usize,
    pub guard_size: u8,
}
impl CapData {
    pub fn new(guard: usize, guard_size: u8) -> CapData {
        CapData { guard, guard_size }
    }
}
impl Default for CapData {
    fn default() -> Self {
        CapData {
            guard: 0,
            guard_size: 0,
        }
    }
}

// -- CNode cap data
// block seL4_CNode_CapData {
//     field guard 58
//     field guardSize 6
// }
// | (guard & 0x3ffffffffffffffull) << 6
// | (guardSize & 0x3full) << 0;
impl Into<usize> for CapData {
    fn into(self: Self) -> usize {
        let l_guard = self.guard & ((1 << 58) - 1);
        let l_guard_size = self.guard_size & ((1 << 6) - 1);
        (l_guard << 6) | (l_guard_size as usize)
    }
}
