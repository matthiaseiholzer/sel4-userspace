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
            extraLen: value.extraLen,
            nodeID: value.nodeID,
            numNodes: value.numNodes,
            numIOPTLevels: value.numIOPTLevels,
            ipcBuffer: value.ipcBuffer as *mut IPCBuffer,
            empty: value.empty.into(),
            sharedFrames: value.sharedFrames.into(),
            userImageFrames: value.userImageFrames.into(),
            userImagePaging: value.userImagePaging.into(),
            ioSpaceCaps: value.ioSpaceCaps.into(),
            extraBIPages: value.extraBIPages.into(),
            initThreadCNodeSizeBits: value.initThreadCNodeSizeBits,
            initThreadDomain: value.initThreadDomain,
            untyped: value.untyped.into(),
            untypedList: untyped_list,
        }
    }
}
