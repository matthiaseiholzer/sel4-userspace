use core::{default, mem::size_of};

use crate::runtime::lib::array_string::ArrayString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReincarnationServerMessage {
    AllocRequest(usize), //0
    AllocResponse(usize),
    DeallocRequest(usize), // 2
    DeallocResponse(usize),
    SetDebugNameRequest(ArrayString<20>), //4
    SetDebugNameResponse(),
}

impl ReincarnationServerMessage {
    pub fn serialize(&self, serializer: &mut [u8]) -> Result<usize, ()> {
        match *self {
            ReincarnationServerMessage::AllocRequest(num_pages) => {
                serializer[0] = 0;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = num_pages };
                return Result::Ok(16);
            }
            ReincarnationServerMessage::AllocResponse(address) => {
                serializer[0] = 1;
                let ptr = (&mut serializer[8] as *mut u8) as *mut usize;
                unsafe { *ptr = address };
                return Result::Ok(16);
            }
            ReincarnationServerMessage::DeallocRequest(_) => Err(()),
            ReincarnationServerMessage::DeallocResponse(_) => Err(()),
            ReincarnationServerMessage::SetDebugNameRequest(ref name) => {
                serializer[0] = 4;
                serializer[1] = name.len() as u8;
                unsafe {
                    core::ptr::copy(
                        name.as_ptr(),
                        &mut serializer[2] as *mut u8,
                        serializer[1] as usize,
                    );
                };
                Ok(2 + name.len())
            }
            ReincarnationServerMessage::SetDebugNameResponse() => {
                serializer[0] = 5;
                Ok(1)
            }
        }
    }

    pub fn deserialize(deserializer: &[u8]) -> Result<ReincarnationServerMessage, ()> {
        let message = deserializer[0];
        match message {
            0 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let num_pages = unsafe { *ptr };
                return Result::Ok(ReincarnationServerMessage::AllocRequest(num_pages));
            }
            1 => {
                let ptr = (&deserializer[8] as *const u8) as *const usize;
                let address = unsafe { *ptr };
                return Result::Ok(ReincarnationServerMessage::AllocResponse(address));
            }
            4 => {
                let len: u8 = deserializer[1];
                let mut array: ArrayString<20> = ArrayString::<20>::default();
                for idx in 2..2 + len {
                    array.push(deserializer[idx as usize] as char);
                }

                return Result::Ok(ReincarnationServerMessage::SetDebugNameRequest(array));
            }
            5 => {
                return Result::Ok(ReincarnationServerMessage::SetDebugNameResponse());
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
