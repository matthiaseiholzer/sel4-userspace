use crate::print_str;
use crate::runtime::cap_space::cap::CapType;
use crate::runtime::cap_space::cap_rights::CapRights;
use crate::runtime::cap_space::CapAddr;
use crate::runtime::cap_space::CapSpaceManager;
use crate::runtime::kernel::{BootInfo, IPCBuffer, Kernel, MessageInfo};
use crate::runtime::protection_domain::thread::Thread;
use crate::runtime::protection_domain::ProtectionDomain;
use crate::runtime::va_space::VASpaceManager;
use core::mem::size_of;

pub fn untyped_server<K: Kernel>(thread: &mut Thread<K>, boot_info: &BootInfo) -> ! {
    let _ = thread.set_debug_name(&"untyped_srv\0");

    let kernel = thread.pd.kernel();
    // let mut untyped_memory_manager = UntypedMemoryManager::new(boot_info);

    // let cap_type: CapType = thread.identify_cap(CapSpaceManager::C_CSPACE_ROOT);
    // print_str!(thread, "Cap Type: {:?}\n", cap_type);

    // //kernel.delete(CapSpaceManager::C_CSPACE_ROOT, CapSpaceManager::C_CSPACE_ROOT);
    // let _ = kernel.copy(
    //     CapSpaceManager::C_CSPACE_ROOT,
    //     CapAddr::from(CapSpaceManager::C_ASID_OFFSET, 2),
    //     CapSpaceManager::C_CSPACE_ROOT,
    //     CapAddr::from(CapSpaceManager::C_ASID_OFFSET, 2),
    //     CapRights::default(),
    // );

    // let c_t0: CapAddr = CapSpaceManager::cap_addr_l012(0, 0, 450);
    // let c_t1: CapAddr = CapSpaceManager::cap_addr_l012(0, 0, 451);

    // let root_space_idx = CapSpaceManager::cap_addr_l012(0, 0, 500);

    // create_endpoint(kernel.clone(), &mut untyped_memory_manager);
    // {
    //     let tcb_cap_idx = CapSpaceManager::cap_addr_l012(0, 0, 501);
    // create_pd(
    //     kernel.clone(),
    //     boot_info,
    //     &mut untyped_memory_manager,
    //     root_space_idx,
    //     tcb_cap_idx,
    //     c_t0,
    //     c_t1,
    //     dummy_server,
    // );

    // create_dummy_server(
    //     kernel.clone(),
    //     untyped_memory_manager.clone(),
    //     root_space_idx,
    //     dummy_server_notification,
    // );
    //let _ = kernel.resume(tcb_cap_idx);
    //}

    loop {
        for i in 0..40_000_000 {}
        print_str!(thread, "Untyped Server\n");
        // let mut sender: usize = 0;
        // thread.receive(CapSpaceManager::C_MT_EP, &mut sender);

        // let res = read_message_from_ipc_buffer(&thread);

        // print_str!(thread, "{:?}\n", res);
        // thread.debug_dump_scheduler();
        // let addr = 0;
        // let message = ReincarnationServerMessage::AllocResponse(addr);

        // let msg_info = write_message_to_ipc_buffer(thread, &message);

        // thread.reply(&msg_info);
    }
}
