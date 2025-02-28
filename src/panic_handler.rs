use crate::kernel::sys::putchar::kernel_putchar_write;
use core::ffi::c_void;
use core::fmt;
use core::fmt::Result;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::writeln;
pub struct PanicHandler {}

impl fmt::Write for PanicHandler {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe {
            kernel_putchar_write(s.as_ptr() as *const c_void, s.len());
        }

        Result::Ok(())
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let _res = writeln!(PanicHandler {}, "*** Panic ***\n {:#?}", info);
    loop {}
}
