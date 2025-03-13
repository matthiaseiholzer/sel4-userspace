use super::sys::boot_info::seL4_BootInfo;
use sel4_us::runtime::kernel::{
    constants::CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS, BootInfo, IPCBuffer, UntypedDesc,
};

impl From<seL4_BootInfo> for BootInfo {
    // Required method
    fn from(value: seL4_BootInfo) -> Self {
        let untyped_list: [UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS] =
            value.untypedList.map(|a| a.into());
        BootInfo {
            extra_len: value.extraLen,
            node_id: value.nodeID,
            num_nodes: value.numNodes,
            num_iopt_levels: value.numIOPTLevels,
            ipc_buffer: value.ipcBuffer as *mut IPCBuffer,
            empty: value.empty.into(),
            shared_frames: value.sharedFrames.into(),
            user_image_frames: value.userImageFrames.into(),
            user_image_paging: value.userImagePaging.into(),
            io_space_caps: value.ioSpaceCaps.into(),
            extra_bi_pages: value.extraBIPages.into(),
            init_thread_cnode_size_bits: value.initThreadCNodeSizeBits,
            init_thread_domain: value.initThreadDomain,
            untyped: value.untyped.into(),
            untyped_list: untyped_list,
        }
    }
}
