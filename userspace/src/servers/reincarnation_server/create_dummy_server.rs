use crate::runtime::cap_space::cap_rights::CapRights;
use crate::runtime::cap_space::{CapAddr, CapData, CapInit, CapSpaceManager};
use crate::runtime::kernel::{Kernel, UntypedType};
use crate::servers::root_server::untyped_memory_manager::UntypedMemoryManager;
use core::default::Default;

pub fn create_dummy_server<K: Kernel>(
    mut kernel: K,
    mut untyped_memory_manager: UntypedMemoryManager,
    cap_space_root_idx: CapAddr,
    notification_idx: CapAddr,
) {
    let root_server_cspace_root = CapAddr::from(CapInit::InitThreadCNode as usize, 64);
    let (untyped_cap, _) = untyped_memory_manager.alloc(30).unwrap();

    // Endpoint
    let cap = CapSpaceManager::cap_addr_l01(
        CapSpaceManager::C_MT_EP_0_OFFSET,
        CapSpaceManager::C_MT_EP_1_OFFSET,
    );

    let _ = kernel.retype(
        untyped_cap,
        UntypedType::EndpointObject,
        0,
        cap_space_root_idx,
        cap.addr,
        cap.depth,
        CapSpaceManager::C_V_MT_EP_2_OFFSET,
        1,
    );

    let cap_ep = CapSpaceManager::cap_addr_l012(
        CapSpaceManager::C_MT_EP_0_OFFSET,
        CapSpaceManager::C_MT_EP_1_OFFSET,
        CapSpaceManager::C_V_MT_EP_2_OFFSET,
    );
    let rights = CapRights::default();
    let badge = CapData::default();
    let _ = kernel.copy(
        root_server_cspace_root,
        notification_idx,
        cap_space_root_idx,
        cap_ep,
        rights,
    );
}
