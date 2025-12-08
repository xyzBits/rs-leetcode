use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut total_tilt = 0;
        Self::dfs_helper_563(root, &mut total_tilt);
        total_tilt
    }

    fn dfs_helper_563(root: Option<Rc<RefCell<TreeNode>>>, total_tilt: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let rc = root.unwrap();
        let root_node = rc.borrow();
        let left_sum = Self::dfs_helper_563(root_node.left.clone(), total_tilt);
        let right_sum = Self::dfs_helper_563(root_node.right.clone(), total_tilt);
        *total_tilt += (left_sum - right_sum).abs();

        left_sum + right_sum + root_node.val
    }
}
