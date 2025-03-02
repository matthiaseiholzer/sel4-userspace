use crate::runtime::cap_space::cap_rights::CapRights;
use crate::runtime::cap_space::{CapInit, CapSpaceManager};
use crate::servers::reincarnation_server::reincarnation_server::reincarnation_server;
use crate::servers::root_server::create_reincarnation_server;
use crate::{
    print_str,
    runtime::{
        cap_space::CapAddr,
        kernel::{BootInfo, Kernel},
    },
    servers::root_server::{create_pd::create_pd, untyped_memory_manager::UntypedMemoryManager},
};

pub fn root_server<K: Kernel>(
    kernel: K,
    boot_info: &BootInfo,
    untyped_memory_manager: UntypedMemoryManager,
) -> ! {
    let c_t0: CapAddr = CapAddr::from(800, 64);
    let c_t1: CapAddr = CapAddr::from(801, 64);

    {
        let root_space_idx = CapAddr::from(500, 64);
        let tcb_cap_idx = CapAddr::from(501, 64);
        create_pd(
            kernel.clone(),
            boot_info,
            untyped_memory_manager.clone(),
            root_space_idx,
            tcb_cap_idx,
            c_t0,
            c_t1,
            reincarnation_server,
        );

        {
            // Copy Cnode Cap to new cap space, therewith reincarnation server can manipulate its onw cap space
            let root_server_cspace_root = CapAddr::from(CapInit::InitThreadCNode as usize, 64);
            {
                let dest_cap = CapSpaceManager::C_CSPACE_ROOT;
                let _ = kernel.copy(
                    root_space_idx,
                    CapAddr::from(dest_cap.addr, 36),
                    root_server_cspace_root,
                    root_space_idx,
                    CapRights::default(),
                );
            }

            {
                let dest_cap = CapSpaceManager::C_ASID_POOL;
                let src_cap = CapInit::InitThreadASIDPool;
                let _ = kernel.copy(
                    root_space_idx,
                    CapAddr::from(dest_cap.addr, 36),
                    root_server_cspace_root,
                    CapAddr::from(src_cap as usize, 64),
                    CapRights::default(),
                );
            }
        }
        //let _ = kernel.resume(tcb_cap_idx);
    }

    loop {
        let _ = kernel.suspend(CapAddr::from_const(CapInit::InitThreadTCB as usize));
    }
}
