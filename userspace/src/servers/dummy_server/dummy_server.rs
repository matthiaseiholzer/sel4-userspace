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

