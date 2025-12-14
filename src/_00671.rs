use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root_rc = match root {
            None => return -1,
            Some(node) => node,
        };

        let min_val = root_rc.borrow().val;

        let mut second_min_val = i32::MAX;
        let mut found_second_min = false;
        let mut queue = VecDeque::new();
        queue.push_back(root_rc);

        while let Some(node_rc) = queue.pop_front() {
            let node = node_rc.borrow();

            if node.val > min_val {
                second_min_val = second_min_val.min(node.val);
                found_second_min = true;
            }

            if let Some(left) = &node.left {
                queue.push_back(Rc::clone(left));
            }

            if let Some(right) = &node.right {
                queue.push_back(right.clone());
            }
        }

        if found_second_min {
            second_min_val
        } else {
            -1
        }
    }
}
