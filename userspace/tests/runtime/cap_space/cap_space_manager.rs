use sel4_us::runtime::cap_space::CapSpaceManager;

#[test]
fn test0() {
    let csp = CapSpaceManager::default();

    //assert!(CapSpaceManager::)
}

#[test]
fn from_tuple() {
    let l2 = 1;
    let l1 = 2;
    let l0 = 3;
    let a = CapSpaceManager::cap_addr_l012(l0, l1, l2);
    let mut ref_addr = (CapSpaceManager::C_0_OFFSET << CapSpaceManager::L0_WIDTH_BITS) | l0;
    ref_addr = ref_addr << CapSpaceManager::L1_WIDTH_BITS | l1;
    ref_addr = ref_addr << CapSpaceManager::L2_WIDTH_BITS | l2;

    assert_eq!(a.addr, ref_addr)
}
