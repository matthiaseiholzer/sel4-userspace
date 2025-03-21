use crate::runtime::kernel::{BootInfo, Kernel};
use crate::runtime::protection_domain::thread::Thread;
//use super::untyped_manager::UntypedManager;
use crate::runtime::cap_space::CapSpaceManager;
use crate::servers::reincarnation_server::api::ReincarnationServerMessage;
use crate::print_str;
use crate::runtime::kernel::IPCBuffer;
use crate::runtime::kernel::MessageInfo;
use core::mem::size_of;

pub fn untyped_server<K: Kernel>(thread: &mut Thread<K>, _boot_info: &BootInfo) -> ! {
    let _ = thread.set_debug_name(&"untyped_srv\0");

    loop {
        for _ in 0..40_000_000 {}
        print_str!(thread, "Untyped Server\n");
        let mut sender: usize = 0;
        thread.receive(CapSpaceManager::C_MT_EP, &mut sender);

        let res = read_message_from_ipc_buffer(&thread);

        print_str!(thread, "{:?}\n", res);
        
        let addr = 0;
        let message = ReincarnationServerMessage::AllocResponse(addr);

        let msg_info = write_message_to_ipc_buffer(thread, &message);

        thread.reply(&msg_info);
    }
}

fn write_message_to_ipc_buffer<K: Kernel>(
    thread: &mut Thread<K>,
    message: &ReincarnationServerMessage,
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

fn read_message_from_ipc_buffer<K: Kernel>(thread: &Thread<K>) -> ReincarnationServerMessage {
    let res = unsafe {
        let ipc_buffer = &*thread.ipc_buffer;
        let buffer = core::slice::from_raw_parts(
            ipc_buffer.msg.as_ptr() as *const u8,
            IPCBuffer::MSG_MAX_LENGTH * size_of::<usize>(),
        );

        ReincarnationServerMessage::deserialize(buffer).unwrap()
    };
    res
}

