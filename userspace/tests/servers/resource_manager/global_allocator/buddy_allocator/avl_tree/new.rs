use global_allocator::buddy_allocator::{
    avl_tree::{AvlTree, TreeNode},
    buddy_allocator::BuddyAllocator,
};
use std::ptr::null;

#[test]
fn new() {
    let tree = AvlTree::default();

    assert_eq!(null(), tree.root)
}