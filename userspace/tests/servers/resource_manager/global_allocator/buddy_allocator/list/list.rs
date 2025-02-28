use core::assert_eq;
use core::default::Default;
use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::{
    buddy_allocator_list::BuddyAllocator,
    list::{List, ListNode},
};

use core::option::Option::Some;

#[test]
fn push_on_empty_list() {
    let memory: [u8; 10 * BuddyAllocator::PAGE_SIZE] = [0; 10 * BuddyAllocator::PAGE_SIZE];

    let mut list = List::default();
    let node = ListNode::create_node_at_addr(memory.as_ptr() as usize);

    list.push(node);

    assert_eq!(false, list.begin.is_null());

    assert_eq!(true, {
        if let Some(n) = list.pop() {
            n.start_addr() == memory.as_ptr() as usize
        } else {
            false
        }
    })
}

#[test]
fn push() {
    let memory: [u8; 10 * BuddyAllocator::PAGE_SIZE] = [0; 10 * BuddyAllocator::PAGE_SIZE];

    let mut list = List::default();
    let node1_addr = memory.as_ptr() as usize;
    let node1 = ListNode::create_node_at_addr(node1_addr);
    let node2_addr = (&memory[1 * BuddyAllocator::PAGE_SIZE] as *const u8) as usize;
    let node2 = ListNode::create_node_at_addr(node2_addr);

    list.push(node1);
    list.push(node2);

    assert_eq!(false, list.begin.is_null());

    assert_eq!(true, {
        if let Some(n) = list.pop() {
            n.start_addr() == node2_addr as usize
        } else {
            false
        }
    });

    assert_eq!(true, {
        if let Some(n) = list.pop() {
            n.start_addr() == node1_addr as usize
        } else {
            false
        }
    })
}

#[test]
fn remove_on_empty_list() {
    let memory: [u8; 10 * BuddyAllocator::PAGE_SIZE] = [0; 10 * BuddyAllocator::PAGE_SIZE];

    let mut list = List::default();
    let node1_addr = memory.as_ptr() as usize;
    let node1 = ListNode::create_node_at_addr(node1_addr);

    let res = list.remove(&node1);

    assert_eq!(true, res.is_none());
}

#[test]
fn remove_0() {
    let memory: [u8; 10 * BuddyAllocator::PAGE_SIZE] = [0; 10 * BuddyAllocator::PAGE_SIZE];

    let mut list = List::default();
    let node1_addr = memory.as_ptr() as usize;
    let node1 = ListNode::create_node_at_addr(node1_addr);
    let node2_addr = (&memory[1 * BuddyAllocator::PAGE_SIZE] as *const u8) as usize;
    let node2 = ListNode::create_node_at_addr(node2_addr);

    let node3 = ListNode::create_node_at_addr(node1_addr);
    let node4 = ListNode::create_node_at_addr(node2_addr);

    list.push(node1);
    list.push(node2);

    assert_eq!(true, {
        let first = unsafe { &mut *list.begin };
        let second = unsafe { &mut *first.next };
        first.start_addr() == node2_addr
            && second.start_addr() == node1_addr
            && second.next.is_null()
    });

    assert_eq!(
        true,
        if let Some(n) = list.remove(node3) {
            n.start_addr() == node1_addr && n.next.is_null() && !list.begin.is_null()
        } else {
            false
        } && if let Some(n) = list.remove(node4) {
            n.start_addr() == node2_addr && n.next.is_null() && list.begin.is_null()
        } else {
            false
        }
    );
}

#[test]
fn remove_1() {
    let memory: [u8; 10 * BuddyAllocator::PAGE_SIZE] = [0; 10 * BuddyAllocator::PAGE_SIZE];

    let mut list = List::default();
    let node0_addr = memory.as_ptr() as usize;
    let node0 = ListNode::create_node_at_addr(node0_addr);
    let node0a = ListNode::get_at_addr(node0_addr);

    let node1_addr = (&memory[1 * BuddyAllocator::PAGE_SIZE] as *const u8) as usize;
    let node1 = ListNode::create_node_at_addr(node1_addr);
    let node1a = ListNode::get_at_addr(node1_addr);

    let node2_addr = (&memory[2 * BuddyAllocator::PAGE_SIZE] as *const u8) as usize;
    let node2 = ListNode::create_node_at_addr(node2_addr);
    let node2a = ListNode::create_node_at_addr(node2_addr);

    let node3_addr = (&memory[3 * BuddyAllocator::PAGE_SIZE] as *const u8) as usize;
    let node3 = ListNode::create_node_at_addr(node3_addr);
    let node3a = ListNode::get_at_addr(node3_addr);

    list.push(node0);
    list.push(node1);
    list.push(node2);
    list.push(node3);

    assert_eq!(true, {
        let first = unsafe { &mut *list.begin };
        let second = unsafe { &mut *first.next };
        first.start_addr() == node3_addr
            && second.start_addr() == node2_addr
            && unsafe { &mut *node3a.next } == node2a
            && unsafe { &mut *node2a.next } == node1a
            && unsafe { &mut *node1a.next } == node0a
            && node0a.next.is_null()
    });

    assert_eq!(
        true,
        if let Some(n) = list.remove(node2a) {
            let first = unsafe { &mut *list.begin };
            n.start_addr() == node2_addr
                && first == node3a
                && unsafe { &mut *node3a.next } == node1a
                && unsafe { &mut *node1a.next } == node0a
                && node0a.next.is_null()
        } else {
            false
        } // } && if let Some(n) = list.remove(node1a) {
          //     n.start_addr() == node1_addr && n.next.is_null() && list.begin.is_null()
          // } else {
          //     false
          // }
    );
}
