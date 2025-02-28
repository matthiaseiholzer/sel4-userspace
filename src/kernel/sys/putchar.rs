use core::ffi::{c_size_t, c_void};

extern "C" {
    pub fn kernel_putchar_write(data: *const c_void, count: c_size_t) -> c_size_t;

    pub fn seL4_RISCV_PageTableValue() -> usize;
}
