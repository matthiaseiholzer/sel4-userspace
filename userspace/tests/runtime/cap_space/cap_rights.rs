use sel4_userspace::runtime::cap_space::cap_rights::CapRights;

#[test]
fn default() {
    let cap_rights = CapRights::default();

    assert_eq!(cap_rights.allow_grant_reply, true);

    assert_eq!(cap_rights.allow_grant, true);
    assert_eq!(cap_rights.allow_read, true);
    assert_eq!(cap_rights.allow_write, true);
}

#[test]
fn new() {
    let cap_rights = CapRights::new(true, false, true, false);

    assert_eq!(cap_rights.allow_grant_reply, true);
    assert_eq!(cap_rights.allow_grant, false);
    assert_eq!(cap_rights.allow_read, true);
    assert_eq!(cap_rights.allow_write, false);
}
