use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let mut cnt = HashMap::new();

        let mut res = 0_i64;
        let mut prefix = 0;
        cnt.insert(0, 1);

        for i in 0..n {
            prefix += if nums[i] % modulo == k { 1 } else { 0 };
            res += cnt.get(&((prefix - k + modulo) % modulo)).unwrap_or(&0);
            *cnt.entry(prefix % modulo).or_insert(0) += 1;
        }

        res
    }
}
