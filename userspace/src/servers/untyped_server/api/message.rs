use crate::runtime::ipc::SerDe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    AllocReq(usize),    //0
    AllocResp(usize),   //1
    DeallocReq(usize),  // 2
    DeallocResp(usize), //3
}

impl SerDe for Message {
    fn serialize(&self, serializer: &mut [u8]) -> Result<usize, ()> {
        match *self {
            Message::AllocReq(num_pages) => {
                serializer[0] = 0;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = num_pages };
                return Result::Ok(2 * size_of::<usize>());
            }
            Message::AllocResp(address) => {
                serializer[0] = 1;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = address };
                return Result::Ok(2 * size_of::<usize>());
            }
            Message::DeallocReq(_) => Err(()),
            Message::DeallocResp(_) => Err(()),
        }
    }

    fn deserialize(deserializer: &[u8]) -> Result<Self, ()> {
        let message = deserializer[0];
        match message {
            0 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let num_pages = unsafe { *ptr };
                return Result::Ok(Message::AllocReq(num_pages));
            }
            1 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let address = unsafe { *ptr };
                return Result::Ok(Message::AllocResp(address));
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
