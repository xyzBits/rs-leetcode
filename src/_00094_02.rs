use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            if root.is_some() {
                let next = root.as_mut().unwrap().borrow_mut().left.take();
                stack.push(root);
                root = next;
            } else {
                let mut node = stack.pop().unwrap();
                let mut node = node.as_mut().unwrap().borrow_mut();
                ans.push(node.val);
                root = node.right.take();
            }
        }

        ans
    }
}
