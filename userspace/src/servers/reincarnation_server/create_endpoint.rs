// use crate::runtime::cap_space::{CapAddr, CapData, CapInit, CapSpaceManager};
// use crate::runtime::kernel::{Kernel, UntypedType};
// use crate::servers::root_server::untyped_memory_manager::UntypedMemoryManager;

// pub fn create_endpoint<K: Kernel>(_kernel: K, untyped_memory_manager: &mut UntypedMemoryManager) {
//     let cap_space_root_idx = CapAddr::from(1, 64);
//     let (untyped_cap, _) = untyped_memory_manager.alloc(30).unwrap();

//     // Endpoint
//     let cap = CapSpaceManager::cap_addr_l01(
//         CapSpaceManager::C_MT_EP_0_OFFSET,
//         CapSpaceManager::C_MT_EP_1_OFFSET,
//     );

//     // let _ = kernel.copy(
//     //     cap_space_root_idx,
//     //     CapAddr::from(5, 64),
//     //     cap_space_root_idx,
//     //     CapAddr::from(1, 64),
//     //     CapRights::default(),
//     // );

//     // let _ = kernel.retype(
//     //     untyped_cap,
//     //     UntypedType::EndpointObject,
//     //     0,
//     //     cap_space_root_idx,
//     //     //cap.addr,
//     //     //cap.depth,
//     //     0,
//     //     0,
//     //     5,//CapSpaceManager::C_V_MT_EP_2_OFFSET,
//     //     1,
//     // );
// }
