use crate::print_str;
use crate::runtime::cap_space::CapSpaceManager;
use crate::runtime::kernel::MessageInfo;
use crate::runtime::kernel::{BootInfo, Kernel};
use crate::runtime::protection_domain::thread::Thread;
use crate::servers::untyped_server::api::Message;

pub fn untyped_server<K: Kernel>(thread: &mut Thread<K>, _boot_info: &BootInfo) -> ! {
    let _ = thread.set_debug_name(&"untyped_srv\0");
    print_str!(thread, "Untyped Server\n");

    loop {
        for _ in 0..40_000_000 {}

        let mut sender: usize = 0;
        thread.receive(CapSpaceManager::C_MT_EP, &mut sender);
        
        let msg_info = MessageInfo::default();
        
        let message_read: Message = thread.ipc_buffer.read_message(&msg_info);

        print_str!(thread, "{:?}\n", message_read);

        let addr = 12345;
        let message = Message::AllocResp(addr);

        let msg_info = thread.ipc_buffer.write_message(&message);

        thread.reply(&msg_info);
    }
}
