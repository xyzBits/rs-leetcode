use crate::list::ListNode;
use crate::Solution;

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            carry = sum / 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let mut x = Some(2);

        match x.as_mut() {
            None => {}
            Some(v) => *v = 42,
        }

        assert_eq!(x, Some(42));

        let option = x.as_mut();
        *option.unwrap() = 33;

        assert_eq!(x, Some(33));
    }
}
