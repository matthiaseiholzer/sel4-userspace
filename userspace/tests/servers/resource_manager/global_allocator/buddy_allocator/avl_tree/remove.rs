use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::avl_tree::{TreeNode, AvlTree};
use core::assert_eq;
use sel4_userspace::servers::resource_manager::global_allocator::buddy_allocator::buddy_allocator_list::BuddyAllocator;
use core::default::Default;

#[test]
fn remove_left_rotation() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0 = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[4 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    let node2_addr = &memory[6 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    let node3_addr = &memory[7 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node3 = TreeNode::create_node_at_addr(node3_addr);
    tree.insert(node3);

    assert_eq!(2, unsafe { &mut *tree.root }.height);

    tree.remove(node1);

    assert_eq!(node3, unsafe { &mut *tree.root });
    assert_eq!(1, unsafe { &mut *tree.root }.height);
}

#[test]
fn remove_two_sub_children() {
    let memory: [u8; 16 * BuddyAllocator::PAGE_SIZE] = [0; 16 * BuddyAllocator::PAGE_SIZE];

    let mut tree = AvlTree::default();

    let node0_addr = &memory[8 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node0: &mut TreeNode = TreeNode::create_node_at_addr(node0_addr);
    tree.insert(node0);

    let node1_addr = &memory[4 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    let node1 = TreeNode::create_node_at_addr(node1_addr);
    tree.insert(node1);

    let node2_addr = &memory[6 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node2 = TreeNode::create_node_at_addr(node2_addr);
    tree.insert(node2);

    let node3_addr = &memory[7 * BuddyAllocator::PAGE_SIZE] as *const u8 as usize;
    memory.as_ptr() as usize;
    let node3 = TreeNode::create_node_at_addr(node3_addr);
    tree.insert(node3);

    assert_eq!(2, unsafe { &mut *tree.root }.height);
    assert_eq!(node2, unsafe { &mut *tree.root });

    tree.remove(node2);

    assert_eq!(node3, unsafe { &mut *tree.root });
    assert_eq!(1, node3.height);
    assert_eq!(node1, unsafe { &mut *node3.left });
    assert_eq!(node0, unsafe { &mut *node3.right });
    assert_eq!(true, node0.left.is_null());
    assert_eq!(true, node0.right.is_null());

    assert_eq!(0, node1.height);
    assert_eq!(0, node0.height);
}
