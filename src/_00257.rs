use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn binary_tree_paths_(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Self::construct_paths(root, "".to_string(), &mut result);
        result
    }

    fn construct_paths(root: Option<Rc<RefCell<TreeNode>>>, path: String, paths: &mut Vec<String>) {
        if let Some(root) = root {
            let root_node = root.borrow();
            let mut new_path = String::from(path);
            new_path.push_str(root_node.val.to_string().as_str());
            if root_node.left.is_none() && root_node.right.is_none() {
                paths.push(new_path);
            } else {
                new_path.push_str("->");
                Self::construct_paths(root_node.left.clone(), new_path.clone(), paths);
                Self::construct_paths(root_node.right.clone(), new_path.clone(), paths);
            }
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = Vec::new();
        if root.is_none() {
            return paths;
        }

        let mut queue = VecDeque::new();
        let root_rc = root.unwrap();
        let root_val = root_rc.borrow().val.to_string();
        queue.push_back((root_rc, root_val));

        while !queue.is_empty() {
            let (node_rc, current_path) = queue.pop_front().unwrap();
            let node = node_rc.borrow();
            if node.left.is_none() && node.right.is_none() {
                paths.push(current_path);
            } else {
                if let Some(left) = &node.left {
                    let mut new_path = current_path.clone();
                    new_path.push_str("->");
                    new_path.push_str(&left.borrow().val.to_string());

                    queue.push_back((left.clone(), new_path));
                }

                if let Some(right) = &node.right {
                    let mut new_path = current_path.clone();
                    new_path.push_str("->");
                    new_path.push_str(&right.borrow().val.to_string());

                    queue.push_back((right.clone(), new_path));
                }
            }
        }

        paths
    }
}
