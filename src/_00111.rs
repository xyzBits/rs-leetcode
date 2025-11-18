use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root_node = root.as_ref().unwrap().borrow();
        if root_node.left.is_none() && root_node.right.is_none() {
            return 1;
        }

        let mut min_depth = i32::MAX;
        if root_node.left.is_some() {
            min_depth = min_depth.min(Self::min_depth(root_node.left.clone()));
        }
        if root_node.right.is_some() {
            min_depth = min_depth.min(Self::min_depth(root_node.right.clone()));
        }

        min_depth + 1
    }
}
