use super::types::{seL4_Uint8, seL4_Word};
use core::clone::Clone;
use core::mem::size_of;
use core::prelude::rust_2024::derive;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct seL4_UntypedDesc {
    pub paddr: seL4_Word,     /* physical address of untyped cap  */
    pub sizeBits: seL4_Uint8, /* size (2^n) bytes of each untyped */
    pub isDevice: seL4_Uint8, /* whether the untyped is a device  */
    pub padding: [seL4_Uint8; size_of::<seL4_Word>() - 2 * size_of::<seL4_Uint8>()],
}
