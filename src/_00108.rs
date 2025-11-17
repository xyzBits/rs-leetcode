use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build_tree_from_slice(&nums[..])
    }

    fn build_tree_from_slice(nums_slice: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums_slice.is_empty() {
            return None;
        }

        let mid = nums_slice.len() / 2;

        let mut root = TreeNode {
            val: nums_slice[mid],
            left: None,
            right: None,
        };
        root.left = Self::build_tree_from_slice(&nums_slice[..mid]);
        root.right = Self::build_tree_from_slice(&nums_slice[mid + 1..]);
        Some(Rc::new(RefCell::new(root)))
    }
}

/// Box::new() 创建一个独占的，堆分配的节点
/// Rc::new(T) 创建共享所有权的根节点
/// .clone() 核心API，复制智能指针，增加引用计数，用于递归或者将子节点传递给多个所有者
/// * Deref 访问智能指针内部的数据  
///
#[cfg(test)]
mod tests {
    #[test]
    fn test_sorted_array_to_bst() {}
}
