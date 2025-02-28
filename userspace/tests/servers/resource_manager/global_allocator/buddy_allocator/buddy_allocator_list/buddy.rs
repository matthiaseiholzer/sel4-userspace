use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::buddy_allocator_list::BuddyAllocator;
use std::vec::Vec;
use core::assert_eq;

#[test]
fn buddy() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let allocator = BuddyAllocator::new(addr, 4);

    let buddy_addr = allocator.buddy_of(addr, 2);
    assert_eq!(
        buddy_addr,
        (memory.as_ptr() as usize) + 4 * BuddyAllocator::PAGE_SIZE
    );
}

#[test]
fn buddy_2() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let allocator = BuddyAllocator::new(addr, 4);

    let buddy_addr = allocator.buddy_of(addr, 3);
    assert_eq!(
        buddy_addr,
        (memory.as_ptr() as usize) + 8 * BuddyAllocator::PAGE_SIZE
    );
}

#[test]
fn buddy_3() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];
    let addr = memory.as_ptr() as usize;
    let allocator = BuddyAllocator::new(addr, 4);
    {
        let buddy_addr = allocator.buddy_of(addr, 3);
        assert_eq!(buddy_addr, addr + 8 * BuddyAllocator::PAGE_SIZE);
    }
    {
        let buddy_addr = memory.as_ptr() as usize;
        let addr = allocator.buddy_of(buddy_addr, 3);

        assert_eq!(addr, buddy_addr + 8 * BuddyAllocator::PAGE_SIZE);
    }
}
