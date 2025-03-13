use super::sys::message_info::seL4_MessageInfo;
use sel4_us::runtime::kernel::MessageInfo;

// block seL4_MessageInfo {
//     field label 52
//     field capsUnwrapped 3
//     field extraCaps 2
//     field length 7
// }
impl From<seL4_MessageInfo> for MessageInfo {
    fn from(value: seL4_MessageInfo) -> Self {
        let mut content = value.words[0];

        let length = (content & 0x7F) as u8;
        content = content >> 7;

        let extra_caps = (content & 0x3) as u8;
        content = content >> 2;

        let caps_unwrapped = (content & 0x7) as u8;
        content = content >> 3;

        let label = content;

        MessageInfo {
            label,
            caps_unwrapped,
            extra_caps,
            length,
        }
    }
}

// block seL4_MessageInfo {
//     field label 52
//     field capsUnwrapped 3
//     field extraCaps 2
//     field length 7
// }
impl From<MessageInfo> for seL4_MessageInfo {
    fn from(value: MessageInfo) -> Self {
        let word = value.label << (3 + 2 + 7)
            | ((value.caps_unwrapped as u64) << (2 + 7))
            | ((value.extra_caps as u64) << 7)
            | (value.length as u64);

        seL4_MessageInfo { words: [word] }
    }
}
