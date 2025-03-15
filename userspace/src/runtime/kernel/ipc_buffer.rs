use super::constants::{MSG_MAX_EXTRA_CAPS, MSG_MAX_LENGTH};

#[repr(C, align(4096))]
pub struct IPCBuffer {
    pub tag: usize,
    pub msg: [usize; MSG_MAX_LENGTH],
    pub user_data: usize,
    pub caps_or_badges: [usize; MSG_MAX_EXTRA_CAPS],
    pub receive_cnode: usize,
    pub receive_index: usize,
    pub receive_depth: usize,
}

impl IPCBuffer {
    pub const MSG_MAX_LENGTH: usize = MSG_MAX_LENGTH;
}

impl Default for IPCBuffer {
    fn default() -> Self {
        IPCBuffer {
            tag: 0, //MessageInfo::default(),
            msg: [0; MSG_MAX_LENGTH],
            user_data: 0,
            caps_or_badges: [0; MSG_MAX_EXTRA_CAPS],
            receive_cnode: 0,
            receive_index: 0,
            receive_depth: 0,
        }
    }
}
