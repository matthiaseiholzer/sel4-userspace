use super::thread::{Thread, ThreadBuilder, ThreadId, ThreadIdManager};
use crate::runtime::protection_domain::Id;
use crate::runtime::{
    cap_space::{CapAddr, CapSpaceManager},
    kernel::Kernel,
    lib::array::Array,
    va_space::VASpaceManager,
};
use core::ptr::NonNull;
use core::result::Result;
use core::unimplemented;

#[derive(Clone, Copy)]
pub struct ProtectionDomain<K: Kernel> {
    inner: NonNull<ProtectionDomainInner<K>>,
}

impl<K: Kernel> ProtectionDomain<K> {
    pub fn new(pd_inner: *mut ProtectionDomainInner<K>) -> ProtectionDomain<K> {
        ProtectionDomain {
            inner: unsafe { NonNull::new_unchecked(pd_inner) },
        }
    }

    pub fn kernel(&self) -> &K {
        unsafe { &self.inner.as_ref().kernel }
    }

    pub fn alloc_pages(&mut self, _num_pages: usize) -> Result<usize, ()> {
        unimplemented!()
    }

    pub fn dealloc_pages(&mut self, _addr: usize, _num_pages: usize) -> Result<(), ()> {
        unimplemented!()
    }
}

#[repr(align(8))]
pub struct ProtectionDomainInner<K: Kernel> {
    pub id: Id,
    pub kernel: K,
    pub thread: Array<*mut Thread<K>, { ThreadIdManager::MAX_NUM_THREADS }>,

    pub cap_space_manager: CapSpaceManager,
    pub va_space_manager: VASpaceManager,
    pub thread_id_manager: ThreadIdManager,
}

impl<K: Kernel> ProtectionDomain<K> {
    pub const VSPACE_ROOT_CAP_ADDR: CapAddr = CapAddr::from_const(1);
}

impl<K: Kernel> ProtectionDomainInner<K> {
    pub fn new(
        id: Id,
        kernel: K,
        cap_space_manager: CapSpaceManager,
        va_space_manager: VASpaceManager,
        thread_id_manager: ThreadIdManager,
    ) -> ProtectionDomainInner<K> {
        let mut pd = ProtectionDomainInner {
            id,
            kernel,
            thread: Array::default(),
            cap_space_manager,
            va_space_manager,
            thread_id_manager,
        };

        pd
    }
}

impl<K: Kernel> ProtectionDomain<K> {
    pub fn id(&self) -> Id {
        unsafe { self.inner.as_ref().id }
    }

    pub fn spawn_thread(&'static mut self, _thread_builder: ThreadBuilder<K>) -> ThreadId {
        //let thread_id: ThreadId = self.thread_id_manager.alloc_id().unwrap();

        // let thread = Thread::<K>::new(
        //     thread_id,
        //     //self,
        //     //thread_builder.priority,
        //     //tcb_cap_addr,
        // );
        //thread.push(thread);

        //thread_id
        unimplemented!()
    }

    // pub fn new_thread(&mut self, routine: fn()) -> ThreadId {
    //     let thread_id: ThreadId = 0;

    //     let thread = Thread {
    //         id: thread_id,
    //         routine: routine,
    //         tcb: ThreadControlBlock {
    //             cptr: 0
    //         },
    //     };
    //     self.thread.push(thread);
    //     thread_id
    // }
}
