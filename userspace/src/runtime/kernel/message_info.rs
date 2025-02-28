#[derive(Default, PartialEq, Eq, Debug, Clone)]
#[repr(C, align(8))]
pub struct MessageInfo {
    pub label: u64,
    pub caps_unwrapped: u8,
    pub extra_caps: u8,
    pub length: u8,
}

// block seL4_MessageInfo {
//     field label 52
//     field capsUnwrapped 3
//     field extraCaps 2
//     field length 7
// }
impl From<usize> for MessageInfo {
    fn from(mut value: usize) -> Self {
        let mut content = value;

        let length = (content & 0x7F) as u8;
        content = content >> 7;

        let extra_caps = (content & 0x3) as u8;
        content = content >> 2;

        let caps_unwrapped = (content & 0x7) as u8;
        content = content >> 3;

        let label = content as u64;

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
impl From<MessageInfo> for usize {
    fn from(value: MessageInfo) -> usize {
        let word = value.label << (3 + 2 + 7)
            | ((value.caps_unwrapped as u64) << (2 + 7))
            | ((value.extra_caps as u64) << 7)
            | (value.length as u64);

        word as usize
    }
}
