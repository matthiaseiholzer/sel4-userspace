use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::buddy_allocator_tree::BuddyAllocator;
use std::vec::Vec;
use core::assert_eq;
use core::option::Option::None;

#[test]
fn alloc_3() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let mut allocator = BuddyAllocator::new(addr, 4);

    let mem = allocator.alloc(4).unwrap();

    assert_eq!(mem, (memory.as_ptr() as usize));

    for i in 0..BuddyAllocator::SIZE_MAX {
        assert_eq!(true, allocator.free[i as usize].is_empty());
    }
}

#[test]
fn alloc() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);
    let mem = allocator.alloc(2);

    assert_eq!(mem.unwrap(), memory.as_ptr() as usize);
}

#[test]
fn alloc_2() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let mut allocator = BuddyAllocator::new(addr, 4);

    for i in 0..4 {
        let mem = allocator.alloc(2).unwrap();

        assert_eq!(addr + i * 4 * BuddyAllocator::PAGE_SIZE, mem);
    }
}

#[test]
fn alloc_complete() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);
    let mem = allocator.alloc(4);

    assert_eq!(mem.unwrap(), memory.as_ptr() as usize);
}

#[test]
fn alloc_size_0() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);
    let mem = allocator.alloc(0);

    assert_eq!(None, mem);
}

#[test]
fn alloc_too_big() {
    let num_pages: usize = 16;
    let memory: Vec<u8> = Vec::with_capacity(num_pages * BuddyAllocator::PAGE_SIZE);

    let mut allocator = BuddyAllocator::new(memory.as_ptr() as usize, 4);
    let mem = allocator.alloc(5);

    assert_eq!(None, mem);
}

#[test]
fn alloc_15() {
    const SIZE_BITS: u8 = 12;
    let memory: Vec<u8> = Vec::with_capacity((1 << SIZE_BITS) * BuddyAllocator::PAGE_SIZE);

    let addr = memory.as_ptr() as usize;
    let mut allocator = BuddyAllocator::new(addr, SIZE_BITS);

    // std::process::Command::new("clear").status().unwrap();

    // let size = rng.gen_range(1..(SIZE_BITS - 6));
    let mem = allocator.alloc(2).unwrap();
    assert_eq!(addr, mem);
}
