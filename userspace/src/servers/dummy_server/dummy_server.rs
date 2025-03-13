use crate::runtime::cap_space::CapSpaceManager;
use crate::runtime::kernel::IPCBuffer;
use crate::runtime::kernel::Kernel;
use crate::runtime::kernel::MessageInfo;
use crate::runtime::protection_domain::thread::Thread;
use crate::servers::reincarnation_server::api::ReincarnationServerMessage;
use core::mem::size_of;

pub fn dummy_server<K: Kernel>(thread: &mut Thread<K>) -> ! {
    let pd = &thread.pd;
    let id = pd.id();
    let _ = thread.set_debug_name("dummy server\0");

    loop {
        let mut a = [0; 100];
        for k in 0..100 {
            for _ in 0..1_000_000 {
                a[k] += 1;
            }
        }
        {
            let message = ReincarnationServerMessage::AllocRequest(5);
            let msg_info = write_message_to_ipc_buffer(thread, &message);
            thread.call(CapSpaceManager::C_MT_EP, &msg_info);
        }

        let message = read_message_from_ipc_buffer(&thread);
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
