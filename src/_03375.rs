use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();

        for x in nums {
            if x < k {
                return -1;
            } else if x > k {
                set.insert(x);
            }
        }

        set.len() as i32
    }
}
