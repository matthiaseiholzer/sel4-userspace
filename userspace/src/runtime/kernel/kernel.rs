use super::{error::Error, UserContext};
use crate::runtime::{
    cap_space::{cap_rights::CapRights, CapAddr, CapData},
    va_space::page::Attributes,
};

use core::result::Result;

//     seL4_SysCall = -1,
//        seL4_SysReplyRecv = -2,
//        seL4_SysSend = -3,
//        seL4_SysNBSend = -4,
//        seL4_SysRecv = -5,
//        seL4_SysReply = -6,
//        seL4_SysYield = -7,
//        seL4_SysNBRecv = -8,
// #if defined(CONFIG_PRINTING)
//        seL4_SysDebugPutChar = -9,
//        seL4_SysDebugDumpScheduler = -10,
// #endif /* defined(CONFIG_PRINTING) */
// #if defined(CONFIG_DEBUG_BUILD)
//        seL4_SysDebugHalt = -11,
//        seL4_SysDebugCapIdentify = -12,
//        seL4_SysDebugSnapshot = -13,
//        seL4_SysDebugNameThread = -14,

pub trait Kernel: Clone + CNode + TCB + Untyped + PageTable + Page + ASIDPool {
    fn sys_send_null(&self, sys: usize, src: usize, info_arg: usize);

    fn sys_recv(
        &self,
        sys: usize,
        src: usize,
        out_badge: &mut usize,
        out_info: &mut usize,
        out_mr0: &mut usize,
        out_mr1: &mut usize,
        out_mr2: &mut usize,
        out_mr3: &mut usize,
        reply: usize,
    );

    fn sys_send_recv(
        &self,
        sys: usize,
        dest: usize,
        out_badge: &mut usize,
        info_arg: usize,
        out_info: &mut usize,
        in_out_mr0: &mut usize,
        in_out_mr1: &mut usize,
        in_out_mr2: &mut usize,
        int_out_mr3: &mut usize,
        reply: usize,
    );

    fn sys_reply(
        &self,
        sys: usize,
        info_arg: usize,
        mr0: usize,
        mr1: usize,
        mr2: usize,
        mr3: usize,
    );

    fn debug_cap_identify(&self, cap: usize) -> usize;
    
    fn dump_scheduler(&self);
}

pub trait CNode {
    fn copy(
        &self,
        dest_cspace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
        rights: CapRights,
    ) -> Result<(), Error>;

    fn r#move(
        &self,
        dest_cpace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
    ) -> Result<(), Error>;

    /*
        seL4_CNode 	_service 	CPtr to the CNode that forms the root of the destination CSpace. Must be at a depth equivalent to the wordsize.
    seL4_Word 	dest_index 	CPtr to the destination slot. Resolved from the root of the destination CSpace.
    seL4_Uint8 	dest_depth 	Number of bits of dest_index to resolve to find the destination slot.
    seL4_CNode 	src_root 	CPtr to the CNode that forms the root of the source CSpace. Must be at a depth equivalent to the wordsize.
    seL4_Word 	src_index 	CPtr to the source slot. Resolved from the root of the source CSpace.
    seL4_Uint8 	src_depth 	Number of bits of src_index to resolve to find the source slot.
    seL4_Word 	badge
        */
    fn mutate(
        &self,
        service: CapAddr,
        dest_index: usize,
        dest_depth: u8,
        src_root: CapAddr,
        src_index: usize,
        src_depth: u8,
        badge: CapData,
    ) -> Result<(), Error>;

    /// seL4_CNode 	_service 	CPtr to the CNode that forms the root of the destination CSpace. Must be at a depth equivalent to the wordsize.
    /// seL4_Word 	dest_index 	CPtr to the destination slot. Resolved from the root of the destination CSpace.
    /// seL4_Uint8 	dest_depth 	Number of bits of dest_index to resolve to find the destination slot.
    /// seL4_CNode 	src_root 	CPtr to the CNode that forms the root of the source CSpace. Must be at a depth equivalent to the wordsize.
    /// seL4_Word 	src_index 	CPtr to the source slot. Resolved from the root of the source CSpace.
    /// seL4_Uint8 	src_depth 	Number of bits of src_index to resolve to find the source slot.
    /// seL4_CapRights_t 	rights 	The rights inherited by the new capability.
    /// seL4_Word 	badge 	Badge or guard to be applied to the new capability. For badges on 32-bit platforms, the high 4 bits are ignored.
    fn mint(
        &self,
        dest_cspace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
        rights: CapRights,
        badge: CapData,
    ) -> Result<(), Error>;

    ///_service CPTR to the CNode at the root of the CSpace where
    /// the capability will be found. Must be at a depth
    /// equivalent to the wordsize.
    /// seL4_Word index CPTR to the capability. Resolved from the root of
    /// the _service parameter.
    /// seL4_Uint8 depth Number of bits of index to resolve to find the capa-
    /// bility being operated on.
    fn delete(&self, service: CapAddr, cap: CapAddr) -> Result<(), Error>;
}

pub trait TCB {
    fn suspend(&self, tcb: CapAddr) -> Result<(), Error>;

    fn resume(&self, tcb: CapAddr) -> Result<(), Error>;

    fn set_priority(&self, tcb: CapAddr, authority: CapAddr, priority: u8) -> Result<(), Error>;

    fn read_registers(&self) -> Result<UserContext, Error>;

    fn write_registers(&self, tcb: CapAddr, registers: &UserContext) -> Result<(), Error>;

    fn configure(
        &self,
        tcb: CapAddr,
        fault_ep: CapAddr,
        cspace_root: CapAddr,
        cspace_root_data: CapData,
        vspace_root: CapAddr,
        vspace_root_data: usize,
        buffer: usize,
        buffer_frame: CapAddr,
    ) -> Result<(), Error>;

    fn set_space(
        &self,
        _service: CapAddr,
        fault_ep: CapAddr,
        cspace_root: CapAddr,
        cspace_root_data: CapData,
        vspace_root: CapAddr,
        vspace_root_data: usize,
    ) -> Result<(), Error>;

    fn set_ipc_buffer(
        &self,
        tcb: CapAddr,
        buffer: usize,
        buffer_frame: CapAddr,
    ) -> Result<(), Error>;
}
pub enum UntypedType {
    UntypedObject = 0,
    TCBObject = 1,
    EndpointObject = 2,
    NotificationObject = 3,
    CapTableObject = 4,
    Riscv4KPage = 6,
    RiscvMegaPage = 7,
    RiscvPageTableObject = 8,
}

pub trait Untyped {
    fn retype(
        &self,
        service: CapAddr,
        r#type: UntypedType,
        size_bits: u8,
        root: CapAddr,
        node_index: usize,
        node_depth: u8,
        node_offset: usize,
        num_objects: usize,
    ) -> Result<(), Error>;
}

pub trait PageTable {
    /// seL4_RISCV_PageTable 	_service 	Capability to the page table to invoke.
    /// seL4_RISCV_PageTable 	vspace 	VSpace to map the lower-level page table into.
    /// seL4_Word 	vaddr 	Virtual address at which to map the page table.
    /// seL4_RISCV_VMAttributes 	attr 	VM Attributes for the mapping
    fn map_page_table(
        &self,
        service: CapAddr,
        vspace: CapAddr,
        vaddr: usize,
        attr: Attributes,
    ) -> Result<(), Error>;
}

pub trait Page {
    /// seL4_RISCV_Page 	_service 	Capability to the page to invoke.
    /// seL4_RISCV_PageTable 	vspace 	VSpace to map the page into.
    /// seL4_Word 	vaddr 	Virtual address at which to map the page.
    /// SEL4_CAPRIGHTS_T 	RIGHTS 	RIGHTS FOR THE MAPPING.
    /// SEL4_RISCV_VMATTRIBUTES 	ATTR
    fn map_page(
        &self,
        service: CapAddr,
        vspace: CapAddr,
        vaddr: usize,
        rights: CapRights,
        attr: Attributes,
    ) -> Result<(), Error>;

    fn unmap_page(&self, service: CapAddr) -> Result<(), Error>;
}

pub trait ASIDPool {
    /// seL4_RISCV_ASIDPool 	_service 	The ASID Pool capability to invoke, which must be to an ASID pool that is not full.
    /// seL4_CPtr 	vspace 	The top-level PageTable that is being assigned to an ASID pool. Must not already be assigned to an ASID pool.
    fn assign(&self, service: CapAddr, vspace: CapAddr) -> Result<(), Error>;
}
