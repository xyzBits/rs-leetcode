use crate::Solution;

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        while n % 3 == 0 {
            n /= 3;
        }

        while n % 5 == 0 {
            n /= 5;
        }

        (n & (n - 1)) == 0
    }
}
