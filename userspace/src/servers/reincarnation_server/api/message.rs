use crate::runtime::ipc::SerDe;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    Req,  //0
    Resp, //1,
}

impl SerDe for Message {
    /**
     */
    fn serialize(&self, serializer: &mut [u8]) -> Result<usize, ()> {
        match *self {
            Message::Req => {
                serializer[0] = 0;
                return Result::Ok(size_of::<usize>());
            }
            Message::Resp => {
                serializer[0] = 1;
                return Result::Ok(size_of::<usize>());
            }
        }
    }

    fn deserialize(deserializer: &[u8]) -> Result<Self, ()> {
        let message = deserializer[0];
        match message {
            0 => {
                return Result::Ok(Message::Req);
            }
            1 => {
                return Result::Ok(Message::Resp);
            }

            _ => {
                unimplemented!()
            }
        }
    }
}
