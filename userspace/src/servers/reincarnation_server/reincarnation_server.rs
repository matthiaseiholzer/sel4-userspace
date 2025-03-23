use crate::print_str;
use crate::runtime::cap_space::{CapAddr, CapSpaceManager};
use crate::runtime::kernel::{BootInfo, Kernel, MessageInfo};
use crate::runtime::protection_domain::thread::Thread;
use crate::servers::pd_manager::api::Message;
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

        let message_req = UntypedServerMessage::AllocReq(1);
        let msg_info_req = thread.ipc_buffer.write_message(&message_req);
        print_str!(thread, "Request {:?}\n", message_req);
        
        let _ = thread.call(ENDPOINT, &msg_info_req);
        
        let msg_info_resp = MessageInfo::default();
        let message_resp: Message  = thread.ipc_buffer.read_message(&msg_info_resp);
        print_str!(thread, "Response {:?}\n", message_resp);
    }
}