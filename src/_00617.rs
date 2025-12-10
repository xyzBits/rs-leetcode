use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root1.is_none() {
            return root2;
        }

        if root2.is_none() {
            return root1;
        }

        // let rf_root1 = root1.as_ref().unwrap().borrow();
        let rf_root1 = root1.as_ref().unwrap().borrow();
        let rf_root2 = root2.as_ref().unwrap().borrow();

        let mut merged = TreeNode::new(rf_root1.val + rf_root2.val);
        merged.left = Self::merge_trees(rf_root1.left.clone(), rf_root2.left.clone());
        merged.right = Self::merge_trees(rf_root1.right.clone(), rf_root2.right.clone());

        Some(Rc::new(RefCell::new(merged)))
    }
}

#[test]
fn test_001() {
    let cell = RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    });

    // borrow 获取不可变引用
    let node_ref = cell.borrow();

    // 访问数据，像使用 &TreeNode 一样
    println!("TreeNode.val = {}", node_ref.val);
}

#[test]
fn test_002() {
    let cell = RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    });

    // borrow_mut 获取可变引用
    let mut node_mut_ref = cell.borrow_mut();

    node_mut_ref.val = 5;

    println!("TreeNode.val = {}", node_mut_ref.val);
}

#[test]
fn test_003() {
    let cell = RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    });

    // into_inner() 消耗 RefCell ，获取 TreeNode的所有权
    let final_node = cell.into_inner();
    println!("TreeNode.val = {}", final_node.val);
}

#[test]
fn test_004() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));

    if root.is_none() {
        println!("None in root");
    }

    // let rc_node = root.as_ref().unwrap();
    let rc = root.unwrap();
    let x = rc.borrow();
}
