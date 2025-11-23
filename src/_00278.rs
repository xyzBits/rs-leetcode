use crate::Solution;

impl Solution {
    fn is_bad_version(n: i32) -> bool { true }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut left, mut right) = (1, n);

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::is_bad_version(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
