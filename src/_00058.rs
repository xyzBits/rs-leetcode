use crate::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        if s.trim().is_empty() {
            0
        } else {
            s.split_whitespace().last().unwrap().len() as i32
        }
    }
}
