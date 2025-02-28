use sel4_userspace::runtime::cap_space::cap_rights::CapRights;
use sel4_userspace::runtime::cap_space::{CapAddr, CapData};
use sel4_userspace::runtime::kernel::MessageInfo;
use sel4_userspace::runtime::kernel::{
    ASIDPool, CNode, Error, Kernel, Page, PageTable, Untyped, UntypedType, UserContext, TCB,
};
use sel4_userspace::runtime::va_space::page::Attributes;

#[derive(Clone, Copy)]
pub struct DummyKernel {}

impl Default for DummyKernel {
    fn default() -> DummyKernel {
        DummyKernel {}
    }
}

impl Kernel for DummyKernel {
    fn sys_send_null(&self, _: usize, _: usize, _: usize) {
        todo!()
    }
    fn sys_recv(
        &self,
        _: usize,
        _: usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: usize,
    ) {
        todo!()
    }
    fn sys_send_recv(
        &self,
        _: usize,
        _: usize,
        _: &mut usize,
        _: usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: &mut usize,
        _: usize,
    ) {
        todo!()
    }

    fn sys_reply(&self, _: usize, _: usize, _: usize, _: usize, _: usize, _: usize) {
        todo!()
    }
    
    fn debug_cap_identify(&self, _: usize) -> usize { todo!() }
}

impl TCB for DummyKernel {
    fn suspend(&self, _tcb: CapAddr) -> Result<(), Error> {
        unimplemented!()
    }

    fn resume(&self, _: CapAddr) -> Result<(), Error> {
        unimplemented!()
    }

    fn set_priority(&self, _: CapAddr, _authority: CapAddr, _priority: u8) -> Result<(), Error> {
        unimplemented!()
    }

    fn read_registers(&self) -> Result<UserContext, Error> {
        unimplemented!()
    }

    fn write_registers(&self, _: CapAddr, _registers: &UserContext) -> Result<(), Error> {
        unimplemented!()
    }

    fn configure(
        &self,
        _tcb: CapAddr,
        _fault_ep: CapAddr,
        _cspace_root: CapAddr,
        _cspace_root_data: CapData,
        _vspace_root: CapAddr,
        _vspace_root_data: usize,
        _buffer: usize,
        _buffer_frame: CapAddr,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn set_space(
        &self,
        _: CapAddr,
        _: CapAddr,
        _: CapAddr,
        _: CapData,
        _: CapAddr,
        _: usize,
    ) -> Result<(), sel4_userspace::runtime::kernel::Error> {
        todo!()
    }

    fn set_ipc_buffer(
        &self,
        _tcb: CapAddr,
        _buffer: usize,
        _buffer_frame: CapAddr,
    ) -> Result<(), Error> {
        todo!()
    }
}

impl CNode for DummyKernel {
    fn copy(
        &self,
        _dest_cspace_root: CapAddr,
        _dest_cap: CapAddr,
        _src_cspace_root: CapAddr,
        _src_cap: CapAddr,
        _rights: CapRights,
    ) -> Result<(), Error> {
        unimplemented!();
    }

    fn r#move(
        &self,
        _dest_cpace_root: CapAddr,
        _dest_cap: CapAddr,
        _src_cspace_root: CapAddr,
        _src_cap: CapAddr,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn mutate(
        &self,
        _service: CapAddr,
        _dest_index: usize,
        _dest_depth: u8,
        _src_root: CapAddr,
        _src_index: usize,
        _src_depth: u8,
        _badge: CapData,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    fn mint(
        &self,
        _dest_cspace_root: CapAddr,
        _dest_cap: CapAddr,
        _src_cspace_root: CapAddr,
        _src_cap: CapAddr,
        _rights: CapRights,
        _badge: CapData,
    ) -> Result<(), sel4_userspace::runtime::kernel::Error> {
        todo!()
    }
    fn delete(
        &self,
        _: CapAddr,
        _: CapAddr
    ) -> Result<(), sel4_userspace::runtime::kernel::Error> {
        todo!()
    }
    
    
}

impl Untyped for DummyKernel {
    fn retype(
        &self,
        _service: CapAddr,
        r#_type: UntypedType,
        _size_bits: u8,
        _root: CapAddr,
        _node_index: usize,
        _node_depth: u8,
        _node_offset: usize,
        _num_objects: usize,
    ) -> Result<(), Error> {
        unimplemented!()
    }
}

impl Page for DummyKernel {
    fn map_page(
        &self,
        _: CapAddr,
        _: CapAddr,
        _: usize,
        _: CapRights,
        _: Attributes,
    ) -> Result<(), sel4_userspace::runtime::kernel::Error> {
        todo!()
    }

    fn unmap_page(&self, _service: CapAddr) -> Result<(), Error> {
        todo!()
    }
}

impl PageTable for DummyKernel {
    fn map_page_table(&self, _: CapAddr, _: CapAddr, _: usize, _: Attributes) -> Result<(), Error> {
        todo!()
    }
}

impl ASIDPool for DummyKernel {
    fn assign(&self, _: CapAddr, _: CapAddr) -> Result<(), Error> {
        unimplemented!();
    }
}
