use crate::runtime::{ipc::SerDe, kernel::IPCBuffer};



pub struct IpcBuffer {
    ipc_buffer_sel4: *mut IPCBuffer,
}

impl IpcBuffer {
    fn write_message(&mut self, message: &impl SerDe) -> MessageInfo
    
    fn write_message_to_ipc_buffer<K: Kernel>(
        thread: &mut Thread<K>,
        message: &UntypedServerMessage,
    ) -> MessageInfo {
        unsafe {
            let ipc_buffer = &mut *thread.ipc_buffer;
            let buffer = core::slice::from_raw_parts_mut(
                ipc_buffer.msg.as_mut_ptr() as *mut u8,
                IPCBuffer::MSG_MAX_LENGTH * size_of::<usize>(),
            );
    
            let res = message.serialize(buffer);
    
            let mut msg_info = MessageInfo::default();
            msg_info.length = res.unwrap() as u8;
            msg_info
        }
    }
    
    fn read_message_from_ipc_buffer<K: Kernel>(thread: &Thread<K>) -> UntypedServerMessage {
        let res = unsafe {
            let ipc_buffer = &*thread.ipc_buffer;
            let buffer = core::slice::from_raw_parts(
                ipc_buffer.msg.as_ptr() as *const u8,
                IPCBuffer::MSG_MAX_LENGTH * size_of::<usize>(),
            );
    
            UntypedServerMessage::deserialize(buffer).unwrap()
        };
        res
    }
}