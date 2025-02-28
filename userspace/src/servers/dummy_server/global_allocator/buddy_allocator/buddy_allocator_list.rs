use super::list::{List, ListNode};
use core::default::Default;
use core::option::Option::{self, None, Some};

pub struct BuddyAllocator {
    pub freelists: [List; Self::SIZE_MAX as usize + 1],
    pub start_addr: usize,
    pub size: u8,
}

impl BuddyAllocator {
    pub fn new(addr: usize, size: u8) -> BuddyAllocator {
        let mut allocator = BuddyAllocator {
            freelists: core::array::from_fn(|_| List::default()),
            start_addr: addr,
            size,
        };

        allocator.dealloc(addr, size);

        allocator
    }
}

impl BuddyAllocator {
    const PAGE_SIZE_BITS: u8 = 12;
    pub const PAGE_SIZE: usize = (2 << Self::PAGE_SIZE_BITS);
    pub const SIZE_MAX: u8 = 64 - Self::PAGE_SIZE_BITS;

    pub fn alloc(&mut self, size: u8) -> Option<usize> {
        if size > Self::SIZE_MAX || size == 0 {
            return None;
        }

        if let Some(block) = self.freelists[size as usize].pop() {
            Some(block.start_addr())
        } else {
            let block = self.alloc(size + 1);

            if let Some(b) = block {
                let buddy_addr = self.buddy_of(b, size);

                self.freelists[size as usize].push(ListNode::create_node_at_addr(buddy_addr));
            }

            block
        }
    }

    pub fn buddy_of(&self, block_addr: usize, size: u8) -> usize {
        let b = block_addr - self.start_addr;
        let pattern = 2 << Self::PAGE_SIZE_BITS << size;
        let buddy = b ^ pattern;
        buddy + self.start_addr
    }

    pub fn block_size(size: u8) -> usize {
        1 << Self::PAGE_SIZE_BITS << size
    }

    pub fn dealloc(&mut self, pages_addr: usize, size: u8) {
        if size > Self::SIZE_MAX || size == 0 {
            return;
        }
        let block = ListNode::create_node_at_addr(pages_addr);

        let buddy_addr = self.buddy_of(pages_addr, size);
        let buddy = ListNode::get_at_addr(buddy_addr);

        if let Some(_) = self.freelists[size as usize].remove(buddy) {
            if block > buddy {
                self.dealloc(buddy.start_addr(), size + 1);
            } else {
                self.dealloc(block.start_addr(), size + 1);
            }
        } else {
            self.freelists[size as usize].push(block);
        }
    }
}
