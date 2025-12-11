use crate::tree::TreeNode;
use crate::Solution;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut averages = Vec::new();
        if root.is_none() {
            return averages;
        }

        let mut queue = VecDeque::new();
        queue.push_back(root.unwrap());

        while !queue.is_empty() {
            let mut sum = 0.0;
            let size = queue.len();
            for _i in 0..size {
                let node_rc = queue.pop_front().unwrap();
                let node = node_rc.borrow();
                sum += node.val as f64;

                if let Some(left) = &node.left {
                    queue.push_back(left.clone());
                }

                if let Some(right) = &node.right {
                    queue.push_back(right.clone());
                }
            }

            averages.push(sum / size as f64);
        }

        averages
    }
}

#[test]
fn test_001() {
    // 不可变引用，允许多个不可变引用同时存在
    let mut x = 5;
    let r1 = &x;
    let r2 = &x;
    println!("r1: {:?}, r2: {:?}", r1, r2);

    // 可变引用，
    // 必须独占，在r3 的作用域内，不能再使用 r1 r2 或者 x 本身
    let r3 = &mut x;

    *r3 += 5;
    println!("r3: {:?}", r3);
    // println!("{}", r1);
}

#[test]
fn test_002() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // r1 r3 同时指向 num，这是引用规则中是不允许的，但是裸指针中可以存在
    unsafe {
        println!("r1: {:?}, r2: {:?}", *r1, *r2);
        *r2 = 10;
        println!("r2: {:?}", *r2);
        println!("{}", num);
    }
}

#[test]
fn test_003() {
    let mut deque: VecDeque<i32> = VecDeque::new();
    deque.push_back(10);
    deque.push_back(20);

    let first = deque.pop_front();
    let second = deque.pop_front();
    let third = deque.pop_front();
    println!(
        "first: {:?}, second: {:?}, third: {:?}",
        first, second, third
    );

    let initialized_dequeue: VecDeque<i32> = (1..=5).collect::<_>();

    let mut stack = VecDeque::new();
    stack.push_back("A");
    stack.push_back("B");
    stack.push_back("C");

    let top = stack.pop_back();
    let next = stack.pop_back();

    println!("top: {:?}, next: {:?}", top, next);

    let front = stack.front();
    println!("front: {:?}", front);
}
