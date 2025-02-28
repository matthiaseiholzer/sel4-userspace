use sel4_userspace::runtime::va_space::VASpaceManager;

#[test]
fn test_0() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 0;
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(2, res.2);
    assert_eq!(3, res.3);
}

#[test]
fn test_1() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 1;
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(2, res.2);
    assert_eq!(3, res.3);
}

#[test]
fn test_2() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 1 << VASpaceManager::PAGE_WIDTH;
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(2, res.2);
    assert_eq!(4, res.3);
}

#[test]
fn test_3() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 2 * (1 << VASpaceManager::PAGE_WIDTH);
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(2, res.2);
    assert_eq!(5, res.3);
}

#[test]
fn test_4() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 2 * (1 << VASpaceManager::PAGE_WIDTH);
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(2, res.2);
    assert_eq!(5, res.3);
}

#[test]
fn test_5() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 1 << (VASpaceManager::PAGE_WIDTH + VASpaceManager::L2_WIDTH);
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(515, res.2);
    assert_eq!(516, res.3);
}

#[test]
fn test_6() {
    let va_space_manager = VASpaceManager::default();

    let va_addr = 2 * (1 << (VASpaceManager::PAGE_WIDTH + VASpaceManager::L2_WIDTH));
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!(1, res.1);
    assert_eq!(1028, res.2);
    assert_eq!(1029, res.3);
}

#[test]
fn test_7() {
    let va_space_manager = VASpaceManager::default();

    let va_addr =
        1 << (VASpaceManager::PAGE_WIDTH + VASpaceManager::L2_WIDTH + VASpaceManager::L1_WIDTH);
    let res = va_space_manager.calc_cap_space_slots(va_addr);

    assert_eq!(0, res.0);
    assert_eq!((513 * 512) + 1, res.1);
    assert_eq!((513 * 512) + 2, res.2);
    assert_eq!((513 * 512) + 3, res.3);
}
