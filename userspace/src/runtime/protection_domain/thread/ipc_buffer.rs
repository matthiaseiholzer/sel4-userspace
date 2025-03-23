use crate::runtime::{ipc::SerDe, kernel::{IPCBuffer, MessageInfo}};

pub struct IpcBuffer {
    pub ipc_buffer: *mut IPCBuffer,
}

impl IpcBuffer {
    
    pub fn new(ipc_buffer: *mut IPCBuffer) -> IpcBuffer {
        IpcBuffer {
            ipc_buffer
        }
    }
    
    pub fn write_message(&mut self, message: &impl SerDe) -> MessageInfo {
        let buffer = unsafe {
            let ipc_buffer = &mut *self.ipc_buffer;
            core::slice::from_raw_parts_mut(
                ipc_buffer.msg.as_mut_ptr() as *mut u8,
                IPCBuffer::MSG_MAX_LENGTH * size_of::<usize>(),
            )
        };
    
        let res = message.serialize(buffer);

        let mut msg_info = MessageInfo::default();
        msg_info.length = res.unwrap() as u8;
        msg_info
        
    }
    
    pub fn read_message<T: SerDe>(&self, _msg_info: &MessageInfo) -> T {
    
        let buffer = unsafe {
            let ipc_buffer = &*self.ipc_buffer;
            core::slice::from_raw_parts(
                ipc_buffer.msg.as_ptr() as *const u8,
                IPCBuffer::MSG_MAX_LENGTH * size_of::<usize>(),
            )
        };   
        T::deserialize(buffer).unwrap()
        
    }
}