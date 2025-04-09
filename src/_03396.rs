use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut i = 0;

        while i < nums.len() {
            if Self::check_unique(&nums, i) {
                return ans;
            }

            i += 3;
            ans += 1;
        }

        ans
    }

    fn check_unique(nums: &Vec<i32>, start: usize) -> bool {
        let mut cnt = HashSet::new();

        for i in start..nums.len() {
            if cnt.contains(&nums[i]) {
                return false;
            }
            cnt.insert(nums[i]);
        }

        true
    }
}
