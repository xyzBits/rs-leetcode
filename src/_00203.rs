use crate::list::ListNode;
use crate::Solution;
use std::collections::BinaryHeap;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head?;

        head.next = Solution::remove_elements(head.next, val);

        if head.val == val {
            head.next
        } else {
            Some(head)
        }
    }
}
