use crate::kernel::sys::syscall::riscv_sys_recv;
use crate::kernel::sys::syscall::riscv_sys_reply;
use crate::kernel::sys::syscall::riscv_sys_send_null;
use crate::kernel::sys::syscall::riscv_sys_send_recv;
use crate::kernel::sys::syscall::seL4_CNode_Delete;
use crate::kernel::sys::syscall::seL4_CNode_Mint;
use crate::kernel::sys::syscall::seL4_CNode_Move;
use crate::kernel::sys::syscall::seL4_CNode_Mutate;
use crate::kernel::sys::syscall::seL4_DebugCapIdentify;
use crate::kernel::sys::syscall::seL4_DebugDumpScheduler;
use crate::kernel::sys::syscall::seL4_RISCV_ASIDPool_Assign;
use crate::kernel::sys::syscall::seL4_RISCV_PageTable_Map;
use crate::kernel::sys::syscall::seL4_RISCV_Page_Map;
use crate::kernel::sys::syscall::seL4_RISCV_Page_Unmap;
use crate::kernel::sys::syscall::seL4_TCB_Configure;
use crate::kernel::sys::syscall::seL4_TCB_Resume;
use crate::kernel::sys::syscall::seL4_TCB_SetIPCBuffer;
use crate::kernel::sys::syscall::seL4_TCB_SetPriority;
use crate::kernel::sys::syscall::seL4_TCB_SetSpace;
use crate::kernel::sys::syscall::seL4_TCB_Suspend;
use crate::kernel::sys::syscall::seL4_TCB_WriteRegisters;
use crate::kernel::sys::syscall::{seL4_CNode_Copy, seL4_Untyped_Retype};
use crate::kernel::sys::types::seL4_Error;
use crate::kernel::sys::types::seL4_UserContext;
use sel4_us::runtime::cap_space::cap_rights::CapRights;
use sel4_us::runtime::cap_space::CapAddr;
use sel4_us::runtime::cap_space::CapData;
use sel4_us::runtime::kernel::ASIDPool;
use sel4_us::runtime::kernel::Error;
use sel4_us::runtime::kernel::Page;
use sel4_us::runtime::kernel::PageTable;
use sel4_us::runtime::kernel::Untyped;
use sel4_us::runtime::kernel::UntypedType;
use sel4_us::runtime::kernel::UserContext;
use sel4_us::runtime::kernel::{CNode, Kernel, TCB};
use sel4_us::runtime::va_space::page::Attributes;

impl From<seL4_Error> for Result<(), Error> {
    fn from(value: seL4_Error) -> Self {
        match value {
            seL4_Error::seL4_NoError => Result::Ok(()),
            seL4_Error::seL4_InvalidArgument => Result::Err(Error::InvalidArgument),
            seL4_Error::seL4_InvalidCapability => Result::Err(Error::InvalidCapability),
            seL4_Error::seL4_IllegalOperation => Result::Err(Error::IllegalOperation),
            seL4_Error::seL4_RangeError => Result::Err(Error::RangeError),
            seL4_Error::seL4_AlignmentError => Result::Err(Error::AlignmentError),
            seL4_Error::seL4_FailedLookup => Result::Err(Error::FailedLookup),
            seL4_Error::seL4_TruncatedMessage => Result::Err(Error::TruncatedMessage),
            seL4_Error::seL4_DeleteFirst => Result::Err(Error::DeleteFirst),
            seL4_Error::seL4_RevokeFirst => Result::Err(Error::RevokeFirst),
            seL4_Error::seL4_NotEnoughMemory => Result::Err(Error::NotEnoughMemory),
        }
    }
}

#[derive(Clone)]
pub struct SeL4Kernel {}

impl SeL4Kernel {
    pub const fn default() -> Self {
        SeL4Kernel {}
    }
}

impl Kernel for SeL4Kernel {
    fn sys_send_null(&self, sys: usize, src: usize, info_arg: usize) {
        unsafe {
            riscv_sys_send_null(sys, src, info_arg);
        }
    }

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
    ) {
        unsafe {
            riscv_sys_recv(
                sys, src, out_badge, out_info, out_mr0, out_mr1, out_mr2, out_mr3, reply,
            );
        }
    }

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
        in_out_mr3: &mut usize,
        reply: usize,
    ) {
        unsafe {
            riscv_sys_send_recv(
                sys, dest, out_badge, info_arg, out_info, in_out_mr0, in_out_mr1, in_out_mr2,
                in_out_mr3, reply,
            );
        }
    }

    fn sys_reply(
        &self,
        sys: usize,
        info_arg: usize,
        mr0: usize,
        mr1: usize,
        mr2: usize,
        mr3: usize,
    ) {
        unsafe {
            riscv_sys_reply(sys, info_arg, mr0, mr1, mr2, mr3);
        }
    }

    fn debug_cap_identify(&self, cap: usize) -> usize {
        unsafe { seL4_DebugCapIdentify(cap) as usize }
    }

    fn dump_scheduler(&self) {
        unsafe {
            seL4_DebugDumpScheduler();
        }
    }
}

impl TCB for SeL4Kernel {
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
    ) -> Result<(), Error> {
        unsafe {
            seL4_TCB_Configure(
                tcb.addr,
                fault_ep.addr,
                cspace_root.addr,
                cspace_root_data.into(),
                vspace_root.addr,
                vspace_root_data,
                buffer,
                buffer_frame.addr,
            )
        }
        .into()
    }

    fn set_space(
        &self,
        _service: CapAddr,
        fault_ep: CapAddr,
        cspace_root: CapAddr,
        cspace_root_data: CapData,
        vspace_root: CapAddr,
        vspace_root_data: usize,
    ) -> Result<(), Error> {
        unsafe {
            seL4_TCB_SetSpace(
                _service.addr,
                fault_ep.addr,
                cspace_root.addr,
                cspace_root_data.into(),
                vspace_root.addr,
                vspace_root_data,
            )
        }
        .into()
    }

    fn read_registers(&self) -> Result<UserContext, Error> {
        unimplemented!()
    }

    fn write_registers(&self, tcb: CapAddr, registers: &UserContext) -> Result<(), Error> {
        let mut regs = seL4_UserContext::default();
        regs.sp = registers.sp;
        regs.pc = registers.pc;
        regs.a0 = registers.a0;
        regs.a1 = registers.a1;

        unsafe {
            seL4_TCB_WriteRegisters(tcb.addr, false, 0, 32, (&regs) as *const seL4_UserContext)
        }
        .into()
    }

    fn resume(&self, tcb: CapAddr) -> Result<(), Error> {
        unsafe { seL4_TCB_Resume(tcb.addr) }.into()
    }

    fn set_priority(&self, tcb: CapAddr, authority: CapAddr, priority: u8) -> Result<(), Error> {
        unsafe { seL4_TCB_SetPriority(tcb.addr, authority.addr, priority as usize) }.into()
    }

    fn set_ipc_buffer(
        &self,
        tcb: CapAddr,
        buffer: usize,
        buffer_frame: CapAddr,
    ) -> Result<(), Error> {
        unsafe { seL4_TCB_SetIPCBuffer(tcb.addr, buffer, buffer_frame.addr) }.into()
    }

    fn suspend(&self, tcb: CapAddr) -> Result<(), Error> {
        unsafe { seL4_TCB_Suspend(tcb.addr) }.into()
    }
}

impl CNode for SeL4Kernel {
    fn copy(
        &self,
        dest_cspace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
        rights: CapRights,
    ) -> Result<(), Error> {
        let res = unsafe {
            seL4_CNode_Copy(
                dest_cspace_root.addr,
                dest_cap.addr,
                dest_cap.depth,
                src_cspace_root.addr,
                src_cap.addr,
                src_cap.depth,
                rights.into(),
            )
        };
        res.into()
    }
    fn r#move(
        &self,
        dest_cpace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
    ) -> Result<(), Error> {
        unsafe {
            seL4_CNode_Move(
                dest_cpace_root.addr,
                dest_cap.addr,
                dest_cap.depth,
                src_cspace_root.addr,
                src_cap.addr,
                src_cap.depth,
            )
        }
        .into()
    }

    fn mutate(
        &self,
        service: CapAddr,
        dest_index: usize,
        dest_depth: u8,
        src_root: CapAddr,
        src_index: usize,
        src_depth: u8,
        badge: CapData,
    ) -> Result<(), Error> {
        unsafe {
            seL4_CNode_Mutate(
                service.addr,
                dest_index,
                dest_depth,
                src_root.addr,
                src_index,
                src_depth,
                badge.into(),
            )
        }
        .into()
    }

    fn mint(
        &self,
        dest_cspace_root: CapAddr,
        dest_cap: CapAddr,
        src_cspace_root: CapAddr,
        src_cap: CapAddr,
        rights: CapRights,
        badge: CapData,
    ) -> Result<(), Error> {
        unsafe {
            seL4_CNode_Mint(
                dest_cspace_root.addr,
                dest_cap.addr,
                dest_cap.depth,
                src_cspace_root.addr,
                src_cap.addr,
                src_cap.depth,
                rights.into(),
                badge.into(),
            )
        }
        .into()
    }

    fn delete(&self, service: CapAddr, cap: CapAddr) -> Result<(), Error> {
        unsafe { seL4_CNode_Delete(service.addr, cap.addr, cap.depth) }.into()
    }
}

impl Untyped for SeL4Kernel {
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
    ) -> Result<(), Error> {
        unsafe {
            seL4_Untyped_Retype(
                service.addr,
                r#type as usize,
                size_bits as usize,
                root.addr,
                node_index,
                node_depth as usize,
                node_offset,
                num_objects,
            )
        }
        .into()
    }
}
impl PageTable for SeL4Kernel {
    fn map_page_table(
        &self,
        service: CapAddr,
        vspace: CapAddr,
        vaddr: usize,
        attr: Attributes,
    ) -> Result<(), Error> {
        unsafe { seL4_RISCV_PageTable_Map(service.addr, vspace.addr, vaddr, attr.into()) }.into()
    }
}

impl Page for SeL4Kernel {
    fn map_page(
        &self,
        service: CapAddr,
        vspace: CapAddr,
        vaddr: usize,
        rights: CapRights,
        attr: Attributes,
    ) -> Result<(), Error> {
        unsafe { seL4_RISCV_Page_Map(service.addr, vspace.addr, vaddr, rights.into(), attr.into()) }
            .into()
    }

    fn unmap_page(&self, service: CapAddr) -> Result<(), Error> {
        unsafe { seL4_RISCV_Page_Unmap(service.addr) }.into()
    }
}

impl ASIDPool for SeL4Kernel {
    fn assign(&self, service: CapAddr, vspace: CapAddr) -> Result<(), Error> {
        unsafe { seL4_RISCV_ASIDPool_Assign(service.addr, vspace.addr) }.into()
    }
}
