use core::default::Default;
use sel4_userspace::runtime::lib::array::Array;

#[test]
fn default() {
    const LEN: usize = 10;
    let array = Array::<usize, LEN>::default();

    assert_eq!(array.len, 0);
    assert_eq!(array.data, [0; LEN]);
}

#[test]
fn push_empty() {
    const LEN: usize = 4;
    let mut array = Array::<usize, LEN>::default();

    assert_eq!(array.len, 0);
    assert_eq!(array.data, [0; LEN]);

    array.push(5);

    assert_eq!(array.len, 1);
    assert_eq!(array.data, [5, 0, 0, 0]);
}

#[test]
fn pop() {
    const LEN: usize = 4;
    let mut array = Array::<usize, LEN>::default();

    assert_eq!(array.len, 0);
    assert_eq!(array.data, [0; LEN]);

    let value = 4;
    let _ = array.push(value);

    assert_eq!(array.len, 1);
    assert_eq!(array.data, [value, 0, 0, 0]);

    assert_eq!(array.pop(), value);
}
