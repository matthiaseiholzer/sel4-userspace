use sel4_userspace::runtime::kernel::UntypedDesc;

use super::sys::untypeddesc::seL4_UntypedDesc;

impl From<seL4_UntypedDesc> for UntypedDesc {
    // Required method
    fn from(value: seL4_UntypedDesc) -> Self {
        UntypedDesc {
            paddr: value.paddr,
            size_bits: value.sizeBits,
            is_device: value.isDevice,
            padding: value.padding,
        }
    }
}
