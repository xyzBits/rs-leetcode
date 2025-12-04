use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_diff = i32::MAX;
        let mut prev_val: Option<i32> = None;

        Self::inorder_traverse_530(root, &mut prev_val, &mut min_diff);

        min_diff
    }

    fn inorder_traverse_530(
        node: Option<Rc<RefCell<TreeNode>>>,
        prev_val: &mut Option<i32>,
        min_diff: &mut i32,
    ) {
        if node.is_none() {
            return;
        }

        let rc_node = node.unwrap();
        let n = rc_node.borrow();
        let current_val = n.val;

        Self::inorder_traverse_530(n.left.clone(), prev_val, min_diff);

        if let Some(prev) = *prev_val {
            let diff = current_val - prev;
            *min_diff = std::cmp::min(*min_diff, diff);
        }

        *prev_val = Some(current_val);

        Self::inorder_traverse_530(n.right.clone(), prev_val, min_diff);
    }
}
