use super::ProtectionDomainId;
use crate::runtime::ipc::SerDe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    CreateProtectionDomainReq,                      //0
    CreateProtectionDomainResp(ProtectionDomainId), //1,
    DeleteProtectionDomainReq(ProtectionDomainId),  //2
    DeleteProtectionDomainResp,                     //3
}

impl SerDe for Message {
    /**
     */
    fn serialize(&self, serializer: &mut [u8]) -> Result<usize, ()> {
        match *self {
            Message::CreateProtectionDomainReq => {
                serializer[0] = 0;
                return Result::Ok(size_of::<usize>());
            }
            Message::CreateProtectionDomainResp(pd_id) => {
                serializer[0] = 1;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = pd_id };
                return Result::Ok(2 * size_of::<usize>());
            }
            Message::DeleteProtectionDomainReq(pd_id) => {
                serializer[0] = 2;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = pd_id };
                return Result::Ok(2 * size_of::<usize>());
            }
            Message::DeleteProtectionDomainResp => {
                serializer[0] = 3;
                return Result::Ok(size_of::<usize>());
            }
        }
    }

    fn deserialize(deserializer: &[u8]) -> Result<Self, ()> {
        let message = deserializer[0];
        match message {
            0 => {
                return Result::Ok(Message::CreateProtectionDomainReq);
            }
            1 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let pd_id = unsafe { *ptr };
                return Result::Ok(Message::CreateProtectionDomainResp(pd_id));
            }
            2 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let pd_id = unsafe { *ptr };
                return Result::Ok(Message::DeleteProtectionDomainReq(pd_id));
            }
            3 => {
                return Result::Ok(Message::DeleteProtectionDomainResp);
            }

            _ => {
                unimplemented!()
            }
        }
    }
}
