use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        Self::postorder_helper(root, &mut result);

        result
    }

    fn postorder_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root_node = root.as_ref().unwrap().borrow();
        Self::postorder_helper(root_node.left.clone(), result);
        Self::postorder_helper(root_node.right.clone(), result);
        result.push(root_node.val);
    }
}
