use sel4_us::runtime::cap_space::CapAddr;

#[test]
fn from_const() {
    let addr = 1234;
    let a = CapAddr::from_const(addr);
    assert_eq!(a.addr, addr)
}

#[test]
fn from() {
    let addr = 1234;
    let a = CapAddr::from(addr, 64);
    assert_eq!(a.addr, addr)
}

// #[test]
// fn from_tuple() {
//     let l2 = 1;
//     let l1 = 2;
//     let l0 = 3;
//     let a = CapAddr::from((l0, l1, l2));
//     let mut ref_addr = (CapSpaceManager::C_0_OFFSET << CapSpaceManager::L0_WIDTH_BITS) | l0;
//     ref_addr = ref_addr << CapSpaceManager::L1_WIDTH_BITS | l1;
//     ref_addr = ref_addr << CapSpaceManager::L2_WIDTH_BITS | l2;

//     assert_eq!(a.addr, ref_addr)
// }

// #[test]
// fn into_tuple() {
//     let l2 = 1;
//     let l1 = 2;
//     let l0 = 3;
//     let a = CapAddr::from((l0, l1, l2));

//     assert_eq!(
//         <CapAddr as Into<(usize, usize, usize)>>::into(a),
//         (l0, l1, l2)
//     )
// }
