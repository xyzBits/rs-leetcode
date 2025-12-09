use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }

        if Self::is_same_tree_572(root.clone(), sub_root.clone()) {
            return true;
        }

        let rc_root = root.unwrap();
        let r = rc_root.borrow();

        Self::is_subtree(r.left.clone(), sub_root.clone())
            || Self::is_subtree(r.right.clone(), sub_root.clone())
    }

    fn is_same_tree_572(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(p_rc), Some(q_rc)) => {
                let p_node = p_rc.borrow();
                let q_node = q_rc.borrow();

                p_node.val == q_node.val
                    && Self::is_same_tree_572(p_node.left.clone(), q_node.left.clone())
                    && Self::is_same_tree_572(p_node.right.clone(), q_node.right.clone())
            }
        }
    }
}

#[test]
fn test_001() {
    let hello = Rc::new(RefCell::new("hello world".to_string()));
    let hello_ref = hello.as_ref();

    let hello_cell = RefCell::new("hello world".to_string());

    let hello_rc = Rc::new("hello world".to_string());
    let hello_rc_ref = hello_rc.as_ref();
}
