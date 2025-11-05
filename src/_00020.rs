use crate::Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let mut pairs = HashMap::new();
        pairs.insert(')', '(');
        pairs.insert(']', '[');
        pairs.insert('}', '{');

        let mut stack = VecDeque::new();

        for i in 0..s.len() {
            let ch = s.chars().nth(i).unwrap();
            if pairs.contains_key(&ch) {
                if stack.is_empty() || stack.front() != pairs.get(&ch) {
                    return false;
                }
                stack.pop_front();
            } else {
                stack.push_front(ch);
            }
        }

        stack.is_empty()
    }
}
