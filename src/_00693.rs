use crate::Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut prev = 2;
        while n != 0 {
            let cur = n % 2;
            if cur == prev {
                return false;
            }
            prev = cur;
            n /= 2;
        }

        true
    }
}
