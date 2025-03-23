use crate::runtime::kernel::Kernel;
use crate::runtime::protection_domain::thread::Thread;

pub fn pd_manager<K: Kernel>(thread: &mut Thread<K>) -> ! {
    loop {}
}
