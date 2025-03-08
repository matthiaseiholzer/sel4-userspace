use sel4_us::runtime::cap_space::CapData;

#[test]
fn new() {
    let guard = 1234;
    let guard_size = 11;
    let data = CapData::new(guard, guard_size);

    assert_eq!(data.guard_size, guard_size);
    assert_eq!(data.guard, guard);
}

#[test]
fn default() {
    let cap_data = CapData::default();

    assert_eq!(cap_data.guard_size, 0);
    assert_eq!(cap_data.guard, 0);
}

#[test]
fn into() {
    let guard_size: u8 = 57;
    let guard = 827348729;

    let cap_data = CapData::new(guard, guard_size);
    let cap_data_serialized = <CapData as Into<usize>>::into(cap_data);

    assert_eq!((cap_data_serialized >> 6), guard);
    assert_eq!((cap_data_serialized & ((1 << 6) - 1)) as u8, guard_size);
}
