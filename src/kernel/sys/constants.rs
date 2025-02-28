use super::types::seL4_Uint8;

pub const seL4_MsgLengthBits: usize = 7;
pub const seL4_MsgMaxLength: usize = 120;
pub const seL4_MsgExtraCapBits: usize = 2;
pub const seL4_MsgMaxExtraCaps: usize = (1usize << seL4_MsgExtraCapBits) - 1;

pub const CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS: usize = 230;
pub const CONFIG_MAX_NUM_BOOTINFO_DEVICE_REGIONS: usize = 199;

pub const seL4_WordBits: seL4_Uint8 = 64;
