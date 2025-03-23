use crate::print_str;
use crate::runtime::cap_space::{CapAddr, CapSpaceManager};
use crate::runtime::ipc::SerDe;
use crate::runtime::kernel::IPCBuffer;
use crate::runtime::kernel::{BootInfo, Kernel, MessageInfo};
use crate::runtime::protection_domain::thread::Thread;
use crate::servers::untyped_server::api::Message as UntypedServerMessage;

pub const ENDPOINT: CapAddr = CapSpaceManager::new(
    CapSpaceManager::C_MT_EP_0_OFFSET,
    CapSpaceManager::C_MT_EP_1_OFFSET,
    CapSpaceManager::C_V_MT_EP_2_OFFSET + 1,
    64,
);

pub fn reincarnation_server<K: Kernel>(thread: &mut Thread<K>, _boot_info: &BootInfo) -> ! {
    let _ = thread.set_debug_name(&"reincarnation_srv\0");

    loop {
        for _ in 0..40_000_000 {}
        print_str!(thread, "Reincarnation Server\n");

        let send_message = UntypedServerMessage::AllocReq(1);
        let msg_info = write_message_to_ipc_buffer(thread, &send_message);
        print_str!(thread, "Request {:?}\n", send_message);
        
        let _ = thread.call(ENDPOINT, &msg_info);
        
        let resp = read_message_from_ipc_buffer(&thread);
        print_str!(thread, "Response {:?}\n", resp);
    }
}

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
