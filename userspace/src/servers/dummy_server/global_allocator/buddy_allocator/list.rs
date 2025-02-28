use core::clone::Clone;
use core::cmp::PartialOrd;
use core::cmp::{Eq, PartialEq};
use core::default::Default;
use core::fmt::Debug;
use core::option::Option::{self, None, Some};
use core::prelude::rust_2024::derive;
use core::{cmp::Ordering, ptr::null_mut};

#[derive(Debug, Clone)]
pub struct List {
    pub begin: *mut ListNode,
}

impl Default for List {
    fn default() -> Self {
        List { begin: null_mut() }
    }
}

impl List {
    pub fn push(&mut self, node: &'static mut ListNode) {
        let previous = self.begin;
        node.next = previous;
        self.begin = node;
    }

    pub fn pop(&mut self) -> Option<&'static mut ListNode> {
        if !self.begin.is_null() {
            let first = unsafe { &mut *self.begin };
            self.begin = first.next;
            Some(first)
        } else {
            None
        }
    }

    pub fn remove<'a>(&mut self, c: &'a ListNode) -> Option<&'a ListNode> {
        if !self.begin.is_null() {
            let mut previous = unsafe { &mut *self.begin };
            while !previous.next.is_null() {
                let next = unsafe { &mut *previous.next };
                if next.start_addr() == c.start_addr() {
                    previous.next = next.next;
                    return Some(c);
                } else {
                    previous = next;
                }
            }
            if previous.start_addr() == c.start_addr() {
                self.begin = null_mut();
                Some(previous)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.begin.is_null()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub next: *mut ListNode,
}

impl Default for ListNode {
    fn default() -> Self {
        ListNode { next: null_mut() }
    }
}

impl ListNode {
    pub fn create_node_at_addr(addr: usize) -> &'static mut ListNode {
        let node = ListNode::default();
        let node_ptr = addr as *mut ListNode;
        unsafe {
            node_ptr.write(node);
            &mut *node_ptr as &'static mut ListNode
        }
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn get_at_addr(addr: usize) -> &'static mut ListNode {
        let node_ptr = addr as *mut ListNode;
        unsafe { &mut *node_ptr as &'static mut ListNode }
    }
}

impl PartialOrd<Self> for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.start_addr() > other.start_addr() {
            Some(Ordering::Greater)
        } else {
            if self.start_addr() == other.start_addr() {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}
