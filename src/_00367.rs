use crate::Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut left, mut right) = (0, num);

        while left <= right {
            let mid = (right - left) / 2 + left;
            let square = mid as i128 * mid as i128;
            if square < num as i128 {
                left = mid + 1;
            } else if square > num as i128 {
                right = mid - 1;
            } else {
                return true;
            }
        }

        false
    }
}
