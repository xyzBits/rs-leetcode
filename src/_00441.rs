use crate::Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut left, mut right) = (1, n);

        while left < right {
            let mid = (right - left + 1) / 2 + left;
            if mid as i64 * (mid + 1) as i64 <= 2 * n as i64 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}
