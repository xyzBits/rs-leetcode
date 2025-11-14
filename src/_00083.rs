use crate::list::ListNode;
use crate::Solution;

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut cur = head.as_mut()?;
        while let Some(nxt) = cur.next.take() {
            if nxt.val == cur.val {
                cur.next = nxt.next;
            } else {
                cur.next = Some(nxt);
                cur = cur.next.as_mut()?;
            }
        }

        head
    }
}

fn print_vec(data: Box<Vec<String>>) {
    println!("{:?}", data);
}

#[test]
fn test_001() {
    let data = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let box_data = Box::new(data);
    print_vec(box_data);
}
