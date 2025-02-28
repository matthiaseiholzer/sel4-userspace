use core::clone::Clone;
use core::mem::size_of;
use core::prelude::rust_2024::derive;

#[derive(Clone, Copy, Default)]
#[repr(align(8))]
pub struct UntypedDesc {
    pub paddr: usize,  /* physical address of untyped cap  */
    pub size_bits: u8, /* size (2^n) bytes of each untyped */
    pub is_device: u8, /* whether the untyped is a device  */
    pub padding: [u8; size_of::<usize>() - 2 * size_of::<u8>()],
}
