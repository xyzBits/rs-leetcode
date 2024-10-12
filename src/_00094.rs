use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        Self::dfs(&root, &mut ans);
        ans
    }

    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let node = root.as_ref().unwrap().borrow();
        Self::dfs(&node.left, ans);
        ans.push(node.val);
        Self::dfs(&node.right, ans);
    }
}

#[cfg(test)]
mod tests {
    use crate::tree::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_rc_as_ref() {
        let rc = Rc::new(1);
        let x = rc.as_ref();

        let root = Rc::new(RefCell::new(TreeNode::new(1)));

        let root = Some(root);

        let option = root.as_ref();

        let x1 = option.unwrap();

        let x2 = x1.borrow();
    }

    #[test]
    fn test_1() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));

        let root = Some(root);
        let x = root.as_ref().unwrap().borrow();
        let x1 = &x;
    }
}
