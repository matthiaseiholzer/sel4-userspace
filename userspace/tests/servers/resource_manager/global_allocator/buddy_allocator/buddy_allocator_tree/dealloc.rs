use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::buddy_allocator_tree::BuddyAllocator;
use std::vec::Vec;
use core::assert_eq;

#[test]
fn dealloc_0() {
    let size: u8 = 1;
    let num_pages: usize = 2 << size;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let allocator = BuddyAllocator::new(memory.as_ptr() as usize, size);

    assert_eq!(false, allocator.free[size as usize].is_empty())
}

#[test]
fn dealloc_1() {
    let size: u8 = 4;
    let num_pages: usize = 2 << size;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let allocator = BuddyAllocator::new(addr, size);

    assert_eq!(false, allocator.free[size as usize].is_empty());
    assert_eq!(true, {
        let first = unsafe { &mut *allocator.free[size as usize].root };
        first.start_addr() == addr
    });
}

#[test]
fn dealloc_block_without_buddy_block_lt_buddy() {
    let size: u8 = 4;
    let num_pages: usize = 2 << size;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let allocator = BuddyAllocator::new(addr, 3);

    assert_eq!(false, allocator.free[3].is_empty());
    assert_eq!(true, {
        let first = unsafe { &mut *allocator.free[3].root };
        first.start_addr() == addr
    });
}

// #[test]
// fn dealloc_block_with_buddy_block_lt_buddy() {
//     let size: u8 = 4;
//     let num_pages: usize = 1 << size;
//     let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

//     let block_addr = memory.as_ptr() as usize;
//     let buddy_addr = memory.as_ptr() as usize + 8 * BuddyAllocator::PAGE_SIZE;

//     let mut allocator = BuddyAllocator::new(block_addr, 4);

//     assert_eq!(false, allocator.freelists[3].is_empty());
//     assert_eq!(true, {
//         let first = unsafe { &mut *allocator.freelists[3].begin };
//         first.start_addr() == buddy_addr
//     });

//     allocator.dealloc(block_addr, 3);

//     assert_eq!(true, allocator.freelists[3].is_empty());
//     assert_eq!(false, allocator.freelists[4].is_empty());
//     assert_eq!(true, {
//         let first = unsafe { &mut *allocator.freelists[4].begin };
//         first.start_addr() == block_addr
//     });
// }

#[test]
fn dealloc_block_without_buddy_block_gt_buddy() {
    let size: u8 = 4;
    let num_pages: usize = 1 << size;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let block_addr = memory.as_ptr() as usize + 8 * BuddyAllocator::PAGE_SIZE;
    let allocator = BuddyAllocator::new(block_addr, 3);

    assert_eq!(false, allocator.free[3].is_empty());
    assert_eq!(true, {
        let first = unsafe { &mut *allocator.free[3].root };
        first.start_addr() == block_addr
    });
}

#[test]
fn dealloc_block_with_buddy_block_gt_buddy() {
    let size: u8 = 4;
    let num_pages: usize = 2 << size;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let buddy_addr = memory.as_ptr() as usize;
    let block_addr = memory.as_ptr() as usize + 8 * BuddyAllocator::PAGE_SIZE;
    let mut allocator = BuddyAllocator::new(buddy_addr, 3);

    assert_eq!(false, allocator.free[3].is_empty());
    assert_eq!(true, {
        let first = unsafe { &mut *allocator.free[3].root };
        first.start_addr() == buddy_addr
    });

    allocator.dealloc(block_addr, 3);

    assert_eq!(true, allocator.free[3].is_empty());
    assert_eq!(false, allocator.free[4].is_empty());
    assert_eq!(true, {
        let first = unsafe { &mut *allocator.free[4].root };
        first.start_addr() == buddy_addr
    });
}

#[test]
fn dealloc() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);

    let mut mem: Vec<(usize, u8)> = Vec::new();
    for _ in 0..4 {
        let mem_alloc = allocator.alloc(2);

        mem.push((mem_alloc.unwrap(), 2));
    }

    mem.iter().for_each(|m| allocator.dealloc(m.0, m.1));
}

#[test]
fn dealloc_random_order() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);

    let mut mem: Vec<(usize, u8)> = Vec::new();
    for _ in 0..4 {
        let mem_alloc = allocator.alloc(2);

        mem.push((mem_alloc.unwrap(), 2));
    }

    mem.iter().for_each(|m| allocator.dealloc(m.0, m.1));
}
