use crate::Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // s.reverse();
        let n = s.len();
        let (mut left, mut right) = (0, n - 1);
        while left < right {
            s.swap(left, right);

            left += 1;
            right -= 1;
        }
    }
}
