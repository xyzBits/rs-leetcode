use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        Self::preorder_helper(root, &mut result);

        result
    }

    fn preorder_helper(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root_node = root.as_ref().unwrap().borrow();
        result.push(root_node.val);
        Self::preorder_helper(root_node.left.clone(), result);
        Self::preorder_helper(root_node.right.clone(), result);
    }
}
