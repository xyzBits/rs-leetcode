use crate::Solution;

impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let n = s.len();
        let mut cnt = 0;

        for ch in s.chars() {
            if ch == letter {
                cnt += 1;
            }
        }

        100 * cnt / n as i32
    }
}
