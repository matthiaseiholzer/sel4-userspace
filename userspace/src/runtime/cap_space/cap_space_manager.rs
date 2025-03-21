use crate::runtime::va_space::VASpaceManager;

use super::CapAddr;

pub struct CapSpaceManager {}

impl CapSpaceManager {
    pub const CSPACE_WIDTH_BITS: u8 =
        Self::ROOT_WIDTH_BITS + Self::L0_WIDTH_BITS + Self::L1_WIDTH_BITS + Self::L2_WIDTH_BITS;

    //pub const CSPACE_CAP: usize = 1;
    pub const C_ASID_OFFSET: usize = 1;

    pub const ROOT_WIDTH_BITS: u8 = 7;

    pub const L0_WIDTH_BITS: u8 = VASpaceManager::L0_WIDTH_BITS + 1;
    pub const L0_SIZE: usize = 1 << Self::L0_WIDTH_BITS;

    pub const L1_WIDTH_BITS: u8 = VASpaceManager::L1_WIDTH_BITS + 1;
    pub const L1_SIZE: usize = 1 << Self::L1_WIDTH_BITS;

    pub const L2_WIDTH_BITS: u8 = VASpaceManager::L2_WIDTH_BITS;
    pub const L2_SIZE: usize = 1 << Self::L2_WIDTH_BITS;

    pub const C_0_OFFSET: usize = 2;
    pub const C_V_0_OFFSET: usize = 3;

    // CSpace Root Node
    pub const C_CSPACE_ROOT: CapAddr = Self::new(
        Self::C_CSPACE_ROOT_0_OFFSET,
        Self::C_CSPACE_ROOT_1_OFFSET,
        Self::C_CSPACE_ROOT_2_OFFSET,
        64,
    );
    pub const C_CSPACE_ROOT_0_OFFSET: usize = Self::C_PD_0_OFFSET;
    pub const C_CSPACE_ROOT_1_OFFSET: usize = Self::C_PD_1_OFFSET;
    pub const C_CSPACE_ROOT_2_OFFSET: usize = Self::C_V_PD_2_OFFSET + 1;

    //
    pub const C_ASID_POOL: CapAddr = Self::new(
        Self::C_ASID_POOL_0_OFFSET,
        Self::C_ASID_POOL_1_OFFSET,
        Self::C_ASID_POOL_2_OFFSET,
        64,
    );
    pub const C_ASID_POOL_0_OFFSET: usize = Self::C_PD_0_OFFSET;
    pub const C_ASID_POOL_1_OFFSET: usize = Self::C_PD_1_OFFSET;
    pub const C_ASID_POOL_2_OFFSET: usize = Self::C_CSPACE_ROOT_2_OFFSET + 1;

    pub const C_VSPACE_ROOT: CapAddr = Self::new(
        Self::C_VSPACE_ROOT_0_OFFSET,
        Self::C_VSPACE_ROOT_1_OFFSET,
        Self::C_VSPACE_ROOT_2_OFFSET,
        64,
    );
    pub const C_VSPACE_ROOT_0_OFFSET: usize = Self::C_PD_0_OFFSET;
    pub const C_VSPACE_ROOT_1_OFFSET: usize = Self::C_PD_1_OFFSET;
    pub const C_VSPACE_ROOT_2_OFFSET: usize = Self::C_ASID_POOL_2_OFFSET + 1;

    // Protection Domain
    pub const C_PD_0_OFFSET: usize = 2 * VASpaceManager::PD_L0_OFFSET;
    pub const C_V_PD_0_OFFSET: usize = Self::C_PD_0_OFFSET + 1;
    pub const C_PD_1_OFFSET: usize = 2 * VASpaceManager::PD_L1_OFFSET;
    pub const C_V_PD_1_OFFSET: usize = Self::C_PD_1_OFFSET + 1;
    pub const C_V_PD_2_OFFSET: usize = VASpaceManager::PD_L2_OFFSET;

    // Main Thread
    pub const C_MT_0_OFFSET: usize = 2 * VASpaceManager::MT_L0_OFFSET;
    pub const C_V_MT_0_OFFSET: usize = Self::C_MT_0_OFFSET + 1;
    pub const C_MT_1_OFFSET: usize = 2 * VASpaceManager::MT_L1_OFFSET;
    pub const C_V_MT_1_OFFSET: usize = Self::C_MT_1_OFFSET + 1;
    pub const C_V_MT_2_OFFSET: usize = VASpaceManager::MT_L2_OFFSET;

    // Main Thread IPC BUFFER
    pub const C_MT_IPC_BUFFER: CapAddr = CapSpaceManager::new(
        CapSpaceManager::C_MT_IPC_BUFFER_0_OFFSET,
        CapSpaceManager::C_MT_IPC_BUFFER_1_OFFSET,
        CapSpaceManager::C_V_MT_IPC_BUFFER_2_OFFSET,
        64,
    );
    pub const C_MT_IPC_BUFFER_0_OFFSET: usize = 2 * VASpaceManager::MT_IPC_BUFFER_L0_OFFSET;
    pub const C_V_MT_IPC_BUFFER_0_OFFSET: usize = Self::C_MT_IPC_BUFFER_0_OFFSET + 1;
    pub const C_MT_IPC_BUFFER_1_OFFSET: usize = 2 * VASpaceManager::MT_IPC_BUFFER_L1_OFFSET;
    pub const C_V_MT_IPC_BUFFER_1_OFFSET: usize = Self::C_MT_IPC_BUFFER_1_OFFSET + 1;
    pub const C_V_MT_IPC_BUFFER_2_OFFSET: usize = VASpaceManager::MT_IPC_BUFFER_L2_OFFSET;

    // Main Thread TCB
    pub const C_MT_TCB: CapAddr = CapSpaceManager::new(
        CapSpaceManager::C_MT_TCB_0_OFFSET,
        CapSpaceManager::C_MT_TCB_1_OFFSET,
        CapSpaceManager::C_V_MT_TCB_2_OFFSET,
        64,
    );
    pub const C_MT_TCB_0_OFFSET: usize = 2 * VASpaceManager::MT_TCB_L0_OFFSET;
    pub const C_V_MT_TCB_0_OFFSET: usize = Self::C_MT_TCB_0_OFFSET + 1;
    pub const C_MT_TCB_1_OFFSET: usize = 2 * VASpaceManager::MT_TCB_L1_OFFSET;
    pub const C_V_MT_TCB_1_OFFSET: usize = Self::C_MT_TCB_1_OFFSET + 1;
    pub const C_V_MT_TCB_2_OFFSET: usize = VASpaceManager::MT_TCB_L2_OFFSET;

    // Main Thread Notification
    pub const C_MT_EP: CapAddr = CapSpaceManager::new(
        CapSpaceManager::C_MT_EP_0_OFFSET,
        CapSpaceManager::C_MT_EP_1_OFFSET,
        CapSpaceManager::C_V_MT_EP_2_OFFSET,
        64,
    );
    
    pub const C_MT_EP_0_OFFSET: usize = 2 * VASpaceManager::MT_EP_L0_OFFSET;
    pub const C_V_MT_EP_0_OFFSET: usize = Self::C_MT_EP_0_OFFSET + 1;
    pub const C_MT_EP_1_OFFSET: usize = 2 * VASpaceManager::MT_EP_L1_OFFSET;
    pub const C_V_MT_EP_1_OFFSET: usize = Self::C_MT_EP_1_OFFSET + 1;
    pub const C_V_MT_EP_2_OFFSET: usize = VASpaceManager::MT_EP_L2_OFFSET;

    //Main Thread Stack
    pub const C_MT_STACK_0_OFFSET: usize = 2 * VASpaceManager::MT_STACK_L0_OFFSET;
    pub const C_V_MT_STACK_0_OFFSET: usize = Self::C_MT_STACK_0_OFFSET + 1;
    pub const C_MT_STACK_1_OFFSET: usize = 2 * VASpaceManager::MT_STACK_L1_OFFSET;
    pub const C_V_MT_STACK_1_OFFSET: usize = Self::C_MT_STACK_1_OFFSET + 1;
    pub const C_V_MT_STACK_2_OFFSET: usize = VASpaceManager::MT_STACK_L2_OFFSET;
}

impl Default for CapSpaceManager {
    fn default() -> Self {
        CapSpaceManager {}
    }
}

impl CapSpaceManager {
    pub const fn new(c_l0_idx: usize, c_l1_idx: usize, c_l2_idx: usize, depth: u8) -> CapAddr {
        let cap = Self::cap_addr_l012(c_l0_idx, c_l1_idx, c_l2_idx);

        CapAddr {
            addr: cap.addr,
            depth,
        }
    }

    pub fn cap_addr_l0(c_l0_idx: usize) -> CapAddr {
        let addr = (Self::C_0_OFFSET << Self::L0_WIDTH_BITS) | c_l0_idx;

        CapAddr {
            addr,
            depth: Self::ROOT_WIDTH_BITS + Self::L0_WIDTH_BITS,
        }
    }

    pub fn cap_addr_l01(c_l0_idx: usize, c_l1_idx: usize) -> CapAddr {
        let addr = (((Self::C_0_OFFSET << Self::L0_WIDTH_BITS) | c_l0_idx) << Self::L1_WIDTH_BITS)
            | c_l1_idx;

        CapAddr {
            addr,
            depth: Self::ROOT_WIDTH_BITS + Self::L0_WIDTH_BITS + Self::L1_WIDTH_BITS,
        }
    }

    pub const fn cap_addr_l012(c_l0_idx: usize, c_l1_idx: usize, c_l2_idx: usize) -> CapAddr {
        let addr = (((((Self::C_0_OFFSET << Self::L0_WIDTH_BITS) | c_l0_idx)
            << Self::L1_WIDTH_BITS)
            | c_l1_idx)
            << Self::L2_WIDTH_BITS)
            | c_l2_idx;

        CapAddr {
            addr,
            depth: Self::ROOT_WIDTH_BITS
                + Self::L0_WIDTH_BITS
                + Self::L1_WIDTH_BITS
                + Self::L2_WIDTH_BITS,
        }
    }
}
