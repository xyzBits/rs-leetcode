use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_from_slice(&nums[..])
    }

    fn build_tree_from_slice(nums_slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums_slice.is_empty() {
            return None;
        }

        let mid = nums_slice.len() / 2;

        let mut root = TreeNode {
            val: nums_slice[mid],
            left: None,
            right: None,
        };
        root.left = Self::build_tree_from_slice(&nums_slice[..mid]);
        root.right = Self::build_tree_from_slice(&nums_slice[mid + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}
