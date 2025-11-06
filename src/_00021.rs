use std::rc::Rc;
use crate::list::ListNode;
use crate::Solution;

impl Solution {
    pub fn merge_two_lists_v2(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None, // 两个链表都为空，返回空
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists_v2(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists_v2(Some(node1), node2.next);
                    Some(node2)
                }
            }
        }
    }

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        while let (Some(node1), Some(node2)) = (&list1, &list2) {
            if node1.val < node2.val {
                cur.next = list1.take();
                cur = cur.next.as_mut()?;
                list1 = cur.next.take();
            } else {
                cur.next = list2.take();
                cur = cur.next.as_mut()?;
                list2 = cur.next.take();
            };
        }

        cur.next = list1.or(list2);
        dummy.next
    }
}


#[test]
fn test_001() {
    let mut rc = Rc::new(Box::new(5));
}