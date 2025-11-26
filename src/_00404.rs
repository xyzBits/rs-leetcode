use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs_helper_404(&root, false)
    }

    fn dfs_helper_404(node: &Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        match node {
            None => 0,
            Some(rc_node) => {
                let node = rc_node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    return if is_left { node.val } else { 0 };
                }
                let left_sum = Self::dfs_helper_404(&node.left, true);

                let right_sum = Self::dfs_helper_404(&node.right, false);
                left_sum + right_sum
            }
        }
    }
}
