use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut ans = None;
        let mut max_depth = -1;

        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            depth: i32,
            max_depth: &mut i32,
            ans: &mut Option<Rc<RefCell<TreeNode>>>,
        ) -> i32 {
            if let Some(node) = node {
                let x = node.borrow();
                let left_max_depth = dfs(&x.left, depth + 1, max_depth, ans);
                let right_max_depth = dfs(&x.right, depth + 1, max_depth, ans);
                if left_max_depth == right_max_depth && left_max_depth == *max_depth {
                    *ans = Some(node.clone());
                }
                left_max_depth.max(right_max_depth)
            } else {
                *max_depth = (*max_depth).max(depth);
                depth
            }
        }

        dfs(&root, 0, &mut max_depth, &mut ans);

        ans
    }
}
