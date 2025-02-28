use sel4_userspace::runtime::va_space::page::Attributes;

use super::sys::types::seL4_RISCV_VMAttributes;

impl Into<seL4_RISCV_VMAttributes> for Attributes {
    fn into(self) -> seL4_RISCV_VMAttributes {
        match self {
            Self::Default => seL4_RISCV_VMAttributes::seL4_RISCV_Default_VMAttributes,
            Self::ExecuteNever => seL4_RISCV_VMAttributes::seL4_RISCV_ExecuteNever,
        }
    }
}
