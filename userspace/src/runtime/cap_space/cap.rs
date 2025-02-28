use core::result::Result;
use core::unimplemented;

#[repr(usize)]
#[derive(Debug)]
pub enum CapType {
    Null = 0,
    Untyped = 2,
    Endpoint = 4,
    Notification = 6,
    Reply = 8,
    Cnode = 10,
    Thread = 12,
    IrqControl = 14,
    IrqHandler = 16,
    Zombie = 18,
    Domain = 20,
    Frame = 1,
    PageTable = 3,
    AsidControl = 11,
    AsidPool = 13,
}

impl From<usize> for CapType {
    fn from(value: usize) -> Self {
        match value {
            _ if value == Self::Null as usize => Self::Null,
            _ if value == Self::Untyped as usize => Self::Untyped,
            _ if value == Self::Endpoint as usize => Self::Endpoint,
            _ if value == Self::Notification as usize => Self::Notification,
            _ if value == Self::Reply as usize => Self::Reply,
            _ if value == Self::Cnode as usize => Self::Cnode,
            _ if value == Self::Thread as usize => Self::Thread,
            _ if value == Self::IrqControl as usize => Self::IrqControl,
            _ if value == Self::IrqHandler as usize => Self::IrqHandler,
            _ if value == Self::Zombie as usize => Self::Zombie,
            _ if value == Self::Domain as usize => Self::Domain,
            _ if value == Self::Frame as usize => Self::Frame,
            _ if value == Self::PageTable as usize => Self::PageTable,
            _ if value == Self::AsidControl as usize => Self::AsidControl,
            _ if value == Self::AsidPool as usize => Self::AsidPool,
            _ => {
                panic!()
            }
        }
    }
}

pub struct UntypedCap {
    pub cap: usize,
}

pub struct PageTableCap {
    pub cap_ptr: usize,
}

impl PageTableCap {
    pub fn new(cap_ptr: usize) -> PageTableCap {
        PageTableCap { cap_ptr: cap_ptr }
    }
}

pub struct PageCap {
    pub cap_ptr: usize,
}

impl PageCap {
    pub fn new(cap_ptr: usize) -> PageCap {
        PageCap { cap_ptr: cap_ptr }
    }
}

impl PageCap {
    pub fn get_phys_addr(&self) -> Result<usize, i64> {
        unimplemented!()
        // let res = unsafe { seL4_RISCV_Page_GetAddress(self.cap_ptr) };

        // if res.error == 0 {
        //     {
        //         let mut message = ArrayString::<50>::default();
        //         let _ = write!(
        //             &mut message,
        //             "{}: Phys addr {:?}\n",
        //             self.cap_ptr, res.paddr
        //         );
        //         unsafe {
        //             kernel_putchar_write(message.as_ptr() as *const c_void, message.len());
        //         }
        //     }
        //     Result::Ok(res.paddr)
        // } else {
        //     Result::Err(res.error)
        // }
    }
}
