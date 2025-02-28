// #[test]
// fn api_test() {
//     fn thread1(thread: &mut Thread) {}

//     let kernel = DummyKernel::default();
//     let pd_id: Id = 1;
//     let mut pd = ProtectionDomain::new(
//         pd_id,
//         &kernel,
//         CapAddr::from(CapInit::InitThreadCNode as usize),
//     );

//     let mut thread_builder = ThreadBuilder::new(thread1);
//     thread_builder.stack_size(14);
//     pd.spawn(thread_builder);

//     let thread: &mut Thread = &mut pd.thread[0];
//     assert_eq!(thread.id, 0);
//     assert_eq!(thread.priority, ThreadBuilder::DEFAULT_THREAD_PRIORITY);
// }
