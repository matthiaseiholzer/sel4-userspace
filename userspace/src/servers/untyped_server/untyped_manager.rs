use crate::runtime::cap_space::CapAddr;
use crate::runtime::kernel::{
    constants::CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS, BootInfo, SlotRegion, UntypedDesc,
};
use core::clone::Clone;
use core::option::Option;
use core::prelude::rust_2024::derive;

#[derive(Clone)]
#[repr(align(8))]
pub struct UntypedManager {
    pub untyped: SlotRegion, /* untyped-object caps (untyped caps) */
    pub untyped_list: [UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
}

// impl Default for UntypedManager {
//     fn default() -> Self {
//         UntypedMemoryManager{
//             untyped: SlotRegion::default(),
//             untyped_list: [UntypedDesc::default(); CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS]
//         }
//     }
// }

impl UntypedManager {
    pub fn new(boot_info: &BootInfo) -> UntypedManager {
        UntypedManager {
            untyped: boot_info.untyped.clone(),
            untyped_list: boot_info.untyped_list.clone(),
        }
    }

    pub fn alloc(&mut self, size_bits: u8) -> Option<(CapAddr, u8)> {
        let cap_start = self.untyped.start;
        let cap_end = self.untyped.end;

        for cap_idx in cap_start..cap_end {
            let s: &UntypedDesc = &self.untyped_list[cap_idx - cap_start];

            if s.is_device == 0 as u8 && s.size_bits >= size_bits {
                return Option::Some((CapAddr::from(cap_idx, 64), s.size_bits));
            }
        }

        Option::None
    }
}
