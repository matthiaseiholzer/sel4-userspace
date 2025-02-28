use sel4_userspace::runtime::protection_domain::thread::{ThreadId, ThreadIdManager};

#[test]
fn alloc_id_empty() {
    let mut thread_id_manager = ThreadIdManager::default();
    let thread_id = thread_id_manager.alloc_id().unwrap();

    assert_eq!(0 as ThreadId, thread_id);
}

#[test]
fn alloc_id_full() {
    let mut thread_id_manager = ThreadIdManager::default();

    for i in 0..ThreadIdManager::MAX_NUM_THREADS {
        let thread_id = thread_id_manager.alloc_id().unwrap();

        assert_eq!(i as ThreadId, thread_id);
    }

    assert_eq!(thread_id_manager.alloc_id(), Err(()));
}

#[test]
fn free_non_empty_empty() {
    let mut thread_id_manager = ThreadIdManager::default();
    {
        let thread_id = thread_id_manager.alloc_id().unwrap();

        assert_eq!(0 as ThreadId, thread_id);
    }
    {
        thread_id_manager.free_id(0);
        let thread_id = thread_id_manager.alloc_id().unwrap();

        assert_eq!(0 as ThreadId, thread_id);
    }
}

#[test]
fn free_non_sequential() {
    let mut thread_id_manager = ThreadIdManager::default();
    {
        let thread_id_0 = thread_id_manager.alloc_id().unwrap();

        assert_eq!(0 as ThreadId, thread_id_0);

        let thread_id_1 = thread_id_manager.alloc_id().unwrap();

        assert_eq!(1 as ThreadId, thread_id_1);
    }
    {
        thread_id_manager.free_id(0);
        let thread_id = thread_id_manager.alloc_id().unwrap();

        assert_eq!(0 as ThreadId, thread_id);
    }
}
