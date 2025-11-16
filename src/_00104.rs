use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth_v1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            let root_node = root.as_ref().unwrap().borrow();
            let left_height = Self::max_depth_v1(root_node.left.clone());
            let right_height = Self::max_depth_v1(root_node.right.clone());
            left_height.max(right_height) + 1
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());
        let mut ans = 0;

        while !queue.is_empty() {
            let mut level_size = queue.len();
            while level_size > 0 {
                let node_rc = queue.pop_front().unwrap();
                let node = node_rc.borrow();
                if let Some(left_node) = node.left.clone() {
                    queue.push_back(left_node);
                }
                if let Some(right_node) = node.right.clone() {
                    queue.push_back(right_node);
                }
                level_size -= 1;
            }
            ans += 1;
        }

        ans
    }
}

// 确保您的 Solution 结构体在这里或在父模块中定义
// pub struct Solution;
// impl Solution { ... }

// -----------------------------------------------------------------
// 1. 将测试代码放入一个测试模块中
// -----------------------------------------------------------------
#[cfg(test)]
mod tests {
    // 2. 导入必要的模块
    use super::*; // 导入父模块的 Solution, TreeNode 等
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    // 3. 在测试模块中重新定义 TreeNode (或者确保它在父模块中是 pub)
    //    这里假设 TreeNode 在父模块中定义且 pub。
    //    type Node = Option<Rc<RefCell<TreeNode>>>;

    // -----------------------------------------------------------------
    // 4. 关键：构建树的辅助函数
    // -----------------------------------------------------------------
    /// 此函数将 LeetCode 的层序遍历数组 (null 用 None 表示)
    /// 转换为 Option<Rc<RefCell<TreeNode>>> 结构的树。
    fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if nodes.is_empty() || nodes[0].is_none() {
            return None;
        }

        // 辅助函数：创建一个新的 TreeNode 包装
        fn new_node(val: i32) -> Rc<RefCell<TreeNode>> {
            Rc::new(RefCell::new(TreeNode {
                val,
                left: None,
                right: None,
            }))
        }

        // 根节点
        let root_node = new_node(nodes[0].unwrap());

        // 使用 VecDeque (队列) 来执行层序遍历构建
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root_node.clone());

        let mut i = 1; // 数组的索引

        while !queue.is_empty() {
            let current_node_rc = queue.pop_front().unwrap();
            let mut current_node = current_node_rc.borrow_mut();

            // 1. 处理左子节点
            if i < nodes.len() {
                if let Some(left_val) = nodes[i] {
                    let left_node = new_node(left_val);
                    current_node.left = Some(left_node.clone());
                    queue.push_back(left_node);
                }
                // 如果 nodes[i] 是 None, current_node.left 保持 None
                i += 1;
            }

            // 2. 处理右子节点
            if i < nodes.len() {
                if let Some(right_val) = nodes[i] {
                    let right_node = new_node(right_val);
                    current_node.right = Some(right_node.clone());
                    queue.push_back(right_node);
                }
                // 如果 nodes[i] 是 None, current_node.right 保持 None
                i += 1;
            }
        }

        Some(root_node)
    }

    // -----------------------------------------------------------------
    // 5. 您的测试用例
    // -----------------------------------------------------------------
    #[test]
    fn test_max_depth_example_1() {
        // 输入: root = [3,9,20,null,null,15,7]
        // 对应 Rust:
        let tree_data = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        // 1. 构建树
        let root = build_tree(&tree_data);

        // 2. 调用您的解答 (假设测试 max_depth)
        let depth = Solution::max_depth(root);

        // 3. 断言结果
        // 这棵树的深度是 3
        assert_eq!(depth, 3);
    }

    #[test]
    fn test_is_symmetric_example() {
        // 假设测试 is_symmetric
        // 输入: [1,2,2,3,4,4,3]
        let tree_data = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ];

        let root = build_tree(&tree_data);
        assert_eq!(Solution::is_symmetric(root), true);
    }
}
