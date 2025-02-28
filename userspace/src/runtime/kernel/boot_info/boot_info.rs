use super::SlotRegion;
use crate::runtime::kernel::constants::CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS;
use crate::runtime::kernel::{IPCBuffer, UntypedDesc};
use core::clone::Clone;
use core::fmt::{Debug, Error, Formatter};
use core::format_args;
use core::prelude::rust_2024::derive;
use core::result::Result;

#[derive(Clone)]
pub struct BootInfo {
    pub extraLen: usize,           /* length of any additional bootinfo information */
    pub nodeID: usize,             /* ID [0..numNodes-1] of the seL4 node (0 if uniprocessor) */
    pub numNodes: usize,           /* number of seL4 nodes (1 if uniprocessor) */
    pub numIOPTLevels: usize,      /* number of IOMMU PT levels (0 if no IOMMU support) */
    pub ipcBuffer: *mut IPCBuffer, /* pointer to initial thread's IPC buffer */
    pub empty: SlotRegion,         /* empty slots (null caps) */
    pub sharedFrames: SlotRegion,  /* shared-frame caps (shared between seL4 nodes) */
    pub userImageFrames: SlotRegion, /* userland-image frame caps */
    pub userImagePaging: SlotRegion, /* userland-image paging structure caps */
    pub ioSpaceCaps: SlotRegion,   /* IOSpace caps for ARM SMMU */
    pub extraBIPages: SlotRegion, /* caps for any pages used to back the additional bootinfo information */
    pub initThreadCNodeSizeBits: usize, /* initial thread's root CNode size (2^n slots) */
    pub initThreadDomain: usize,  /* Initial thread's domain ID */
    pub untyped: SlotRegion,      /* untyped-object caps (untyped caps) */
    pub untypedList: [UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS], /* information about each untyped */
                                                                          /* the untypedList should be the last entry in this struct, in order
                                                                           * to make this struct easier to represent in other languages */
}

impl Debug for BootInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_fmt(format_args!("IOPRT levels: {}\n", self.numIOPTLevels))?;
        f.write_fmt(format_args!("IPC buffer: {:?}\n", self.ipcBuffer))?;
        f.write_fmt(format_args!(
            "Empty Slots: [{} --> {})\n",
            self.empty.start, self.empty.end
        ))?;
        f.write_fmt(format_args!(
            "sharedFrames: [{} --> {})\n",
            self.sharedFrames.start, self.sharedFrames.end
        ))?;
        f.write_fmt(format_args!(
            "userImageFrames: [{} --> {})\n",
            self.userImageFrames.start, self.userImageFrames.end
        ))?;
        f.write_fmt(format_args!(
            "initThreadCNodeSizeBits: {}\n",
            self.initThreadCNodeSizeBits
        ))?;
        f.write_fmt(format_args!(
            "userImagePaging: [{} --> {})\n",
            self.userImagePaging.start, self.userImagePaging.end
        ))?;
        f.write_fmt(format_args!(
            "untyped: [{} --> {})\n",
            self.untyped.start, self.untyped.end
        ))?;

        // f.write_fmt(format_args!("Memory:\n\n"))?;
        // for i in self.untyped.start..self.untyped.end {
        //     let idx = i - self.untyped.start;
        //     let untyped_desc = &self.untypedList[idx];
        //     if untyped_desc.is_device == 0 {
        //         let size_bits = untyped_desc.size_bits;
        //         let paddr = untyped_desc.paddr;

        //         f.write_fmt(format_args!(
        //             "cap: {}, phys addr: 0x{:x} size: {}\n",
        //             i, paddr, size_bits
        //         ))?;
        //     }
        // }

        // f.write_fmt(format_args!("Devices:\n\n"))?;
        // for i in self.untyped.start..self.untyped.end {
        //     let idx = i - self.untyped.start;
        //     let untyped_desc = &self.untypedList[idx];
        //     if untyped_desc.is_device != 0 {
        //         let size_bits = untyped_desc.size_bits;
        //         let paddr = untyped_desc.paddr;

        //         f.write_fmt(format_args!(
        //             "cap: {}, phys addr: 0x{:x} size: {}\n",
        //             i, paddr, size_bits
        //         ))?;
        //     }
        // }

        Result::Ok(())
    }
}
