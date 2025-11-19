use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum_(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let root_node = root.as_ref().unwrap().borrow();
        if root_node.left.is_none() && root_node.right.is_none() {
            return target_sum == root_node.val;
        }

        Self::has_path_sum(root_node.left.clone(), target_sum - root_node.val)
            || Self::has_path_sum(root_node.right.clone(), target_sum - root_node.val)
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(root_node) => {
                if root_node.borrow().left.is_none() && root_node.borrow().right.is_none() {
                    return root_node.borrow().val == target_sum;
                }
                Self::has_path_sum(
                    root_node.borrow().left.clone(),
                    target_sum - root_node.borrow().val,
                ) || Self::has_path_sum(
                    root_node.borrow().right.clone(),
                    target_sum - root_node.borrow().val,
                )
            }
            None => false,
        }
    }
}
