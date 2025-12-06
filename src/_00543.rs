use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Self::depth(root, &mut max_diameter);
        max_diameter
    }

    fn depth(node: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }
        let rc_node = node.unwrap();
        let n = rc_node.borrow();
        let left_height = Self::depth(n.left.clone(), max_diameter);
        let right_height = Self::depth(n.right.clone(), max_diameter);
        let current_diameter = left_height + right_height;
        *max_diameter = (*max_diameter).max(current_diameter);

        left_height.max(right_height) + 1
    }
}
