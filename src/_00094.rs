use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal_v1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        Self::inorder(root, &mut res);

        res
    }

    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root_node = root.as_ref().unwrap().borrow();
        Self::inorder(root_node.left.clone(), res);
        res.push(root_node.val);
        Self::inorder(root_node.right.clone(), res);
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut stack = Vec::new();

        let mut current = root;

        // 当前节点不为空，或者栈不为空
        while current.is_some() || !stack.is_empty() {
            // 1. 向左探索到底，不断将节点压入栈，并走向左子节点
            while let Some(node) = current {
                // current 的所有权必须保留给 stack，所以要使用 clone
                stack.push(node.clone());

                current = node.borrow().left.clone();
            }

            if let Some(node) = stack.pop() {
                let node_ref = node.borrow();
                result.push(node_ref.val);

                current = node_ref.right.clone();
            }
        }

        result
    }
}
