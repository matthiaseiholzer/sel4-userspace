use super::ThreadId;
use crate::runtime::cap_space::cap::CapType;
use crate::runtime::cap_space::CapAddr;
use crate::runtime::kernel::constants::MSG_MAX_LENGTH;
use crate::runtime::kernel::MessageInfo;
use crate::runtime::kernel::{IPCBuffer, Kernel};
use crate::runtime::protection_domain::ProtectionDomain;
use crate::runtime::va_space::VASpaceManager;
use core::ascii::Char;
use core::ptr::null_mut;

pub struct Thread<K: Kernel> {
    pub id: ThreadId,
    pub pd: ProtectionDomain<K>,
    pub priority: u8,
    tcb: CapAddr,
    pub ipc_buffer: *mut IPCBuffer,
}

impl<K: Kernel> Thread<K> {}

impl<K: Kernel> Thread<K> {
    pub fn new(
        id: ThreadId,
        tcb: CapAddr,
        pd: ProtectionDomain<K>,
        ipc_buffer: *mut IPCBuffer,
    ) -> Thread<K> {
        Thread {
            id,
            pd,
            priority: 255,
            tcb: tcb,
            ipc_buffer,
        }
    }

    pub fn resume(&mut self) {}

    pub fn join() {}

    pub fn signal(&self, dest: CapAddr) {
        let kernel = self.pd.kernel();
        kernel.sys_send_null(
            (-3 as isize) as usize,
            dest.addr,
            MessageInfo::default().into(),
        );
    }

    pub fn put_char(&self, c: char) {
        let kernel = self.pd.kernel();

        let mut unused0: usize = 0;
        let mut unused1: usize = 0;
        let mut unused2: usize = 0;
        let mut unused3: usize = 0;
        let mut unused4: usize = 0;
        let mut unused5: usize = 0;

        kernel.sys_send_recv(
            (-9 as isize) as usize,
            c as usize,
            &mut unused0,
            0,
            &mut unused1,
            &mut unused2,
            &mut unused3,
            &mut unused4,
            &mut unused5,
            0,
        );
    }

    pub fn put_u8_array(&self, char_array: &[u8]) {
        char_array.iter().for_each(|c| self.put_char(*c as char));
    }

    pub fn put_str(&self, str: &str) {
        let str_ascii_array: &[u8] =
            unsafe { core::slice::from_raw_parts(str.as_ptr() as *const u8, str.len()) };
        str_ascii_array
            .iter()
            .for_each(|c| self.put_char(*c as char));
    }

    pub fn set_debug_name(&mut self, name: &str) -> Result<(), ()> {
        let name_ascii_array: &[u8] = unsafe {
            core::slice::from_raw_parts(name.as_ascii().unwrap().as_ptr() as *const u8, name.len())
        };

        if name_ascii_array.len() > MSG_MAX_LENGTH {
            return Err(());
        }

        unsafe {
            let mut dest = (*self.ipc_buffer).msg.as_mut_ptr() as *mut u8;
            core::ptr::copy(name_ascii_array.as_ptr(), dest, name_ascii_array.len());
        }

        let mut unused0: usize = 0;
        let mut unused1: usize = 0;
        let mut unused2: usize = 0;
        let mut unused3: usize = 0;
        let mut unused4: usize = 0;
        let mut unused5: usize = 0;

        let kernel = self.pd.kernel();
        kernel.sys_send_recv(
            (-14 as isize) as usize,
            self.tcb.addr,
            &mut unused0,
            0,
            &mut unused1,
            &mut unused2,
            &mut unused3,
            &mut unused4,
            &mut unused5,
            0,
        );

        return Ok(());
    }

    pub fn debug_dump_scheduler(&self) {
        let kernel = self.pd.kernel();

        let mut unused0: usize = 0;
        let mut unused1: usize = 0;
        let mut unused2: usize = 0;
        let mut unused3: usize = 0;
        let mut unused4: usize = 0;
        let mut unused5: usize = 0;

        kernel.sys_send_recv(
            (-10 as isize) as usize,
            0,
            &mut unused0,
            0,
            &mut unused1,
            &mut unused2,
            &mut unused3,
            &mut unused4,
            &mut unused5,
            0,
        );
    }

    pub fn receive(&mut self, src: CapAddr, sender: &mut usize) -> MessageInfo {
        let mut badge: usize = 0;
        let mut msg0: usize = 0;
        let mut msg1: usize = 0;
        let mut msg2: usize = 0;
        let mut msg3: usize = 0;
        let mut info: usize = 0;

        let kernel = self.pd.kernel();
        kernel.sys_recv(
            (-5 as isize) as usize,
            src.addr,
            &mut badge,
            &mut info,
            &mut msg0,
            &mut msg1,
            &mut msg2,
            &mut msg3,
            0,
        );

        unsafe {
            (*self.ipc_buffer).msg[0] = msg0;
            (*self.ipc_buffer).msg[1] = msg1;
            (*self.ipc_buffer).msg[2] = msg2;
            (*self.ipc_buffer).msg[3] = msg3;
        }

        /* Return back sender and message information. */
        *sender = badge;

        return MessageInfo::from(info);
    }

    pub fn wait(&mut self, src: CapAddr, sender: &mut usize) {
        let _ = self.receive(src, sender);
    }

    pub fn call(&mut self, dest: CapAddr, msg_info: &MessageInfo) -> MessageInfo {
        let mut msg0: usize = unsafe { (*self.ipc_buffer).msg[0] };
        let mut msg1: usize = unsafe { (*self.ipc_buffer).msg[1] };
        let mut msg2: usize = unsafe { (*self.ipc_buffer).msg[2] };
        let mut msg3: usize = unsafe { (*self.ipc_buffer).msg[3] };
        let mut info: usize = 0;
        let kernel = self.pd.kernel();
        let mut dst: usize = dest.addr;
        kernel.sys_send_recv(
            (-1 as isize) as usize,
            dst,
            &mut dst,
            msg_info.clone().into(),
            &mut info,
            &mut msg0,
            &mut msg1,
            &mut msg2,
            &mut msg3,
            0,
        );

        unsafe {
            (*self.ipc_buffer).msg[0] = msg0;
            (*self.ipc_buffer).msg[1] = msg1;
            (*self.ipc_buffer).msg[2] = msg2;
            (*self.ipc_buffer).msg[3] = msg3;
        }
        return MessageInfo::from(info);
    }

    pub fn reply(&mut self, msg_info: &MessageInfo) {
        let mut msg0: usize = unsafe { (*self.ipc_buffer).msg[0] };
        let mut msg1: usize = unsafe { (*self.ipc_buffer).msg[1] };
        let mut msg2: usize = unsafe { (*self.ipc_buffer).msg[2] };
        let mut msg3: usize = unsafe { (*self.ipc_buffer).msg[2] };

        let kernel = self.pd.kernel();
        kernel.sys_reply(
            (-6 as isize) as usize,
            msg_info.clone().into(),
            msg0,
            msg1,
            msg2,
            msg3,
        );
    }

    pub fn identify_cap(&self, dest: CapAddr) -> CapType {
        let mut unused0: usize = 0;
        let mut unused1: usize = 0;
        let mut unused2: usize = 0;
        let mut unused3: usize = 0;
        let mut unused4: usize = 0;
        let mut resp: usize = 0;

        let kernel = self.pd.kernel();
        let _ = kernel.sys_send_recv(
            (-12 as isize) as usize,
            dest.addr,
            &mut resp,
            0,
            &mut unused0,
            &mut unused1,
            &mut unused2,
            &mut unused3,
            &mut unused4,
            0,
        );

        CapType::from(resp)
    }
}
