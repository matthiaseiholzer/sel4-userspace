use core::clone::Clone;
use core::prelude::rust_2024::derive;

#[derive(Clone, Default)]
pub struct SlotRegion {
    pub start: usize, /* first CNode slot position OF region */
    pub end: usize,   /* first CNode slot position AFTER region */
}
