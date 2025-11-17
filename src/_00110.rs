use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        // 1. 获取当前节点的左右子节点和它们的高度
        let (left_h, right_h, left_child, right_child) = {
            // 访问内部数据需要借用 RefCell
            let node = root.as_ref().unwrap().borrow();

            // 对应 Java: height(root.left) 和 height(root.right)
            let left_h = Self::get_tree_height(node.left.clone());
            let right_h = Self::get_tree_height(node.right.clone());

            // 克隆子节点 Rc，以便在返回时进行递归调用
            (left_h, right_h, node.left.clone(), node.right.clone())
        };

        // 2. 检查当前节点是否平衡
        // 对应 Java: Math.abs(height(root.left) - height(root.right)) <= 1
        let current_node_is_balanced = (left_h - right_h).abs() <= 1;

        // 3. 递归检查左右子树
        // 对应 Java: isBalanced(root.left) && isBalanced(root.right)
        current_node_is_balanced && Self::is_balanced(left_child) && Self::is_balanced(right_child)
    }

    fn get_tree_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            let root_node = root.as_ref().unwrap().borrow();
            std::cmp::max(
                Self::get_tree_height(root_node.left.clone()),
                Self::get_tree_height(root_node.right.clone()),
            ) + 1
        }
    }
}
