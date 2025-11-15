use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root_node = root.as_ref().unwrap().borrow();

        Self::is_mirror(root_node.left.clone(), root_node.right.clone())
    }

    fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }

        if left.is_none() || right.is_none() {
            return false;
        }

        // 到这里，t1 t2 都是 some
        let left_node = left.as_ref().unwrap().borrow();
        let right_node = right.as_ref().unwrap().borrow();

        if left_node.val != right_node.val {
            return false;
        }

        Self::is_mirror(left_node.left.clone(), right_node.right.clone())
            && Self::is_mirror(left_node.right.clone(), right_node.left.clone())
    }
}
