use sel4_us::runtime::protection_domain::thread::Thread;
use sel4_us::runtime::protection_domain::thread::ThreadBuilder;

// fn thread1(thread: &mut Thread<K>) {}

// #[test]
// fn default_values() {
//     let thread_builder = ThreadBuilder::new(thread1);

//     assert_eq!(thread_builder.tcb_cap_addr, Option::None);
//     assert_eq!(
//         thread_builder.stack_size_bits,
//         ThreadBuilder::DEFAULT_STACK_SIZE_BITS
//     );
//     assert_eq!(
//         thread_builder.priority,
//         ThreadBuilder::DEFAULT_THREAD_PRIORITY
//     )
// }
