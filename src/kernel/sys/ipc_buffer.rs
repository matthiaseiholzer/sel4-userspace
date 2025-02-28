use super::{
    constants::{seL4_MsgMaxExtraCaps, seL4_MsgMaxLength},
    message_info::seL4_MessageInfo,
    types::{seL4_CPtr, seL4_Word},
};

#[repr(C, align(1024))]
pub struct seL4_IPCBuffer {
    pub tag: seL4_MessageInfo,
    pub msg: [seL4_Word; seL4_MsgMaxLength],
    pub userData: seL4_Word,
    pub caps_or_badges: [seL4_Word; seL4_MsgMaxExtraCaps],
    pub receiveCNode: seL4_CPtr,
    pub receiveIndex: seL4_CPtr,
    pub receiveDepth: seL4_Word,
}
