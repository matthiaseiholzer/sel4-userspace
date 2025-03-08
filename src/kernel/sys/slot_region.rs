use sel4_us::runtime::kernel::SlotRegion;

use super::types::seL4_Word;
use core::clone::Clone;
use core::marker::Copy;
use core::prelude::rust_2024::derive;

pub type seL4_SlotPos = seL4_Word;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_SlotRegion {
    pub start: seL4_SlotPos, /* first CNode slot position OF region */
    pub end: seL4_SlotPos,   /* first CNode slot position AFTER region */
}

impl Into<SlotRegion> for seL4_SlotRegion {
    fn into(self) -> SlotRegion {
        SlotRegion {
            start: self.start,
            end: self.end,
        }
    }
}
