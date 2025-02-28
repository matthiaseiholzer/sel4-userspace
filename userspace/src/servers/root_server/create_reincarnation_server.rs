use crate::runtime::{
    cap_space::{cap_rights::CapRights, CapAddr, CapData, CapInit, CapSpaceManager},
    kernel::Kernel,
};

pub fn create_reincarnation_server<K: Kernel>(
    mut kernel: K,
    cap_space_root_idx: CapAddr,
    notification_idx: CapAddr,
) {
    let root_server_cspace_root = CapAddr::from(CapInit::InitThreadCNode as usize, 64);

    // Endpoint
    let cap_ep = CapAddr {
        addr: CapSpaceManager::C_MT_EP.addr,
        depth: CapSpaceManager::CSPACE_WIDTH_BITS,
    };

    let rights = CapRights::default();
    let badge = CapData::new(0, 0);

    let _ = kernel.mint(
        cap_space_root_idx,
        cap_ep,
        root_server_cspace_root,
        notification_idx,
        rights,
        badge,
    );
}
