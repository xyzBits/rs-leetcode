use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_same_tree_v1(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            _ => false,
        }
    }

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }

        if p.is_none() || q.is_none() {
            return false;
        }

        let p_node = p.as_ref().unwrap().borrow();
        let q_node = q.as_ref().unwrap().borrow();
        if p_node.val != q_node.val {
            return false;
        }

        Self::is_same_tree(p_node.left.clone(), q_node.left.clone())
            && Self::is_same_tree(p_node.right.clone(), q_node.right.clone())
    }
}

#[test]
fn test_001() {
    let data = Some(String::from("hello world"));
    // as_ref 在不消耗所有权的前提下，访问 Option 中的值
    let ref_data = data.as_ref();

    // Option<Rc<RefCell< crate::tree::TreeNode >>>
    let tree_node = Some(Rc::new(RefCell::new(TreeNode::new(123))));

    let ref_tree_node = tree_node.as_ref();
    // 为什么需要 as_ref 如果直接使用 unwrap 或者 match ，会导致 p 被 move
    // 希望在不转移所有权的情况下，安全地检查它是否包含一个节点
    {
        let mut ref_mut = ref_tree_node.unwrap().borrow_mut();
        ref_mut.val = 12;
    }
    println!("{:?}", tree_node);
}
