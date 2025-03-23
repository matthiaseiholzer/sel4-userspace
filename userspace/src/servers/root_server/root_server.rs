use crate::runtime::cap_space::cap_rights::CapRights;
use crate::runtime::cap_space::{CapInit, CapSpaceManager};
use crate::servers::reincarnation_server::reincarnation_server;
use crate::servers::root_server::create_pd::create_pd;
use crate::servers::untyped_server::untyped_server;
use crate::{
    runtime::{
        cap_space::CapAddr,
        kernel::{BootInfo, Kernel},
    },
    servers::root_server::untyped_memory_manager::UntypedMemoryManager,
};

pub fn root_server<K: Kernel>(
    kernel: K,
    boot_info: &BootInfo,
    untyped_memory_manager: UntypedMemoryManager,
) -> ! {
    let c_t0: CapAddr = CapAddr::from(800, 64);
    let c_t1: CapAddr = CapAddr::from(801, 64);

    let reinc_space_idx = CapAddr::from(500, 64);
    let reinc_tcb_cap_idx = CapAddr::from(501, 64);
    let reinc_ep_cap_idx = CapAddr::from(502, 64);

    create_reincarnation_srv_pd(
        kernel.clone(),
        boot_info,
        untyped_memory_manager.clone(),
        c_t0,
        c_t1,
        reinc_space_idx,
        reinc_tcb_cap_idx,
        reinc_ep_cap_idx,
    );

    let untyped_space_idx = CapAddr::from(503, 64);
    let untyped_tcb_cap_idx = CapAddr::from(504, 64);
    let untyped_ep_cap_idx = CapAddr::from(505, 64);
    create_untyped_srv_pd(
        kernel.clone(),
        boot_info,
        untyped_memory_manager,
        c_t0,
        c_t1,
        untyped_space_idx,
        untyped_tcb_cap_idx,
        untyped_ep_cap_idx,
    );

    let src_ep_cap = CapSpaceManager::cap_addr_l012(
        CapSpaceManager::C_MT_EP_0_OFFSET,
        CapSpaceManager::C_MT_EP_1_OFFSET,
        CapSpaceManager::C_V_MT_EP_2_OFFSET,
    );

    let dest_ep_cap = CapSpaceManager::cap_addr_l012(
        CapSpaceManager::C_MT_EP_0_OFFSET,
        CapSpaceManager::C_MT_EP_1_OFFSET,
        CapSpaceManager::C_V_MT_EP_2_OFFSET + 1,
    );

    let _ = kernel.copy(
        reinc_space_idx,
        dest_ep_cap,
        untyped_space_idx,
        src_ep_cap,
        CapRights::default(),
    );

    loop {
        for _ in 0..40_000_000 {}
        kernel.dump_scheduler();
    }
}

fn create_reincarnation_srv_pd<K: Kernel>(
    kernel: K,
    boot_info: &BootInfo,
    untyped_memory_manager: UntypedMemoryManager,
    c_t0: CapAddr,
    c_t1: CapAddr,
    root_space_idx: CapAddr,
    tcb_cap_idx: CapAddr,
    ep_cap_idx: CapAddr,
) {
    {
        create_pd(
            kernel.clone(),
            boot_info,
            untyped_memory_manager.clone(),
            root_space_idx,
            tcb_cap_idx,
            ep_cap_idx,
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
        let _ = kernel.resume(tcb_cap_idx);
    }
}

fn create_untyped_srv_pd<K: Kernel>(
    kernel: K,
    boot_info: &BootInfo,
    untyped_memory_manager: UntypedMemoryManager,
    c_t0: CapAddr,
    c_t1: CapAddr,
    root_space_idx: CapAddr,
    tcb_cap_idx: CapAddr,
    ep_cap_idx: CapAddr,
) {
    {
        create_pd(
            kernel.clone(),
            boot_info,
            untyped_memory_manager.clone(),
            root_space_idx,
            tcb_cap_idx,
            ep_cap_idx,
            c_t0,
            c_t1,
            untyped_server,
        );

        {
            // Copy Cnode Cap to new cap space, therewit new pd can manipulate its onw cap space
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
        let _ = kernel.resume(tcb_cap_idx);
    }
}
