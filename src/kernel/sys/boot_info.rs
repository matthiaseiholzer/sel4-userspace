use super::{
    constants::CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS,
    ipc_buffer::seL4_IPCBuffer,
    slot_region::seL4_SlotRegion,
    types::{seL4_Domain, seL4_NodeId, seL4_Word},
    untypeddesc::seL4_UntypedDesc,
};

use core::marker::Copy;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct seL4_BootInfo {
    pub extraLen: seL4_Word, /* length of any additional bootinfo information */
    pub nodeID: seL4_NodeId, /* ID [0..numNodes-1] of the seL4 node (0 if uniprocessor) */
    pub numNodes: seL4_Word, /* number of seL4 nodes (1 if uniprocessor) */
    pub numIOPTLevels: seL4_Word, /* number of IOMMU PT levels (0 if no IOMMU support) */
    pub ipcBuffer: *mut seL4_IPCBuffer, /* pointer to initial thread's IPC buffer */
    pub empty: seL4_SlotRegion, /* empty slots (null caps) */
    pub sharedFrames: seL4_SlotRegion, /* shared-frame caps (shared between seL4 nodes) */
    pub userImageFrames: seL4_SlotRegion, /* userland-image frame caps */
    pub userImagePaging: seL4_SlotRegion, /* userland-image paging structure caps */
    pub ioSpaceCaps: seL4_SlotRegion, /* IOSpace caps for ARM SMMU */
    pub extraBIPages: seL4_SlotRegion, /* caps for any pages used to back the additional bootinfo information */
    pub initThreadCNodeSizeBits: seL4_Word, /* initial thread's root CNode size (2^n slots) */
    pub initThreadDomain: seL4_Domain, /* Initial thread's domain ID */
    pub untyped: seL4_SlotRegion,      /* untyped-object caps (untyped caps) */
    pub untypedList: [seL4_UntypedDesc; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS], /* information about each untyped */
                                                                               /* the untypedList should be the last entry in this struct, in order
                                                                                * to make this struct easier to represent in other languages */
}

extern "C" {
    pub fn sel4runtime_bootinfo() -> *const seL4_BootInfo;
}

#[repr(usize)]
pub enum CapInit {
    seL4_CapNull = 0,                 /* null cap */
    seL4_CapInitThreadTCB = 1,        /* initial thread's TCB cap */
    seL4_CapInitThreadCNode = 2,      /* initial thread's root CNode cap */
    seL4_CapInitThreadVSpace = 3,     /* initial thread's VSpace cap */
    seL4_CapIRQControl = 4,           /* global IRQ controller cap */
    seL4_CapASIDControl = 5,          /* global ASID controller cap */
    seL4_CapInitThreadASIDPool = 6,   /* initial thread's ASID pool cap */
    seL4_CapIOPortControl = 7,        /* global IO port control cap (null cap if not supported) */
    seL4_CapIOSpace = 8,              /* global IO space cap (null cap if no IOMMU support) */
    seL4_CapBootInfoFrame = 9,        /* bootinfo frame cap */
    seL4_CapInitThreadIPCBuffer = 10, /* initial thread's IPC buffer frame cap */
    seL4_CapDomain = 11,              /* global domain controller cap */
    seL4_CapSMMUSIDControl = 12,      /* global SMMU SID controller cap, null cap if not supported*/
    seL4_CapSMMUCBControl = 13,       /* global SMMU CB controller cap, null cap if not supported*/
    seL4_CapInitThreadSC = 14,        /* initial thread's scheduling context cap */
    seL4_NumInitialCaps = 15,
}
