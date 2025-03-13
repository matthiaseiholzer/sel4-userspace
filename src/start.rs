use crate::kernel::sys::boot_info::sel4runtime_bootinfo;
use crate::sel4_kernel::SeL4Kernel;
use sel4_us::runtime::kernel::BootInfo;
use sel4_us::servers::root_server::root_server;
use sel4_us::servers::root_server::untyped_memory_manager::UntypedMemoryManager;

#[no_mangle]
pub extern "C" fn main() -> ! {
    const KERNEL: SeL4Kernel = SeL4Kernel::default();

    let sel4_boot_info = unsafe { *sel4runtime_bootinfo() }.clone();
    let boot_info = BootInfo::from(sel4_boot_info.clone());
    
    let untyped_memory_manager = UntypedMemoryManager::new(&sel4_boot_info.clone().into());

    let _ = root_server(KERNEL, &boot_info, untyped_memory_manager);
}
