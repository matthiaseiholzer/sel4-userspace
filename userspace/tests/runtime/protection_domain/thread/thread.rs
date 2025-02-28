// use crate::runtime::protection_domain::DummyKernel;
// use sel4_userspace::runtime::cap_space::CapAddr;
// use sel4_userspace::runtime::protection_domain::thread::Thread;

// #[test]
// fn test() {
//     let id = 234;
//     let kernel = DummyKernel::default();
//     let priority = 224;
//     let tcb_cap_addr = CapAddr::from_const(72984545);
//     let uut = Thread::new(id, &kernel, priority, tcb_cap_addr);

//     assert_eq!(uut.id, id);
// }
