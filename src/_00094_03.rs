use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut work_list = LinkedList::new();
        let mut result = vec![];
        let mut current = root;

        while current.is_some() || !work_list.is_empty() {
            // 遍历左子树
            while let Some(node) = current {
                current = node.borrow_mut().left.take();
                work_list.push_back(node);
            }

            // 处理当前节点
            if let Some(node) = work_list.pop_back() {
                result.push(node.borrow().val);
                current = node.borrow_mut().right.take();
            }
        }
        result
    }
}
