use crate::runtime::{cap_space::CapAddr, kernel::Kernel, protection_domain::ProtectionDomain};

pub struct ThreadBuilder<K: Kernel> {
    pub stack_size_bits: u8,
    pub priority: u8,
    pub routine: fn(&mut ProtectionDomain<K>),
}

impl<K: Kernel> ThreadBuilder<K> {
    pub const DEFAULT_STACK_SIZE_BITS: u8 = 12;
    pub const DEFAULT_THREAD_PRIORITY: u8 = 4;
}

impl<K: Kernel> ThreadBuilder<K> {
    pub fn new(routine: fn(&mut ProtectionDomain<K>)) -> ThreadBuilder<K> {
        ThreadBuilder {
            routine,
            stack_size_bits: Self::DEFAULT_STACK_SIZE_BITS,
            priority: Self::DEFAULT_THREAD_PRIORITY,
        }
    }

    pub fn stack_size(&mut self, size_bits: u8) -> &mut ThreadBuilder<K> {
        self.stack_size_bits = size_bits;
        self
    }

    pub fn tcb_cap_addr(&mut self, _tcb_cap_addr: CapAddr) -> &mut ThreadBuilder<K> {
        unimplemented!()
        //self.tcb_cap_addr = Option::Some(tcb_cap_addr);
        //self
    }
}
