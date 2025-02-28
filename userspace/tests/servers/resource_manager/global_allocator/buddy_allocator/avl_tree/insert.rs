use std::ptr::null;
use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::avl_tree::{TreeNode, AvlTree};
use core::assert_eq;
use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::buddy_allocator_list::BuddyAllocator;
use core::default::Default;

#[test]
fn insert_empty_tree() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node = TreeNode::create_node_at_addr(memory.as_ptr() as usize);

    tree.insert(node);
    assert_eq!(node as *mut TreeNode, tree.root);
    assert_eq!(0, node.height);
    assert_eq!(0, node.balance_factor());
}

#[test]
fn insert_lower() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();
    let root_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;

    let root = TreeNode::create_node_at_addr(root_addr);
    tree.insert(root);

    let node = TreeNode::create_node_at_addr(memory.as_ptr() as usize);
    tree.insert(node);

    assert_eq!(root as *mut TreeNode, tree.root);
    assert_eq!(1, root.height);
    assert_eq!(-1, root.balance_factor());
    assert_eq!(node as *mut TreeNode, root.left);
    assert_eq!(0, node.height);
    assert_eq!(0, node.balance_factor());
}

#[test]
fn insert_rotate_right() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0 = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[4 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    assert_eq!(node0 as *mut TreeNode, tree.root);
    assert_eq!(1, node0.height);
    assert_eq!(-1, node0.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node0.left);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());

    let node2_addr = memory.as_ptr() as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    assert_eq!(node1 as *mut TreeNode, tree.root);
    assert_eq!(1, node1.height);
    assert_eq!(0, node1.balance_factor());
    assert_eq!(node2 as *mut TreeNode, node1.left);
    assert_eq!(0, node2.height);
    assert_eq!(0, node2.balance_factor());
    assert_eq!(null(), node2.left);
    assert_eq!(null(), node2.right);
    assert_eq!(node0 as *mut TreeNode, node1.right);
    assert_eq!(0, node0.height);
    assert_eq!(0, node0.balance_factor());
    assert_eq!(null(), node0.left);
    assert_eq!(null(), node0.right);
}

#[test]
fn insert_rotate_right_double() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0 = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[4 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    assert_eq!(node0 as *mut TreeNode, tree.root);
    assert_eq!(1, node0.height);
    assert_eq!(-1, node0.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node0.left);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());

    let node2_addr = &memory[6 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    assert_eq!(node2 as *mut TreeNode, tree.root);
    assert_eq!(1, node2.height);
    assert_eq!(0, node2.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node2.left);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());
    assert_eq!(null(), node1.left);
    assert_eq!(null(), node1.right);
    assert_eq!(node0 as *mut TreeNode, node2.right);
    assert_eq!(0, node0.height);
    assert_eq!(0, node0.balance_factor());
    assert_eq!(null(), node0.left);
    assert_eq!(null(), node0.right);
}

#[test]
fn insert_rotate_left() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[0 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0 = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[4 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    assert_eq!(node0 as *mut TreeNode, tree.root);
    assert_eq!(1, node0.height);
    assert_eq!(1, node0.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node0.right);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());

    let node2_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    assert_eq!(node1 as *mut TreeNode, tree.root);
    assert_eq!(1, node1.height);
    assert_eq!(0, node1.balance_factor());
    assert_eq!(node2 as *mut TreeNode, node1.right);
    assert_eq!(0, node2.height);
    assert_eq!(0, node2.balance_factor());
    assert_eq!(null(), node2.left);
    assert_eq!(null(), node2.right);
    assert_eq!(node0 as *mut TreeNode, node1.left);
    assert_eq!(0, node0.height);
    assert_eq!(0, node0.balance_factor());
    assert_eq!(null(), node0.left);
    assert_eq!(null(), node0.right);
}

#[test]
fn insert_rotate_left_double() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[0 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0 = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    assert_eq!(node0 as *mut TreeNode, tree.root);
    assert_eq!(1, node0.height);
    assert_eq!(1, node0.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node0.right);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());

    let node2_addr = &memory[6 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    assert_eq!(node2 as *mut TreeNode, tree.root);
    assert_eq!(1, node2.height);
    assert_eq!(0, node2.balance_factor());
    assert_eq!(node1 as *mut TreeNode, node2.right);
    assert_eq!(0, node1.height);
    assert_eq!(0, node1.balance_factor());
    assert_eq!(null(), node1.left);
    assert_eq!(null(), node1.right);
    assert_eq!(node0 as *mut TreeNode, node2.left);
    assert_eq!(0, node0.height);
    assert_eq!(0, node0.balance_factor());
    assert_eq!(null(), node0.left);
    assert_eq!(null(), node0.right);
}

#[test]
fn insert_higher() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();
    let root_addr = memory.as_ptr() as usize;

    let root = TreeNode::create_node_at_addr(root_addr);
    tree.insert(root);

    let node_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node = TreeNode::create_node_at_addr(node_addr);
    tree.insert(node);

    assert_eq!(root as *mut TreeNode, tree.root);
    assert_eq!(1, root.height);
    assert_eq!(node as *mut TreeNode, root.right);
    assert_eq!(0, node.height);
}
