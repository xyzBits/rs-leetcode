use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut seen = HashSet::new();

        while n != 1 && !seen.contains(&n) {
            seen.insert(n);
            n = Self::get_next(n);
        }

        n == 1
    }

    fn get_next(mut n: i32) -> i32 {
        let mut total_sum = 0;

        while n > 0 {
            let d = n % 10;
            n = n / 10;
            total_sum += d * d;
        }

        total_sum
    }
}
