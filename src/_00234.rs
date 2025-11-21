use crate::list::ListNode;
use crate::Solution;

impl Solution {
    pub fn is_palindrome_234(head: Option<Box<ListNode>>) -> bool {
        let mut values = vec![];
        let mut current = &head;
        while let Some(node) = current {
            values.push(node.val);
            current = &node.next;
        }

        let len = values.len();
        values
            .iter()
            .take(len / 2)
            .eq(values.iter().rev().take(len / 2))
    }
}
