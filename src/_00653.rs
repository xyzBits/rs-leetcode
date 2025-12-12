use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn find_target_(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        // queue.push_back(root);

        queue.push_back(root.unwrap());

        while let Some(node_rc) = queue.pop_front() {
            // let node_rc = queue.pop_front().unwrap().unwrap();
            let node = node_rc.borrow();

            let complement = k - node.val;
            if set.contains(&complement) {
                return true;
            }

            set.insert(node.val);
            if let Some(left_rc) = &node.left {
                queue.push_back(left_rc.clone());
            }
            if let Some(right_rc) = &node.right {
                queue.push_back(right_rc.clone());
            }
        }

        false
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        if root.is_none() {
            return false;
        }

        let mut set = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let node_rc = queue.pop_front().unwrap().unwrap();
            let node = node_rc.as_ref().borrow();

            if set.contains(&(k - node.val)) {
                return true;
            }

            set.insert(node.val);
            if node.left.is_some() {
                queue.push_back(node.left.clone());
            }
            if node.right.is_some() {
                queue.push_back(node.right.clone());
            }
        }

        false
    }
}
