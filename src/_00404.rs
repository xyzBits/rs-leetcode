use std::cell::RefCell;
use std::rc::Rc;
use crate::Solution;
use crate::tree::TreeNode;

impl Solution {
    // pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //
    //     if root.is_none() {
    //         0
    //     } else {
    //         Self::dfs_404(root)
    //     }
    // }
    //
    // fn dfs_404(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    //     let mut ans = 0;
    //     let root_node = root.as_ref().unwrap().borrow();
    //
    //     if root_node.left.is_some() {
    //         ans += if Self::is_leaf_node(root_node.left.clone()) {
    //             root_node.left.unwrap().borrow().val
    //         } else {
    //             Self::dfs_404(root_node.left.clone())
    //         }
    //     }
    //
    //
    //
    //
    //     ans
    // }
    //
    // fn is_leaf_node(node: Option<Rc<RefCell<TreeNode>>>) -> bool {
    //     true
    // }
}