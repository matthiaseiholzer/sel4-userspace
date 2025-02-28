use sel4_userspace::runtime::lib::array_string::ArrayString;
use sel4_userspace::servers::reincarnation_server::api::ReincarnationServerMessage;

#[repr(align(8))]
struct MessageBuffer {
    pub buffer: [u8; 50],
}

impl Default for MessageBuffer {
    fn default() -> MessageBuffer {
        MessageBuffer { buffer: [0; 50] }
    }
}

#[test]
fn serialize_alloc_request() {
    let alloc_pages = 726363;
    let msg = ReincarnationServerMessage::AllocRequest(alloc_pages);

    let mut buffer = MessageBuffer::default();

    msg.serialize(&mut buffer.buffer);

    assert_eq!(buffer.buffer[0], 0);
    assert_eq!(
        unsafe { *((&mut buffer.buffer[8] as *mut u8) as *mut usize) },
        alloc_pages
    );
}

#[test]
fn deserialize_alloc_request() {
    let alloc_pages = 726363;
    let msg_ref = ReincarnationServerMessage::AllocRequest(alloc_pages);

    let mut buffer = MessageBuffer::default();

    msg_ref.serialize(&mut buffer.buffer);

    let msg = ReincarnationServerMessage::deserialize(&buffer.buffer).unwrap();

    assert_eq!(msg_ref, msg);
}

#[test]
fn serialize_alloc_response() {
    let address = 726363;
    let msg = ReincarnationServerMessage::AllocResponse(address);

    let mut buffer = MessageBuffer::default();

    let length = msg.serialize(&mut buffer.buffer).unwrap();

    assert_eq!(length, 16);
    assert_eq!(buffer.buffer[0], 1);
    assert_eq!(
        unsafe { *((&mut buffer.buffer[8] as *mut u8) as *mut usize) },
        address
    );
}

#[test]
fn deserialize_alloc_response() {
    let address = 726363;
    let msg_ref = ReincarnationServerMessage::AllocResponse(address);

    let mut buffer = MessageBuffer::default();

    msg_ref.serialize(&mut buffer.buffer);

    let msg = ReincarnationServerMessage::deserialize(&buffer.buffer).unwrap();

    assert_eq!(msg_ref, msg);
}

#[test]
fn serialize_set_debug_name_request() {
    let name = "debug_name";
    let mut name_owned = ArrayString::<20>::default();
    name_owned.push_str(name);

    let msg = ReincarnationServerMessage::SetDebugNameRequest(name_owned);

    let mut serialized: [u8; 50] = [0; 50];
    msg.serialize(&mut serialized);

    assert_eq!(serialized[0], 4);
    assert_eq!(serialized[1], 10);
    assert_eq!(serialized[2], 'd' as u8);
    assert_eq!(serialized[11], 'e' as u8);
}

#[test]
fn deserialize_set_debug_name_request() {
    let name = "debug_name";
    let mut name_owned = ArrayString::<20>::default();
    name_owned.push_str(name);

    let msg_ref = ReincarnationServerMessage::SetDebugNameRequest(name_owned);

    let mut buffer = MessageBuffer::default();
    let _ = msg_ref.serialize(&mut buffer.buffer);

    let msg = ReincarnationServerMessage::deserialize(&buffer.buffer).unwrap();

    assert_eq!(msg_ref, msg);
}

#[test]
fn serialize_set_debug_name_response() {
    let msg = ReincarnationServerMessage::SetDebugNameResponse();

    let mut serialized: [u8; 50] = [0; 50];
    msg.serialize(&mut serialized);

    assert_eq!(serialized[0], 5);
}

#[test]
fn deserialize_set_debug_name_response() {
    let msg_ref = ReincarnationServerMessage::SetDebugNameResponse();

    let mut buffer = MessageBuffer::default();
    let _ = msg_ref.serialize(&mut buffer.buffer);

    let msg = ReincarnationServerMessage::deserialize(&buffer.buffer).unwrap();

    assert_eq!(msg_ref, msg);
}
