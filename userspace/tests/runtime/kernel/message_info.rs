use sel4_us::runtime::kernel::MessageInfo;

#[test]
fn default() {
    let message_info = MessageInfo::default();

    let message_info_ref = MessageInfo {
        label: 0,
        caps_unwrapped: 0,
        extra_caps: 0,
        length: 0,
    };

    assert_eq!(message_info, message_info_ref);
}
