use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let mut map = HashMap::new();
        let mut res = 0_i64;

        for i in 0..nums.len() {
            let key = nums[i] - i as i32;
            let value = *map.entry(key).or_insert(0);

            res += i as i64 - value as i64;

            map.insert(key, value + 1);
        }

        res
    }
}
