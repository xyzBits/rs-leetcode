use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::rc::Rc;

impl Solution {
    pub fn search_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_some() {
            let v = root.as_ref().unwrap().borrow().val;
            match v.cmp(&val) {
                Greater => Self::search_bst(root.as_mut().unwrap().borrow_mut().left.take(), val),
                Less => Self::search_bst(root.as_mut().unwrap().borrow_mut().right.take(), val),
                Equal => root,
            }
        } else {
            root
        }
    }
}
