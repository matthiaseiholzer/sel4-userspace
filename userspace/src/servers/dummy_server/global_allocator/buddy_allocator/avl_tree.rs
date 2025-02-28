use core::clone::Clone;
use core::cmp::max;
use core::cmp::PartialOrd;
use core::cmp::{Eq, PartialEq};
use core::default::Default;
use core::fmt::Debug;
use core::option::Option::{self, None, Some};
use core::prelude::rust_2024::derive;
use core::{cmp::Ordering, ptr::null_mut};

#[derive(PartialEq, Eq, Clone)]
pub struct AvlTree {
    pub root: *mut TreeNode,
}

impl Default for AvlTree {
    fn default() -> Self {
        AvlTree { root: null_mut() }
    }
}

impl AvlTree {
    pub fn insert(&mut self, c: &mut TreeNode) {
        if self.root.is_null() {
            self.root = c as *mut TreeNode;
        } else {
            self.root = TreeNode::insert(self.root, c);
        }
    }

    pub fn remove<'a>(&mut self, c: &'a mut TreeNode) -> Option<&'a mut TreeNode> {
        if self.root.is_null() {
            return None;
        } else {
            self.root = TreeNode::remove(self.root, c);
            Some(c)
        }
    }

    pub fn get_smallest(&mut self) -> Option<&mut TreeNode> {
        if self.root.is_null() {
            None
        } else {
            Some(TreeNode::most_left_child(unsafe { &mut *self.root }))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_null()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct TreeNode {
    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
    pub height: usize,
    pub cap: usize,
}

impl Default for TreeNode {
    fn default() -> Self {
        TreeNode {
            left: null_mut(),
            right: null_mut(),
            height: 0,
            cap: 0,
        }
    }
}

impl TreeNode {
    pub fn create_node_at_addr(addr: usize) -> &'static mut TreeNode {
        let node = TreeNode::default();
        let node_ptr = addr as *mut TreeNode;
        unsafe {
            node_ptr.write(node);
            &mut *node_ptr as &'static mut TreeNode
        }
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn get_at_addr(addr: usize) -> &'static mut TreeNode {
        let node_ptr = addr as *mut TreeNode;
        unsafe { &mut *node_ptr as &'static mut TreeNode }
    }

    pub fn insert(node: *mut TreeNode, c: &mut TreeNode) -> *mut TreeNode {
        if node.is_null() {
            return c as *mut TreeNode;
        }

        let n = unsafe { &mut *node };

        if c > n {
            n.right = Self::insert(n.right, c);
        } else {
            n.left = Self::insert(n.left, c);
        }
        n.update_height();

        n.rebalance()
    }

    pub fn remove(node: *mut TreeNode, c: &mut TreeNode) -> *mut TreeNode {
        if node.is_null() {
            return node;
        }

        let mut n = unsafe { &mut *node };

        if c < n {
            n.left = Self::remove(n.left, c);
        } else if n < c {
            n.right = Self::remove(n.right, c);
        } else if n.left.is_null() && n.right.is_null() {
            return null_mut();
        } else if n.left.is_null() {
            n = unsafe { &mut *n.right };
        } else if n.right.is_null() {
            n = unsafe { &mut *n.left }
        } else {
            let most_left_child = Self::most_left_child(unsafe { &mut *n.right });
            let n_right = n.right;
            let n_left = n.left;

            let new_n_right = Self::remove(n_right, most_left_child);
            n = TreeNode::get_at_addr(most_left_child.start_addr());
            n.left = n_left;
            n.right = new_n_right;
        }

        n.update_height();
        n.rebalance()
    }

    pub fn most_left_child(mut node: &mut TreeNode) -> &mut TreeNode {
        while !node.left.is_null() {
            node = unsafe { &mut *node.left };
        }
        return node;
    }

    fn update_height(&mut self) {
        let height_left = Self::get_height(self.left);
        let height_right = Self::get_height(self.right);

        self.height = (max(height_right, height_left) + 1) as usize
    }

    pub fn rebalance(&mut self) -> *mut TreeNode {
        let balance_factor = self.balance_factor();
        if balance_factor < -1 {
            let left = unsafe { &mut *self.left };
            if left.balance_factor() <= 0 {
                return self.rotate_right();
            } else {
                self.left = left.rotate_left();
                return self.rotate_right();
            }
        };

        if balance_factor > 1 {
            let right = unsafe { &mut *self.right };
            if right.balance_factor() >= 0 {
                return self.rotate_left();
            } else {
                self.right = right.rotate_right();
                return self.rotate_left();
            }
        }

        return self as *mut TreeNode;
    }

    pub fn balance_factor(&self) -> i8 {
        let height_right = Self::get_height(self.right);
        let height_left = Self::get_height(self.left);
        (height_right - height_left) as i8
    }

    fn get_height(node: *mut TreeNode) -> isize {
        if !node.is_null() {
            unsafe { &*node }.height as isize
        } else {
            -1
        }
    }

    fn rotate_right(&mut self) -> *mut TreeNode {
        let l = unsafe { &mut *self.left };
        self.left = l.right;
        l.right = self;
        self.update_height();
        l.update_height();
        l as *mut TreeNode
    }

    fn rotate_left(&mut self) -> *mut TreeNode {
        let r = unsafe { &mut *self.right };
        self.right = r.left;
        r.left = self;
        self.update_height();
        r.update_height();
        r as *mut TreeNode
    }
}

impl PartialOrd for TreeNode {
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
