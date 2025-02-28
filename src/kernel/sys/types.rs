use core::clone::Clone;
use core::cmp::{Eq, PartialEq};
use core::default::Default;
use core::fmt::Debug;

pub type seL4_Word = usize;
pub type seL4_CPtr = seL4_Word;
pub type seL4_SlotPos = seL4_Word;

pub type seL4_Uint64 = u64;
pub type seL4_Uint32 = u32;
pub type seL4_Uint8 = u8;
pub type seL4_Bool = bool;

pub type seL4_NodeId = seL4_Word;
pub type seL4_PAddr = seL4_Word;
pub type seL4_Domain = seL4_Word;

pub type seL4_CNode = seL4_CPtr;
pub type seL4_IRQHandler = seL4_CPtr;
pub type seL4_IRQControl = seL4_CPtr;
pub type seL4_TCB = seL4_CPtr;
pub type seL4_Untyped = seL4_CPtr;
pub type seL4_DomainSet = seL4_CPtr;
pub type seL4_SchedContext = seL4_CPtr;
pub type seL4_SchedControl = seL4_CPtr;

pub type seL4_Time = seL4_Uint64;

#[repr(C)]
pub struct seL4_CapRights {
    pub words: [seL4_Uint64; 1],
}

pub struct seL4_CNode_CapData {
    pub words: [seL4_Uint64; 1],
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum seL4_Error {
    seL4_NoError = 0,
    seL4_InvalidArgument,
    seL4_InvalidCapability,
    seL4_IllegalOperation,
    seL4_RangeError,
    seL4_AlignmentError,
    seL4_FailedLookup,
    seL4_TruncatedMessage,
    seL4_DeleteFirst,
    seL4_RevokeFirst,
    seL4_NotEnoughMemory,
    /* This should always be the last item in the list
     * so it gives a count of the number of errors in the
     * enum.
     */
    //seL4_NumErrors,
}

/**
 * 32
 */
#[repr(C)]
#[derive(Default)]
pub struct seL4_UserContext {
    pub pc: seL4_Word,
    pub ra: seL4_Word,
    pub sp: seL4_Word,
    pub gp: seL4_Word,

    pub s0: seL4_Word,
    pub s1: seL4_Word,
    pub s2: seL4_Word,
    pub s3: seL4_Word,
    pub s4: seL4_Word,
    pub s5: seL4_Word,
    pub s6: seL4_Word,
    pub s7: seL4_Word,
    pub s8: seL4_Word,
    pub s9: seL4_Word,
    pub s10: seL4_Word,
    pub s11: seL4_Word,

    pub a0: seL4_Word,
    pub a1: seL4_Word,
    pub a2: seL4_Word,
    pub a3: seL4_Word,
    pub a4: seL4_Word,
    pub a5: seL4_Word,
    pub a6: seL4_Word,
    pub a7: seL4_Word,

    pub t0: seL4_Word,
    pub t1: seL4_Word,
    pub t2: seL4_Word,
    pub t3: seL4_Word,
    pub t4: seL4_Word,
    pub t5: seL4_Word,
    pub t6: seL4_Word,

    pub tp: seL4_Word,
}

#[repr(usize)]
#[derive(PartialEq, Eq)]
pub enum seL4_ObjectType {
    seL4_UntypedObject = 0,
    seL4_TCBObject = 1,
    seL4_EndpointObject = 2,
    seL4_NotificationObject = 3,
    seL4_CapTableObject = 4,
    seL4_NonArchObjectTypeCount,
}

#[repr(usize)]
#[derive(PartialEq, Eq)]
pub enum seL4_ModeObjectType {
    seL4_RISCV_Giga_Page = seL4_ObjectType::seL4_NonArchObjectTypeCount as usize,
    seL4_ModeObjectTypeCount,
}

#[repr(usize)]
#[derive(PartialEq, Eq)]
pub enum seL4_ArchObjectType {
    seL4_RISCV_4K_Page = seL4_ModeObjectType::seL4_ModeObjectTypeCount as usize,
    seL4_RISCV_Mega_Page,
    seL4_RISCV_PageTableObject,
    seL4_ObjectTypeCount,
}

#[repr(usize)]
#[derive(PartialEq, Eq, Clone)]
pub enum seL4_RISCV_VMAttributes {
    seL4_RISCV_ExecuteNever = 0x1,
    seL4_RISCV_Default_VMAttributes = 0,
    //SEL4_FORCE_LONG_ENUM(seL4_RISCV_VMAttributes)
}

#[repr(C)]
#[derive(PartialEq, Eq, Clone)]
pub struct seL4_RISCV_Page_GetAddress {
    pub error: i64,
    pub paddr: usize,
}
