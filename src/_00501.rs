use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn find_mode(root: TreeNodePtr) -> Vec<i32> {
        // 对应 Java 的 class fields/state:
        let mut modes: Vec<i32> = Vec::new(); // answer
        let mut max_count = 0; // maxCount
        let mut current_count = 0; // count
        let mut prev_val: Option<i32> = None; // 对应 Java 的 base (使用 Option 处理起始状态更安全)

        // 启动中序遍历 (结合了 Java 的 findMode -> dfs)
        Self::inorder_traverse(
            root,
            &mut prev_val,
            &mut current_count,
            &mut max_count,
            &mut modes,
        );

        // 返回结果 (对应 Java 最后的数组转换)
        modes
    }

    // 助手函数：实现了 Java 中的 dfs 和 update 方法的逻辑
    fn inorder_traverse(
        node: TreeNodePtr,
        prev_val: &mut Option<i32>, // 追踪前一个节点的值 (base)
        current_count: &mut i32,    // 当前序列计数 (count)
        max_count: &mut i32,        // 全局最大计数 (maxCount)
        modes: &mut Vec<i32>,       // 结果列表 (answer)
    ) {
        if node.is_none() {
            return;
        }

        let rc_node = node.unwrap();
        let n = rc_node.borrow();
        let x = n.val;

        // 1. DFS Left (左子树)
        Self::inorder_traverse(n.left.clone(), prev_val, current_count, max_count, modes);

        // 2. Process Current Node (处理当前节点，对应 Java 的 update(o.val))

        // ** 计数逻辑 **
        if let Some(prev) = *prev_val {
            if x == prev {
                *current_count += 1; // 值相同，计数递增
            } else {
                *current_count = 1; // 值不同，开始新的计数
            }
        } else {
            // 第一个访问的节点
            *current_count = 1;
        }

        // ** 更新 modes 列表 **
        if *current_count == *max_count {
            // 找到另一个众数
            modes.push(x);
        } else if *current_count > *max_count {
            // 找到新的最大频率，更新 max_count，清空旧列表，并添加新值
            *max_count = *current_count;
            modes.clear();
            modes.push(x);
        }

        // 更新 prev_val (base)
        *prev_val = Some(x);

        // 3. DFS Right (右子树)
        Self::inorder_traverse(n.right.clone(), prev_val, current_count, max_count, modes);
    }
}
