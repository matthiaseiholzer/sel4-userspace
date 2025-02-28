use super::buddy_allocator::buddy_allocator_tree::BuddyAllocator;
use crate::runtime::kernel::constants::CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS;
use core::clone::Clone;
use core::default::Default;
use core::option::Option;
use core::option::Option::None;
use core::prelude::rust_2024::derive;
use core::todo;

#[derive(Clone)]
pub struct GlobalAllocator {
    allocators: [BuddyAllocator; CONFIG_MAX_NUM_BOOTINFO_UNTYPED_CAPS],
}

impl Default for GlobalAllocator {
    fn default() -> Self {
        GlobalAllocator {
            allocators: core::array::from_fn(|_| BuddyAllocator::default()),
        }
    }
}

impl GlobalAllocator {
    pub fn alloc(&mut self, size: u8) -> Option<usize> {
        for alloc in self.allocators.iter_mut() {
            let res = alloc.alloc(size);

            if res.is_some() {
                return res;
            }
        }

        None
    }

    pub fn dealloc(&mut self, _pages_addr: usize, _size: u8) {
        todo!()
    }
}
