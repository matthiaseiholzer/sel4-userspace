use core::default::Default;

pub struct VASpaceManager {}

impl VASpaceManager {
    pub const SV39: usize = 39;

    pub const L0_WIDTH_BITS: u8 = 9;
    pub const L1_WIDTH_BITS: u8 = 9;
    pub const L2_WIDTH_BITS: u8 = 9;
    pub const PAGE_WIDTH_BITS: u8 = 12;
    pub const PAGE_SIZE_BYTES: usize = 1 << Self::PAGE_WIDTH_BITS;

    // This the first address, which is not accessible anymore
    const SEL4_USER_TOP: usize = 0x0000003ffffff000;
    pub const STACK_SIZE_BYTES: usize = 1 << (Self::PAGE_WIDTH_BITS + Self::L2_WIDTH_BITS);
    pub const USER_SPACE_TOP: usize =
        Self::SEL4_USER_TOP - Self::SEL4_USER_TOP % Self::STACK_SIZE_BYTES;

    pub const PD_SIZE_BYTES: usize = 4096;
    pub const PD_ADDRESS: usize =
        Self::USER_SPACE_TOP - (1 << Self::L2_WIDTH_BITS) * Self::PD_SIZE_BYTES;
    pub const PD_L0_OFFSET: usize = Self::PD_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const PD_L1_OFFSET: usize = (Self::PD_ADDRESS
        - (Self::PD_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const PD_L2_OFFSET: usize = 0;

    // Main Thread
    pub const MT_ADDRESS: usize =
        Self::PD_ADDRESS - (1 << Self::L2_WIDTH_BITS) * Self::PD_SIZE_BYTES;
    pub const MT_SIZE_BYTES: usize = Self::PAGE_SIZE_BYTES;
    pub const MT_L0_OFFSET: usize = Self::MT_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_L1_OFFSET: usize = (Self::MT_ADDRESS
        - (Self::MT_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_L2_OFFSET: usize = 0;

    // Main Threa IPC Buffer
    pub const MT_IPC_BUFFER_ADDRESS: usize = Self::MT_ADDRESS + Self::MT_SIZE_BYTES;
    pub const MT_IPC_BUFFER_SIZE_BYTES: usize = Self::PAGE_SIZE_BYTES;
    pub const MT_IPC_BUFFER_L0_OFFSET: usize = Self::MT_IPC_BUFFER_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_IPC_BUFFER_L1_OFFSET: usize = (Self::MT_IPC_BUFFER_ADDRESS
        - (Self::MT_IPC_BUFFER_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_IPC_BUFFER_L2_OFFSET: usize = Self::MT_L2_OFFSET + 1;

    //Main Thread TCB
    pub const MT_TCB_ADDRESS: usize = Self::MT_IPC_BUFFER_ADDRESS + Self::MT_IPC_BUFFER_SIZE_BYTES;
    pub const MT_TCB_SIZE_BYTES: usize = Self::PAGE_SIZE_BYTES;
    pub const MT_TCB_L0_OFFSET: usize = Self::MT_TCB_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_TCB_L1_OFFSET: usize = (Self::MT_TCB_ADDRESS
        - (Self::MT_TCB_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_TCB_L2_OFFSET: usize = Self::MT_IPC_BUFFER_L2_OFFSET + 1;

    // Main Thread Endpoint
    pub const MT_EP_ADDRESS: usize = Self::MT_TCB_ADDRESS + Self::MT_TCB_SIZE_BYTES;
    pub const MT_EP_SIZE_BYTES: usize = Self::PAGE_SIZE_BYTES;
    pub const MT_EP_L0_OFFSET: usize = Self::MT_EP_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_EP_L1_OFFSET: usize = (Self::MT_EP_ADDRESS
        - (Self::MT_EP_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_EP_L2_OFFSET: usize = Self::MT_TCB_L2_OFFSET + 1;

    // Main Thread Stack
    pub const MT_STACK_SIZE_PAGES: usize = 1;
    pub const MT_STACK_SIZE_BYTES: usize = Self::MT_STACK_SIZE_PAGES * Self::PAGE_SIZE_BYTES;
    pub const MT_STACK_ADDRESS: usize = Self::PD_ADDRESS - Self::MT_STACK_SIZE_BYTES;
    pub const MT_STACK_L0_OFFSET: usize = Self::MT_STACK_ADDRESS
        / (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_STACK_L1_OFFSET: usize = (Self::MT_STACK_ADDRESS
        - (Self::MT_STACK_L0_OFFSET
            * (1 << (Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS))))
        / (1 << (Self::L2_WIDTH_BITS + Self::PAGE_WIDTH_BITS));
    pub const MT_STACK_L2_OFFSET: usize = (1 << Self::L2_WIDTH_BITS) - Self::MT_STACK_SIZE_PAGES;
}

impl Default for VASpaceManager {
    fn default() -> VASpaceManager {
        VASpaceManager {}
    }
}

impl VASpaceManager {
    pub fn stack_space_start(thread: u8) -> usize {
        Self::USER_SPACE_TOP - (thread as usize + 1) * Self::STACK_SIZE_BYTES
    }

    pub fn stack_space_end(thread: u8) -> usize {
        Self::USER_SPACE_TOP - (thread as usize) * Self::STACK_SIZE_BYTES
    }

    pub fn va_indices(&self, mut addr: usize) -> (usize, usize, usize) {
        addr = addr >> Self::PAGE_WIDTH_BITS;
        let l2 = addr & ((1 << Self::L2_WIDTH_BITS) - 1);
        addr = addr >> Self::L2_WIDTH_BITS;
        let l1 = addr & ((1 << Self::L1_WIDTH_BITS) - 1);
        addr = addr >> Self::L1_WIDTH_BITS;
        let l0 = addr & ((1 << Self::L0_WIDTH_BITS) - 1);

        (l2, l1, l0)
    }
}
